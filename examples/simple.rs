use rpi_video_rs::recorder::Recorder;

fn main() {
    println!("\nStart to record a new H264 video\n");

    let mut recorder = Recorder::new(None);

    match recorder.run() {
        Ok(res) =>
            println!("A new H264 video is generated to `{}`\n", res.output_file_path),
        Err(error) =>
            println!("An error occurred - `{}`\n", error.message),
    }

    println!("\nFinish recording\n");
}
