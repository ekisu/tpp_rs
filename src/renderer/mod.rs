use crate::command::Command;

pub trait Renderer {
    fn new_command(&mut self, command: Command);
}

pub mod console;
pub use console::ConsoleRenderer;

pub mod http;
pub use http::HTTPRenderer;
