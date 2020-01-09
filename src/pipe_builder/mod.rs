mod encoder;
mod input;
mod pipe;
pub use self::{encoder::Encoder, input::Input, pipe::Pipe};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct VideoSize {
    width: u32,
    height: u32,
    framerate: u32,
}

impl VideoSize {
    pub fn new(width: u32, height: u32, framerate: u32) -> VideoSize {
        VideoSize {
            width,
            height,
            framerate,
        }
    }
}

pub fn create_pipe(pipe: &Pipe) -> String {
    if pipe.input() == Input::Raspberry && pipe.encoder() != Encoder::Camera {
        println!("using a raspberry pi camera with any encoder besides the one provided by the driver is a Bad Idea");
    }
    let inp_str = match pipe.input() {
        Input::Video4Linux(device) => format!("v4l2src device={}", device),
        Input::Raspberry => String::from("rpicamsrc"),
        Input::SharedMemory(socket) => format!(
            "shmsrc socket-path={} ! capsfilter caps=video/x-raw,format=I420",
            socket
        ),
    };
    let enc_str = match pipe.encoder() {
        Encoder::Software => format!(
            "video/x-raw,width={w},height={h},framerate={f}/1 ! videoconvert ! x264enc tune=zerolatency",
            w = pipe.size().width,
            h = pipe.size().height,
            f = pipe.size().framerate
        ),
        Encoder::Camera => format!(
            "video/x-h264,width={w},height={h},framerate={f}/1",
            w = pipe.size().width,
            h = pipe.size().height,
            f = pipe.size().framerate
        ),
        // Encoder::OpenMAX => format!("video/x-raw,width={w},height={h},framerate={f}/1 ! videoconvert ! video/x-raw,format=I420 ! omxh264enc ! video/x-h264,profile=baseline", w = dim.width, h = dim.height, f = dim.framerate)
    };
    vec![inp_str, enc_str, String::from("rtph264pay name=pay0")].join(" ! ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_size() -> VideoSize {
        VideoSize::new(320, 240, 30)
    }

    #[test]
    fn test_raspberry_pipe() {
        assert_eq!(
            "rpicamsrc ! video/x-h264,width=320,height=240,framerate=30/1 ! rtph264pay name=pay0",
            create_pipe(Input::Raspberry, Encoder::Camera, test_size())
        );
    }

    #[test]
    fn test_v4l2_pipes() {
        assert_eq!(
            "v4l2src device=/dev/video0 ! video/x-raw,width=320,height=240,framerate=30/1 ! videoconvert ! x264enc tune=zerolatency ! rtph264pay name=pay0",
            create_pipe(
                Input::Video4Linux("/dev/video0".to_string()),
                Encoder::Software,
                test_size(),
            )
        );
        assert_eq!(
            "v4l2src device=/dev/video0 ! video/x-raw,width=320,height=240,framerate=30/1 ! videoconvert ! video/x-raw,format=I420 ! omxh264enc ! video/x-h264,profile=baseline ! rtph264pay name=pay0",
            create_pipe(
                Input::Video4Linux("/dev/video0".to_string()),
                Encoder::OpenMAX,
                test_size(),
            )
        );
    }
}
