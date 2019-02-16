use super::Renderer;
use crate::command::Command;

pub struct ConsoleRenderer {}

impl ConsoleRenderer {
    pub fn new() -> Self {
        ConsoleRenderer {}
    }
}

impl Renderer for ConsoleRenderer {
    fn new_command(&mut self, command: Command) {
        println!("{} - button: {:?}", command.user, command.button);
    }
}
