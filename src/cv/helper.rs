use std::process::{Command, exit};
use anyhow::Error;
use std::fs;

pub fn to_pictures(video_path: &str) -> Result<(), Error>{
    // ffmpeg -i output.mp4 -vf "fps=10,scale=426:240" output_frame_%04d.png
    let output = Command::new("ffmpeg")
        .args(&[
            "-i", video_path,
            "-vf", "fps=10,scale=426:240",
            "./video/output_frame_%04d.png",
        ])
        .output()
        .expect("Failed to execute command");

    // Check if the command executed successfully
    if !output.status.success() {
        eprintln!("Error: Command failed with status {}", output.status);
        eprintln!("Standard Error: {}", String::from_utf8_lossy(&output.stderr));
        exit(1);
    }

    Ok(())
}

pub fn to_video() -> Result<(), Error>{
    let output = Command::new("ffmpeg")
        .args(&[
            "-framerate", "10",
            "-i", "./video/output_frame_%04d.png",
            "-c:v", "libx264",
            "-r", "30",
            "-pix_fmt", "yuv420p",
            "./video/output.mp4",
        ])
        .output()
        .expect("Failed to execute command");

    // Check if the command executed successfully
    if !output.status.success() {
        eprintln!("Error: Command failed with status {}", output.status);
        eprintln!("Standard Error: {}", String::from_utf8_lossy(&output.stderr));
        exit(1);
    }

    Ok(())
}

pub fn get_all_files_in_folder(folder_path: &str) -> Result<Vec<String>, Error>{
    let mut files = Vec::new();
    
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    files.push(file_name_str.to_string());
                }
            }
        }
    }
    
    Ok(files)
}