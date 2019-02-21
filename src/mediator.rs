use serde::Serialize;

use crate::command::{Button, Command};
use crate::command_input::{CommandInput, Input};
use crate::vote_counter::VoteCounter;
use crate::vote_system::{
    VoteFunction, VoteSystem, VoteSystemCreator, VoteSystemUpdateReceiver, VoteSystemUpdateSender,
};

use stats::Frequencies;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub enum MediatedDecision {
    Command(Command),
}

pub enum MediatorUpdate {
    VoteSystemChange(VoteSystem),
    VoteSystemChangeSecsRemaining(u64),
    VoteSystemPercentageChange(Option<f64>),
    VoteSystemDemocracyPartialResults(u64, Frequencies<Command>),
    Input(Input),
    Decision(MediatedDecision),
}

pub type MediatorUpdateSender = Sender<MediatorUpdate>;
pub type MediatorUpdateReceiver = Receiver<MediatorUpdate>;

pub struct Mediator {}

impl Mediator {
    fn spawn_input_reader<I>(
        command_input: I,
        vote_lock: Arc<Mutex<VoteFunction>>,
        system_counter: VoteCounter<VoteSystem>,
        tx_update: MediatorUpdateSender,
    ) where
        I: CommandInput + 'static,
    {
        thread::spawn(move || {
            let rx_input = command_input.create_receiver();

            loop {
                let input = rx_input.recv().unwrap();
                tx_update
                    .send(MediatorUpdate::Input(input.clone()))
                    .unwrap();

                let Input(cmd, user) = input;

                if let Command::ChangeVoteSystem(system) = cmd {
                    system_counter.vote(system);
                } else {
                    vote_lock.lock().unwrap().call(cmd);
                }
            }
        });
    }

    fn spawn_vote_system_update_receiver(
        rx_vote_system_update: VoteSystemUpdateReceiver,
        tx_mediator_update: MediatorUpdateSender,
    ) {
        thread::spawn(move || {
            use crate::vote_system::VoteSystemUpdate as VSU;
            use MediatedDecision::*;
            use MediatorUpdate as MU;

            loop {
                match rx_vote_system_update.recv() {
                    Ok(update) => {
                        let med_update = match update {
                            VSU::Decision(cmd) => MU::Decision(Command(cmd)),
                            VSU::DemocracyPartialResults(t, part) => {
                                MU::VoteSystemDemocracyPartialResults(t, part)
                            }
                        };

                        tx_mediator_update.send(med_update).unwrap();
                    }
                    Err(e) => {
                        println!("Mediator::vote_system_update: got {} err", e);
                    }
                }
            }
        });
    }

    fn swap_vote_system(
        current_system: &Arc<Mutex<VoteSystem>>,
        vote_lock: &Arc<Mutex<VoteFunction>>,
        tx_decision: VoteSystemUpdateSender,
        new_vote_system: VoteSystem,
    ) {
        let vote_fn = new_vote_system.creator().create(tx_decision);

        *vote_lock.lock().unwrap() = vote_fn;
        *current_system.lock().unwrap() = new_vote_system;
    }

    fn spawn_vote_system_changer(
        current_system: Arc<Mutex<VoteSystem>>,
        vote_lock: Arc<Mutex<VoteFunction>>,
        tx_decision: VoteSystemUpdateSender,
        system_counter: VoteCounter<VoteSystem>,
        last_vote_system_change: Arc<Mutex<Instant>>,
        tx_mediator_update: MediatorUpdateSender,
    ) {
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(30));

            println!("spawn_vote_system_changer: running...");
            *last_vote_system_change.lock().unwrap() = Instant::now();

            let winner = system_counter.winner();
            system_counter.reset();

            if let Some(system) = winner {
                if *current_system.lock().unwrap() != system {
                    Self::swap_vote_system(&current_system, &vote_lock, tx_decision.clone(), system);

                    tx_mediator_update
                        .send(MediatorUpdate::VoteSystemChange(system))
                        .unwrap();
                }
            }
        });
    }

    fn spawn_vote_system_time_updater(
        last_vote_system_change: Arc<Mutex<Instant>>,
        tx_mediator_update: MediatorUpdateSender,
    ) {
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(1));

            let time_remaining = Duration::from_secs(30) - last_vote_system_change.lock().unwrap().elapsed();

            tx_mediator_update.send(
                MediatorUpdate::VoteSystemChangeSecsRemaining(time_remaining.as_secs())
            ).unwrap();
        });
    }

    pub fn create<I>(command_input: I, system: VoteSystem) -> MediatorUpdateReceiver
    where
        I: CommandInput + 'static,
    {
        let (tx_decision, rx_vote_system_update) = channel();
        let vote_function = system.creator().create(tx_decision.clone());
        let (tx_mediator_update, rx_mediator_update) = channel();

        let vote_lock = Arc::new(Mutex::new(vote_function));
        let last_vote_system_change = Arc::new(Mutex::new(Instant::now()));

        // Send initial VoteSystem update
        tx_mediator_update
            .send(MediatorUpdate::VoteSystemChange(system))
            .unwrap();

        let vote_counter = VoteCounter::new(tx_mediator_update.clone(), VoteSystem::Anarchy);
        let current_system = Arc::new(Mutex::new(system));

        Self::spawn_input_reader(
            command_input,
            vote_lock.clone(),
            vote_counter.clone(),
            tx_mediator_update.clone(),
        );
        Self::spawn_vote_system_update_receiver(rx_vote_system_update, tx_mediator_update.clone());
        Self::spawn_vote_system_changer(
            current_system.clone(),
            vote_lock.clone(),
            tx_decision.clone(),
            vote_counter.clone(),
            last_vote_system_change.clone(),
            tx_mediator_update.clone(),
        );
        Self::spawn_vote_system_time_updater(
            last_vote_system_change.clone(),
            tx_mediator_update.clone(),
        );

        rx_mediator_update
    }
}
