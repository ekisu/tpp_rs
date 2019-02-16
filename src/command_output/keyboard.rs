use super::CommandOutput;
use crate::command::Button;

use enigo::{Enigo, Key, KeyboardControllable};

pub struct KeyboardOutput {
    enigo: Enigo
}

impl KeyboardOutput {
    pub fn new() -> Self {
        KeyboardOutput {
            enigo: Enigo::new()
        }
    }
}

trait AsKey {
    fn as_key(&self) -> Key;
}

impl AsKey for Button {
    fn as_key(&self) -> Key {
        use Button::*;
        
        match *self {
            Up => Key::UpArrow,
            Down => Key::DownArrow,
            Left => Key::LeftArrow,
            Right => Key::RightArrow,
            A => Key::Layout('a'),
            B => Key::Layout('b'),
            Select => Key::Space,
            Start => Key::Return,
            L => Key::Layout('l'),
            R => Key::Layout('r')
        }
    }
}

impl CommandOutput for KeyboardOutput {
    fn emit(&mut self, c: Button) {
        self.enigo.key_click(c.as_key());
    }
}
