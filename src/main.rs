use glib::MainLoop;
use gstreamer;
use gstreamer_rtsp_server::{
    RTSPMediaFactory, RTSPMediaFactoryExt, RTSPMountPointsExt, RTSPServer, RTSPServerExt,
    RTSPServerExtManual,
};
use structopt::StructOpt;

use pipe_builder::{Encoder, Input, VideoSize};

mod pipe_builder;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rusty-engine",
    about = "rusty-engine RTSP server",
    rename_all = "kebab"
)]
struct Opt {
    #[structopt(
        short,
        long = "fps",
        help = "Video framerate (FPS)",
        default_value = "30"
    )]
    framerate: u32,
    #[structopt(short, long, help = "Video width", default_value = "320")]
    width: u32,
    #[structopt(short, long, help = "Video height", default_value = "240")]
    height: u32,
    #[structopt(
        short,
        long,
        help = "Input mode to use",
        required_unless = "list",
        possible_values(&["v4l2", "shmem", "rpi"])
    )]
    input: Input,
    #[structopt(
        short,
        long,
        help = "Input device to use",
        default_value_if("input", Some("v4l2"), "/dev/video0"),
        default_value_if("input", Some("shmem"), "/tmp/engineering")
    )]
    device: Option<String>,
    #[structopt(
        short,
        long,
        help = "Encoder to use. omx for Pi + USB camera, camera if you know it's available/Pi Camera Module, software any other time.",
        possible_values(&["x264enc", "omx", "camera"]),
        default_value = "x264enc",
        default_value_if("input", Some("rpi"), "camera")
    )]
    encoder: Encoder,
    #[structopt(long, help = "List all input modes and exit.", group = "list")]
    list_in: bool,
    #[structopt(long, help = "List all encoders and exit.", group = "list")]
    list_enc: bool,
}

fn main() {
    let opt = Opt::from_args();
    if opt.list_in {
        println!("Available input modes:");
        for inp in Input::all() {
            println!("{}", inp);
        }
        return;
    }
    if opt.list_enc {
        println!("Available encoder modes:");
        for enc in Encoder::all() {
            println!("{}", enc);
        }
        return;
    }
    // try to set up video size
    let size = VideoSize::new(opt.width, opt.height, opt.framerate);
    let device = opt.device.unwrap_or("".to_string());
    let mut input = opt.input;
    input = match input {
        Input::Video4Linux(_) => Input::Video4Linux(device),
        Input::SharedMemory(_) => Input::SharedMemory(device),
        _ => input,
    };
    let encoder = opt.encoder;
    let pipe = pipe_builder::create_pipe(input, encoder, size);
    println!("Pipeline constructed: {}", pipe);
    gstreamer::init().expect("GStreamer could not init!");
    let loop_ = MainLoop::new(Option::None, false);
    let server = RTSPServer::new();
    server.set_service("1181");
    let factory = RTSPMediaFactory::new();
    factory.set_launch(&pipe);
    factory.set_shared(true);
    let mounts = server
        .get_mount_points()
        .expect("Failed to get mount points");
    // set up mounts
    mounts.add_factory("/stream", &factory);
    server.attach(Option::None);
    println!("Starting loop...");
    loop_.run();
}
