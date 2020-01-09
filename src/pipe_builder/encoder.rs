use serde::Deserialize;
use std::{convert, fmt, str::FromStr};

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
    // shut up
    type Err = convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "omx" => Ok(Encoder::OpenMAX),
            "camera" => Ok(Encoder::Camera),
            "x264enc" | _ => Ok(Encoder::Software),
        }
    }
}
