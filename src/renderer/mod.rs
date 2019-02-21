use crate::command::Command;
use crate::command_input::Input;
use crate::vote_system::VoteSystem;
use stats::Frequencies;

pub trait Renderer {
    fn new_input(&mut self, input: Input);
    fn new_command(&mut self, command: Command);
    fn new_vote_system(&mut self, vote_system: VoteSystem);
    fn new_vote_system_percentage(&mut self, pct: Option<f64>);
    fn new_vote_system_democracy_partial_results(&mut self, t: u64, results: Frequencies<Command>);
}

pub mod console;
pub use console::ConsoleRenderer;

pub mod http;
pub use http::HTTPRenderer;
