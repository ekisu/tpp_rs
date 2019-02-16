extern crate serde;
use serde::Serialize;

#[derive(Debug, Serialize, Copy, Clone)]
pub enum Button {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    Select,
    Start,
    L,
    R
}

impl Button {
    pub fn from_string(s: String) -> Option<Self> {
        use Button::*;

        match s.as_str() {
            "up" => Some(Up),
            "down" => Some(Down),
            "left" => Some(Left),
            "right" => Some(Right),
            "a" => Some(A),
            "b" => Some(B),
            "select" => Some(Select),
            "start" => Some(Start),
            "l" => Some(L),
            "r" => Some(R),
            _ => None
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Command {
    pub user: String,
    pub button: Button
}
