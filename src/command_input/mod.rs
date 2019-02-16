use std::sync::mpsc::Receiver;
use super::command::Command;

pub trait CommandInput {
    fn create_receiver(&self) -> Receiver<Command>;
}

pub mod twitch_input;
pub use twitch_input::TwitchInput;
