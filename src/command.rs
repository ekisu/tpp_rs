extern crate serde;
use crate::vote_system::VoteSystem;
use serde::Serialize;

#[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Button {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    Select,
    Start,
    L,
    R,
}

impl Button {
    pub fn from_string(s: String) -> Option<Self> {
        use Button::*;

        match s.as_str() {
            "up" => Some(Up),
            "down" => Some(Down),
            "left" => Some(Left),
            "right" => Some(Right),
            "a" => Some(A),
            "b" => Some(B),
            "select" => Some(Select),
            "start" => Some(Start),
            "l" => Some(L),
            "r" => Some(R),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, PartialEq, Eq, Hash, Clone)]
pub enum Command {
    ChangeVoteSystem(VoteSystem),
    Action(Button),
}

impl Command {
    pub fn from_string(s: String) -> Option<Self> {
        use Command::*;

        match s.as_str() {
            "anarchy" => Some(ChangeVoteSystem(VoteSystem::Anarchy)),
            "democracy" => Some(ChangeVoteSystem(VoteSystem::Democracy)),
            _ => match Button::from_string(s) {
                Some(button) => Some(Action(button)),
                None => None,
            },
        }
    }
}
