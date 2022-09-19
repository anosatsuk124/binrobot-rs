use super::types::{self, Binrobot};

pub enum CommandAPI {
    Move {
        direction: types::Direction,
        velocity: types::Number,
    },
}

impl Binrobot for CommandAPI {
    fn to_command(&self) -> Result<String, types::CommandTypeParseError> {
        Ok(match self {
            Self::Move {
                direction,
                velocity,
            } => format!(
                "MOVE {} {}",
                direction.to_command()?,
                velocity.to_command()?
            ),
        })
    }
}
