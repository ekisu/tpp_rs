use serde::Serialize;

use crate::command::Command;
use std::sync::mpsc::{Sender, Receiver};

pub type VoteFunction = Box<Fn(Command) -> () + Send>;
pub type DecisionReceiver = Receiver<Command>;
pub type DecisionSender = Sender<Command>;

pub trait VoteSystemCreator {
    fn create(&self, d: DecisionSender) -> VoteFunction;
}

pub mod anarchy;
pub use anarchy::AnarchyCreator;

pub mod democracy;
pub use democracy::DemocracyCreator;

#[derive(Debug, Serialize, PartialEq, Eq, Clone, Copy, Hash)]
pub enum VoteSystem {
    Anarchy,
    Democracy,
}

impl VoteSystem {
    pub fn creator(&self) -> Box<VoteSystemCreator> {
        match self {
            VoteSystem::Anarchy => Box::new(AnarchyCreator {}),
            VoteSystem::Democracy => Box::new(DemocracyCreator {}),
        }
    }
}
