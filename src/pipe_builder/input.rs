use serde::Deserialize;
use std::{fmt, str::FromStr};

#[derive(Debug, PartialEq, Deserialize)]
pub enum Input {
    Video4Linux(String),
    SharedMemory(String),
    Raspberry,
}

impl Input {
    pub fn all() -> Vec<Input> {
        vec![
            Input::Video4Linux(String::new()),
            Input::SharedMemory(String::new()),
            Input::Raspberry,
        ]
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Input::Video4Linux(_) => write!(f, "v4l2"),
            Input::SharedMemory(_) => write!(f, "shmem"),
            Input::Raspberry => write!(f, "rpi"),
        }
    }
}

impl FromStr for Input {
    // TODO: Meaningful error type
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rpi" => Ok(Self::Raspberry),
            "shmem" => Ok(Self::SharedMemory(String::new())),
            "v4l2" => Ok(Self::Video4Linux(String::new())),
            _ => Err(format!("{} is not a valid input type!", s)),
        }
    }
}
