use super::{Vote, VoteFunction, VoteSystemCreator, VoteSystemUpdate, VoteSystemUpdateSender};
use crate::command::Command;

use stats::Frequencies;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

struct _Democracy {
    stop_flag: Arc<AtomicBool>,
    tx_decision: VoteSystemUpdateSender,
    vote_map: Arc<Mutex<Frequencies<Command>>>,
    last_decision: Arc<Mutex<Instant>>,
    handles: Vec<Option<JoinHandle<()>>>,
}

impl _Democracy {
    fn spawn_vote_counter(
        stop_flag: Arc<AtomicBool>,
        tx_decision: VoteSystemUpdateSender,
        vote_map: Arc<Mutex<Frequencies<Command>>>,
        last_decision: Arc<Mutex<Instant>>,
    ) -> JoinHandle<()> {
        thread::spawn(move || loop {
            thread::park_timeout(Duration::from_secs(30));

            if stop_flag.load(Ordering::SeqCst) {
                break;
            }

            {
                let mut _freq = vote_map.lock().unwrap();

                match _freq.most_frequent().first() {
                    Some(&(command, _votes)) => {
                        tx_decision
                            .send(VoteSystemUpdate::Decision(command.clone()))
                            .unwrap();
                    }
                    None => (),
                }

                *_freq = Frequencies::new();
            }

            *last_decision.lock().unwrap() = Instant::now();
        })
    }

    fn spawn_partial_results_sender(
        stop_flag: Arc<AtomicBool>,
        tx_decision: VoteSystemUpdateSender,
        vote_map: Arc<Mutex<Frequencies<Command>>>,
        last_decision: Arc<Mutex<Instant>>,
    ) -> JoinHandle<()> {
        thread::spawn(move || {
            loop {
                thread::park_timeout(Duration::from_secs(1));

                if stop_flag.load(Ordering::SeqCst) {
                    break;
                }

                let mut _vote = vote_map.lock().unwrap();

                let time_remaining =
                    Duration::from_secs(30)
                    .checked_sub(last_decision.lock().unwrap().elapsed())
                    .unwrap_or(Duration::from_secs(0));

                // ye
                tx_decision
                    .send(VoteSystemUpdate::DemocracyPartialResults(
                        time_remaining.as_secs(),
                        _vote.clone(),
                    ))
                    .unwrap();
            }
        })
    }

    fn new(tx_decision: VoteSystemUpdateSender) -> Self {
        let vote_map = Arc::new(Mutex::new(Frequencies::new()));
        let last_decision = Arc::new(Mutex::new(Instant::now()));
        let stop_flag = Arc::new(AtomicBool::new(false));
        let mut handles = Vec::new();

        handles.push(Some(Self::spawn_vote_counter(
            stop_flag.clone(),
            tx_decision.clone(),
            vote_map.clone(),
            last_decision.clone(),
        )));
        handles.push(Some(Self::spawn_partial_results_sender(
            stop_flag.clone(),
            tx_decision.clone(),
            vote_map.clone(),
            last_decision.clone(),
        )));

        Self {
            stop_flag,
            tx_decision,
            vote_map,
            last_decision,
            handles,
        }
    }
}

impl Vote for _Democracy {
    fn call(&self, c: Command) {
        let mut _vote = self.vote_map.lock().unwrap();
        _vote.add(c);

        // Sometimes elapsed() can be >30s.
        let time_remaining = Duration::from_secs(30)
            .checked_sub(self.last_decision.lock().unwrap().elapsed())
            .unwrap_or(Duration::from_secs(0));

        // ye
        self.tx_decision
            .send(VoteSystemUpdate::DemocracyPartialResults(
                time_remaining.as_secs(),
                _vote.clone(),
            ))
            .unwrap();
    }
}

impl Drop for _Democracy {
    fn drop(&mut self) {
        self.stop_flag.store(true, Ordering::SeqCst);

        println!("Dropping _Democracy");
        for handle in self.handles.iter() {
            if let Some(h) = handle {
                h.thread().unpark();
            }
        }

        for handle in &mut self.handles {
            if let Some(h) = handle.take() {
                h.join().unwrap();
            }
        }

        println!("Dropped _Democracy");
    }
}

pub struct DemocracyCreator {}

impl VoteSystemCreator for DemocracyCreator {
    fn create(&self, tx_decision: VoteSystemUpdateSender) -> VoteFunction {
        Box::new(_Democracy::new(tx_decision))
    }
}
