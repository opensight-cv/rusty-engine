use glib::MainLoop;
use gstreamer;
use gstreamer_rtsp_server::{
    RTSPMediaFactory, RTSPMediaFactoryExt, RTSPMountPointsExt, RTSPServer, RTSPServerExt,
    RTSPServerExtManual,
};

mod pipe_builder;

use pipe_builder::{create_pipe, Encoder, Input, VideoSize};

fn main() {
    println!(
        "{}",
        create_pipe(
            Input::Video4Linux("/dev/video0".to_string()),
            Encoder::OpenMAX,
            VideoSize::new(320, 240, 30)
        )
    );
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
