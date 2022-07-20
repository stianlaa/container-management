use nokhwa::{Camera, CameraFormat, CaptureAPIBackend, FrameFormat};

fn main() {
    // set up the Camera
    let mut camera = Camera::with_backend(
        0,
        Some(CameraFormat::new_from(640, 480, FrameFormat::MJPEG, 30)),
        CaptureAPIBackend::Video4Linux,
    )
    .unwrap();

    // open stream
    camera.open_stream().unwrap();
    loop {
        let frame = camera.frame().unwrap();
        println!("{}, {}", frame.width(), frame.height());
    }
}
