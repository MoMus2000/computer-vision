use crate::cv::vision;
use anyhow::Error;

pub fn sobel_edge_filter(image_path: &str) -> Result<(), Error>{
    let cv = vision::CompVision::new(image_path)?;
    let test_img = vision::CompVision::edge_detection_sobel(cv.image).unwrap();
    test_img.save(""); 
    !todo!();
    Ok(())
}