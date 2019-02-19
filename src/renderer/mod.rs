use crate::command::Command;
use crate::vote_system::VoteSystem;
use crate::command_input::Input;

pub trait Renderer {
    fn new_input(&mut self, input: Input);
    fn new_command(&mut self, command: Command);
    fn new_vote_system(&mut self, vote_system: VoteSystem);
    fn new_vote_system_percentage(&mut self, pct: f64);
}

pub mod console;
pub use console::ConsoleRenderer;

pub mod http;
pub use http::HTTPRenderer;
