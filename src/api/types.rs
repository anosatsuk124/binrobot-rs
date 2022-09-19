pub trait Binrobot {
    fn to_command(&self) -> Result<String, CommandTypeParseError>;
}

pub enum Bool {
    True,
    False,
}

pub struct Number(pub f64);

#[derive(Debug)]
pub struct CommandTypeParseError;

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Binrobot for Bool {
    fn to_command(&self) -> Result<String, CommandTypeParseError> {
        Ok(match self {
            Bool::True => "TRUE",
            Bool::False => "FALSE",
        }
        .to_string())
    }
}

impl Binrobot for Number {
    fn to_command(&self) -> Result<String, CommandTypeParseError> {
        Ok(self.0.to_string())
    }
}

impl Binrobot for Direction {
    fn to_command(&self) -> Result<String, CommandTypeParseError> {
        Ok(match self {
            Self::Left => "LEFT",
            Self::Right => "RIGHT",
            Self::Up => "UP",
            Self::Down => "DOWN",
        }
        .to_string())
    }
}
