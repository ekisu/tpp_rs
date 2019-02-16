use crate::command::Command;

pub trait CommandOutput {
    fn emit(&mut self, c: Command);
}

pub mod keyboard;
pub use keyboard::KeyboardOutput;

pub mod virtual_joystick;
