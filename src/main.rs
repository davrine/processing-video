use opencv::{core, highgui, imgproc, prelude::*, videoio, Result};
use std::env;
use std::fs;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let sleep_duration = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);

    eprintln!("{}", sleep_duration);

    let window = "video capture";
    highgui::named_window(window, 1)?;

    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }

    let mut count = 0;

    let (tx, rx) = mpsc::channel();

    // Spawn a new thread
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(sleep_duration));
        tx.send("10 seconds passed")
            .expect("Failed to send message");
    });

    loop {
        let mut frame = Mat::default();
        cam.read(&mut frame)?;
        println!("{}", frame.size()?.width > 0);

        if frame.size()?.width > 0 {
            highgui::imshow(window, &frame)?;
            // Resize the frame
            // Calculate the new dimensions while maintaining the aspect ratio

            let mut gray_frame = Mat::default();
            imgproc::cvt_color(&frame, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0)?;

            let original_size = gray_frame.size()?;
            let aspect_ratio = original_size.width as f64 / original_size.height as f64;
            let new_width: i32;
            let new_height: i32;

            // Assuming you want to set the width to 100 and adjust the height accordingly
            new_height = 100;
            new_width = (100 as f64 * aspect_ratio) as i32;

            let mut resized_frame = Mat::default();
            let new_size = core::Size::new(new_width, new_height);
            imgproc::resize(
                &gray_frame,
                &mut resized_frame,
                new_size,
                0.0,
                0.0,
                imgproc::INTER_LINEAR,
            )?;

            let cropped_width = 100;
            let x_offset = (new_width - cropped_width) / 2; // Center the crop
            let y_offset = 0; // Keep the y-offset as 0 if you want to crop width only

            let roi = core::Rect::new(x_offset, y_offset, cropped_width, new_height);
            let cropped_frame = Mat::roi(&resized_frame, roi)?;

            if !fs::metadata("./output").is_ok() {
                eprintln!("Error with output file")
            }

            // Save each frame as an image
            let filename = format!("./output/griddy_{}_kawohfnaliw.png", count);
            if let Err(e) = opencv::imgcodecs::imwrite(
                &filename,
                &cropped_frame,
                &opencv::core::Vector::<i32>::new(),
            ) {
                eprintln!("Failed to write frame: {}", e);
            }

            if frame.empty() {
                println!("with frame")
            }
        }
        count += 1;
        if let Ok(message) = rx.try_recv() {
            println!("Received message: {}", message);
            break;
        }

        if highgui::wait_key(10)? > 0 {
            break;
        }
    }
    Ok(())
}
