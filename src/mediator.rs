use serde::Serialize;

use crate::command::{Button, Command};
use crate::command_input::{CommandInput, Input};
use crate::vote_system::{DecisionSender, DecisionReceiver, VoteFunction, VoteSystem, VoteSystemCreator};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub enum MediatedDecision {
    Command(Command),
}

pub enum MediatorUpdate {
    VoteSystemChange(VoteSystem),
    VoteSystemPercentageChange(f64),
    Decision(MediatedDecision)
}

pub type MediatorUpdateSender = Sender<MediatorUpdate>;
pub type MediatorUpdateReceiver = Receiver<MediatorUpdate>;

pub struct Mediator {}

impl Mediator {
    fn spawn_input_reader<I>(
        command_input: I,
        vote_lock: Arc<Mutex<VoteFunction>>,
        anarchy_counter: Arc<AtomicUsize>,
        democracy_counter: Arc<AtomicUsize>,
        tx_update: MediatorUpdateSender,
    ) where
        I: CommandInput + 'static,
    {
        thread::spawn(move || {
            let rx_input = command_input.create_receiver();

            loop {
                let Input(cmd, user) = rx_input.recv().unwrap();
                if let Command::ChangeVoteSystem(system) = cmd {
                    match system {
                        VoteSystem::Anarchy => &anarchy_counter,
                        VoteSystem::Democracy => &democracy_counter,
                    }
                    .fetch_add(1, Ordering::SeqCst);

                    let anarchy_votes = anarchy_counter.load(Ordering::SeqCst);
                    let democracy_votes = democracy_counter.load(Ordering::SeqCst);

                    // Maybe we shouldn't send updates here?
                    tx_update
                        .send(MediatorUpdate::VoteSystemPercentageChange(
                            democracy_votes as f64 / (anarchy_votes + democracy_votes) as f64,
                        ))
                        .unwrap();
                } else {
                    let mut _guard = vote_lock.lock().unwrap();

                    (_guard)(cmd);
                }
            }
        });
    }

    fn spawn_decision_receiver(
        rx_decision: DecisionReceiver,
        tx_mediator_update: MediatorUpdateSender,
    ) {
        thread::spawn(move || {
            use MediatedDecision::*;

            loop {
                match rx_decision.recv() {
                    Ok(vote) => {
                        tx_mediator_update
                            .send(MediatorUpdate::Decision(Command(vote)))
                            .unwrap();
                    },
                    Err(e) => {
                        println!("Mediator::spawn_decision_receiver: got {} err", e);
                    }
                }
            }
        });
    }

    fn swap_vote_system(
        vote_lock: &Arc<Mutex<VoteFunction>>,
        tx_decision: DecisionSender,
        new_vote_system: VoteSystem,
    ) {
        let vote_fn = new_vote_system.creator().create(tx_decision);

        *vote_lock.lock().unwrap() = vote_fn;
    }

    fn spawn_vote_system_changer(
        current_system: Arc<Mutex<VoteSystem>>,
        vote_lock: Arc<Mutex<VoteFunction>>,
        tx_decision: DecisionSender,
        anarchy_counter: Arc<AtomicUsize>,
        democracy_counter: Arc<AtomicUsize>,
        tx_mediator_update: MediatorUpdateSender
    ) {
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(30));

            println!("spawn_vote_system_changer: running...");

            let anarchy_votes = anarchy_counter.swap(0, Ordering::SeqCst);
            let democracy_votes = democracy_counter.swap(0, Ordering::SeqCst);

            if anarchy_votes > democracy_votes
                && *current_system.lock().unwrap() == VoteSystem::Democracy {
                Self::swap_vote_system(&vote_lock, tx_decision.clone(), VoteSystem::Anarchy);

                tx_mediator_update.send(MediatorUpdate::VoteSystemChange(
                    VoteSystem::Anarchy
                )).unwrap();
            } else if democracy_votes > anarchy_votes
                && *current_system.lock().unwrap() == VoteSystem::Anarchy {
                Self::swap_vote_system(&vote_lock, tx_decision.clone(), VoteSystem::Democracy);

                tx_mediator_update.send(MediatorUpdate::VoteSystemChange(
                    VoteSystem::Democracy
                )).unwrap();
            }
        });
    }

    pub fn create<I>(command_input: I, system: VoteSystem) -> MediatorUpdateReceiver
    where
        I: CommandInput + 'static,
    {
        let (tx_decision, rx_decision) = channel();
        let vote_function = system.creator().create(tx_decision.clone());
        let (tx_mediator_update, rx_mediator_update) = channel();

        let vote_lock = Arc::new(Mutex::new(vote_function));

        // Send initial VoteSystem update
        tx_mediator_update.send(MediatorUpdate::VoteSystemChange(
            system
        )).unwrap();

        let anarchy_vote_counter = Arc::new(AtomicUsize::new(0));
        let democracy_vote_counter = Arc::new(AtomicUsize::new(0));

        let current_system = Arc::new(Mutex::new(system));

        Self::spawn_input_reader(
            command_input,
            vote_lock.clone(),
            anarchy_vote_counter.clone(),
            democracy_vote_counter.clone(),
            tx_mediator_update.clone(),
        );
        Self::spawn_decision_receiver(rx_decision, tx_mediator_update.clone());
        Self::spawn_vote_system_changer(
            current_system.clone(),
            vote_lock.clone(),
            tx_decision.clone(),
            anarchy_vote_counter.clone(),
            democracy_vote_counter.clone(),
            tx_mediator_update.clone()
        );

        rx_mediator_update
    }
}
