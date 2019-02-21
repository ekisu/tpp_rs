use super::Renderer;
use crate::command::Command;
use crate::command_input::Input;
use crate::vote_system::VoteSystem;

use stats::Frequencies;

pub struct ConsoleRenderer {}

impl ConsoleRenderer {
    pub fn new() -> Self {
        ConsoleRenderer {}
    }
}

impl Renderer for ConsoleRenderer {
    fn new_input(&mut self, input: Input) {
        println!("{:?}", input);
    }

    fn new_command(&mut self, command: Command) {
        println!("{:?}", command);
    }

    fn new_vote_system(&mut self, vote_system: VoteSystem) {}
    fn new_vote_system_percentage(&mut self, pct: Option<f64>) {}
    fn new_vote_system_democracy_partial_results(&mut self, t: u64, results: Frequencies<Command>) {}
}
