mod networker;
mod pipe_builder;

use glib::MainLoop;
use gstreamer_rtsp_server::{
    RTSPMediaFactory, RTSPMediaFactoryExt, RTSPMountPointsExt, RTSPServer, RTSPServerExt,
    RTSPServerExtManual,
};
use structopt::StructOpt;

use pipe_builder::{Encoder, Input, Pipe, VideoSize};

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
        required_unless = "pipes-as-json",
        possible_values(&["v4l2", "shmem", "rpi"])
    )]
    input: Option<Input>, // this looks stupid, but some library freaks out without it. we get the desired effect at run anyway
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
    #[structopt(long, help = "Pipelines to run as JSON.")]
    pipes_as_json: Option<String>,
    #[structopt(long, help = "List all input modes and exit.", group = "list")]
    list_in: bool,
    #[structopt(long, help = "List all encoders and exit.", group = "list")]
    list_enc: bool,

    #[structopt(flatten)]
    net_opt: networker::NetOpt,
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
    // set up basic Gst stuff
    gstreamer::init().expect("GStreamer could not init!");
    let loop_ = MainLoop::new(Option::None, false);
    let server = RTSPServer::new();
    server.set_service(&opt.net_opt.get_port().to_string());
    let mounts = server
        .get_mount_points()
        .expect("Failed to get mount points");
    if opt.pipes_as_json.is_some() {
        let config = opt.pipes_as_json.unwrap();
        let pipes: Vec<Pipe> = serde_json::from_str(&config).expect("JSON could not parse!");
        for pipe in pipes.iter() {
            let factory = RTSPMediaFactory::new();
            let pipe_str = pipe_builder::create_pipe(pipe);
            println!("Pipeline constructed: {}", pipe_str);
            factory.set_launch(&pipe_str);
            factory.set_shared(true);
            mounts.add_factory(pipe.url(), &factory);
        }
    } else {
        // try to set up video size
        let size = VideoSize::new(opt.width, opt.height, opt.framerate);
        let device = opt.device.unwrap_or_default();
        let mut input = opt.input.unwrap();
        input = match input {
            Input::Video4Linux(_) => Input::Video4Linux(device),
            Input::SharedMemory(_) => Input::SharedMemory(device),
            _ => input,
        };
        let encoder = opt.encoder;
        let pipe = Pipe::new(input, encoder, size, String::from(opt.net_opt.get_url()));
        let pipe_str = pipe_builder::create_pipe(&pipe);
        println!("Pipeline constructed: {}", pipe_str);
        let factory = RTSPMediaFactory::new();
        factory.set_launch(&pipe_str);
        factory.set_shared(true);
        // set up mounts
        mounts.add_factory(pipe.url(), &factory);
    }
    server.attach(Option::None);
    println!("Starting loop...");
    loop_.run();
}
