use std::process::Command;

pub fn get_video_resolution(input_path: &str) -> Result<String, std::io::Error> {
	let command = Command::new("ffprobe")
		.arg("-v")
		.arg("error")
		.arg("-select_streams")
		.arg("v:0")
		.arg("-show_entries")
		.arg("stream=width,height")
		.arg("-of")
		.arg("csv=s=x:p=0")
		.arg(input_path)
		.output()?;

	if !command.status.success() {
		println!("Failed to get video resolution: {:?}", command);
		println!("Error: {}", String::from_utf8_lossy(&command.stderr));
		return Err(std::io::Error::new(
			std::io::ErrorKind::Other,
			"Failed to get video resolution",
		));
	}

	let output = String::from_utf8_lossy(&command.stdout);
	let dimensions = output.split("x").collect::<Vec<&str>>();
	let width = dimensions[0];
	let height = dimensions[1];
	let resolution = format!("{}:{}", width, height);
	Ok(resolution)
}

pub fn validate_video_resolution(source_resolution: &str, target_resoulution: &str) -> bool {
	let source_dimensions = source_resolution.split(":").collect::<Vec<&str>>();
	let source_width = source_dimensions[0].parse::<i32>().unwrap();
	let source_height = source_dimensions[1].parse::<i32>().unwrap();

	let target_dimensions = target_resoulution.split(":").collect::<Vec<&str>>();
	let target_width = target_dimensions[0].parse::<i32>().unwrap();
	let target_height = target_dimensions[1].parse::<i32>().unwrap();

	if source_width < target_width || source_height < target_height {
		return false
	}

	true
}