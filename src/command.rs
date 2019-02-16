#[derive(Debug)]
pub enum Command {
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

impl Command {
    pub fn from_string(s: String) -> Option<Self> {
        use Command::*;

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
