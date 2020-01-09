use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pipe {
    input: super::Input,
    encoder: super::Encoder,
    size: super::VideoSize,
}

impl Pipe {
    pub fn input(&self) -> super::Input {
        self.input
    }

    pub fn encoder(&self) -> super::Encoder {
        self.encoder
    }

    pub fn size(&self) -> super::VideoSize {
        self.size
    }
}
