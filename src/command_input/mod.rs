use super::command::Command;
use serde::Serialize;
use std::sync::mpsc::Receiver;

type User = String;
#[derive(Debug, Serialize, Clone)]
pub struct Input(pub Command, pub User);

pub trait CommandInput: Send {
    fn create_receiver(&self) -> Receiver<Input>;
}

pub mod twitch_input;
pub use twitch_input::TwitchInput;
