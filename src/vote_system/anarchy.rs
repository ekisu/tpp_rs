use super::{DecisionSender, VoteFunction, VoteSystemCreator};
use std::sync::mpsc::channel;

pub struct AnarchyCreator {}

impl VoteSystemCreator for AnarchyCreator {
    fn create(&self, tx_decision: DecisionSender) -> VoteFunction {
        let vote_function = Box::new(move |c| {
            tx_decision.send(c).unwrap();
        });

        vote_function
    }
}
