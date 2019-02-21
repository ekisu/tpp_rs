use super::{Vote, VoteFunction, VoteSystemCreator, VoteSystemUpdate, VoteSystemUpdateSender};
use crate::command::Command;

struct _Anarchy {
    tx_decision: VoteSystemUpdateSender,
}

impl Vote for _Anarchy {
    fn call(&self, c: Command) {
        self.tx_decision
            .send(VoteSystemUpdate::Decision(c))
            .unwrap();
    }
}

pub struct AnarchyCreator {}

impl VoteSystemCreator for AnarchyCreator {
    fn create(&self, tx_decision: VoteSystemUpdateSender) -> VoteFunction {
        Box::new(_Anarchy { tx_decision })
    }
}
