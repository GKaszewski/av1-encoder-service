use std::process::Command;

pub fn encode_video(input_path: &str, output_path: &str) -> Result<(), std::io::Error> {
    let command = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_path)
        .arg("-c:v")
        .arg("libaom-av1")
        .arg("-strict")
        .arg("-2")
        .arg("-threads")
        .arg("4")
        .arg("-row-mt")
        .arg("1")
        .arg("-cpu-used")
        .arg("4")
        .arg("-crf")
        .arg("22")
        .arg(output_path)
        .output()?;

    if !command.status.success() {
        println!("Failed to encode video: {:?}", command);
        println!("Error: {}", String::from_utf8_lossy(&command.stderr));
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to encode video",
        ));
    }

    println!("Video encoded successfully!");
    Ok(())
}

pub async fn encode_to_resolution(input_path: &str, output_path: &str, resolution: &str) -> Result<(), std::io::Error> {
    let command = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_path)
        .arg("-c:v")
        .arg("libaom-av1")
        .arg("-strict")
        .arg("-2")
        .arg("-threads")
        .arg("4")
        .arg("-row-mt")
        .arg("1")
        .arg("-cpu-used")
        .arg("4")
        .arg("-crf")
        .arg("22")
        .arg("-vf")
        .arg(format!("scale={}", resolution))
        .arg(output_path)
        .output()?;

    Ok(())
}