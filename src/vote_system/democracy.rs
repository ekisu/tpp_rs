use super::{DecisionSender, VoteFunction, VoteSystemCreator};
use crate::command::Command;

use std::sync::mpsc::channel;
use stats::Frequencies;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct DemocracyCreator {}

impl DemocracyCreator {
    fn spawn_vote_counter(
        &self,
        tx_decision: DecisionSender,
        vote_map: Arc<Mutex<Frequencies<Command>>>
    ) {
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(30));

                {
                    let mut _freq = vote_map.lock().unwrap();

                    match _freq.most_frequent().first() {
                        Some(&(command, _votes)) => {
                            tx_decision.send(command.clone()).unwrap();
                        },
                        None => ()
                    }

                    *_freq = Frequencies::new();
                }
            }
        });
    }
}

impl VoteSystemCreator for DemocracyCreator {
    fn create(&self, tx_decision: DecisionSender) -> VoteFunction {
        let vote_map = Arc::new(Mutex::new(Frequencies::new()));

        self.spawn_vote_counter(tx_decision, vote_map.clone());

        let vote_function = Box::new(move |c| {
            vote_map.lock().unwrap().add(c);
        });

        vote_function
    }
}
