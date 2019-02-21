use serde::Serialize;

use crate::command::Command;
use stats::Frequencies;
use std::sync::mpsc::{Receiver, Sender};

pub trait Vote: Send {
    fn call(&self, c: Command);
}

pub type VoteFunction = Box<Vote>;

pub enum VoteSystemUpdate {
    Decision(Command),
    DemocracyPartialResults(u64, Frequencies<Command>),
}

pub type VoteSystemUpdateReceiver = Receiver<VoteSystemUpdate>;
pub type VoteSystemUpdateSender = Sender<VoteSystemUpdate>;

pub trait VoteSystemCreator {
    fn create(&self, d: VoteSystemUpdateSender) -> VoteFunction;
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
