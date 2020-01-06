use glib::MainLoop;
use gstreamer;
use gstreamer_rtsp_server::{
    RTSPMediaFactory, RTSPMediaFactoryExt, RTSPMountPointsExt, RTSPServer, RTSPServerExt,
    RTSPServerExtManual,
};

mod pipe_builder;

use pipe_builder::{create_pipe, Encoder, Input, VideoSize};
#[macro_use]
extern crate clap;
use clap::{App, Arg, ArgGroup};

fn is_number(v: String) -> Result<(), String> {
    match v.trim().parse::<u32>() {
        Ok(_) => Ok(()),
        _ => Err(String::from(format!(
            "expected a positive number, got {}",
            v
        ))),
    }
}

fn main() {
    let _matches = App::new("rusty-engine")
        .version(crate_version!())
        .author(crate_authors!())
        .about("rusty-engine RTSP server")
        .arg(
            Arg::with_name("framerate")
                .short("f")
                .long("fps")
                .value_name("FRAMERATE")
                .help("Video framerate.")
                .default_value("30")
                .validator(is_number),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .value_name("WIDTH")
                .help("Video width.")
                .default_value("320")
                .validator(is_number),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .value_name("HEIGHT")
                .help("Video height.")
                .default_value("240")
                .validator(is_number),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("--input")
                .value_name("INPUT")
                .help("Input mode to use.")
                .takes_value(true)
                // TODO: Make this tie to Input enum
                .possible_values(&["v4l2", "shmem", "rpi"])
                .required_unless("list"),
        )
        .arg(
            Arg::with_name("device")
                .short("d")
                .long("device")
                .help("Input device or socket.")
                .default_value_if("input", Some("v4l2"), "/dev/video0")
                .default_value_if("input", Some("shmem"), "/tmp/engineering"),
        )
        .arg(
            Arg::with_name("encoder")
                .short("e")
                .long("encoder")
                .help("Encoder to use. omx for Pi + USB camera, camera if you know it's available/Pi Camera Module, software any other time.")
                .possible_values(&["x264enc", "omx", "camera"])
                .default_value("x264enc")
                .default_value_if("input", Some("rpi"), "camera"),
        )
        .arg(
            Arg::with_name("list-in")
                .long("list-inputs")
                .help("List all input modes and exit."),
        )
        .arg(
            Arg::with_name("list-enc")
                .long("list-encoders")
                .help("List all encoders and exit."),
        )
        .group(ArgGroup::with_name("list").args(&["list-enc", "list-in"]))
        .get_matches();
    if _matches.is_present("list") {
        if _matches.is_present("list-in") {
            println!("Available input modes:");
            for inp in Input::all() {
                println!("{}", inp);
            }
        }
        if _matches.is_present("list-enc") {
            println!("Available encoder modes:");
            for enc in Encoder::all() {
                println!("{}", enc);
            }
        }
        return;
    }
    // try to set up video size
    gstreamer::init().expect("GStreamer could not init!");
    let loop_ = MainLoop::new(Option::None, false);
    let server = RTSPServer::new();
    server.set_service("1181");
    let factory = RTSPMediaFactory::new();
    factory.set_launch("shmsrc socket-path=/tmp/blah ! video/x-raw,format=I420,width=320,height=240,framerate=30/1 ! x264enc ! rtph264pay name=pay0");
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
