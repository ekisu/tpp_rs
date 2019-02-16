use crate::command::Button;

pub trait CommandOutput {
    fn emit(&mut self, c: Button);
}

pub mod keyboard;
pub use keyboard::KeyboardOutput;

pub mod virtual_joystick;
