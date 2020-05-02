use serde::Deserialize;
use std::{fmt, str::FromStr};

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub enum Encoder {
    Software,
    OpenMAX,
    Camera,
}

impl Encoder {
    pub fn all() -> Vec<Encoder> {
        vec![Encoder::Software, Encoder::OpenMAX, Encoder::Camera]
    }
}

impl fmt::Display for Encoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Encoder::Software => write!(f, "x264enc"),
            Encoder::OpenMAX => write!(f, "omx"),
            Encoder::Camera => write!(f, "camera"),
        }
    }
}

impl FromStr for Encoder {
    // TODO: Meaningful error type
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "omx" => Ok(Encoder::OpenMAX),
            "camera" => Ok(Encoder::Camera),
            "x264enc" => Ok(Encoder::Software),
            _ => Err(format!("{} is not an encoder type", s)),
        }
    }
}
