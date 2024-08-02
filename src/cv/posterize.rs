
use crate::cv::{vision, helper};
use anyhow::Error;
use std::fs;
use indicatif::{ProgressBar, ProgressStyle};

pub fn posterize_filter(video_path: &str) -> Result<(), Error>{
    helper::to_pictures(video_path)?;

    println!("Fetching pics from {} ..", video_path);

    let pictures = helper::get_all_files_in_folder("./video")?;

    let total = pictures.len() as u64;
    let pb = ProgressBar::new(total);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg} {bar:40.cyan/blue} {percent:>3}%")?
        .progress_chars("█▌▐"));


    for pic in &pictures{
        if pic.contains("png"){
            let image_path = format!("./video/{}", pic);
            let cv = vision::CompVision::new(&image_path)?;
            let test_img = vision::CompVision::posterize(cv.image, 10).unwrap();
            test_img.save(image_path)?; 
            pb.inc(1);
        }
    }

    println!("Building video ..");

    helper::to_video()?;

    for pic in pictures{
        if pic.contains("png"){
            let image_path = format!("./video/{}", pic);
            fs::remove_file(image_path)?;
        }
    }

    Ok(())
}
