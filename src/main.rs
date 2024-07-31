use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader, Luma, Rgba};
use anyhow::{Error, Ok};

fn main() -> Result<(), Error>{

    let image = CompVision::new("/Users/mmuhammad/Desktop/projects/comp_vision/images/chicago.JPG")?;
    let image = image.convert_to_grayscale()?;
    // image.save("/Users/mmuhammad/Desktop/projects/comp_vision/images/test_img2.png")?;

    Ok(())
}

struct CompVision{
    image: DynamicImage
}

impl CompVision{

    //Standard Grayscale Conversion: This formula is widely used in various image processing 
    //applications and standards, including video encoding and image processing libraries. 
    //It provides a good approximation of perceived brightness, which is why it's commonly 
    //used in practice.

    const RED_GRAY : f32 = 0.299;
    const GREEN_GRAY: f32 = 0.587;
    const BLUE_GRAY : f32 = 0.114;

    fn new(image_path: &str) -> Result<CompVision, Error>{
        let img = ImageReader::open(image_path)?.decode()?;
        Ok(CompVision{image: img})
    }

    fn split_rbg(pixel: Rgba<u8>) -> (f32, f32, f32, f32){
        (pixel.0[0] as f32, pixel.0[1] as f32, pixel.0[2] as f32, pixel.0[3] as f32)
    }

    fn convert_to_grayscale(&self) -> Result<ImageBuffer<Luma<u8>, Vec<u8>>, Error>{
        let (width, height) = self.image.dimensions();

        let mut test_img = image::ImageBuffer::new(width, height);

        for (x, y, pixel) in self.image.pixels(){
            let (red, green, blue, _) = CompVision::split_rbg(pixel);
            let grey = (red*CompVision::RED_GRAY + green*CompVision::GREEN_GRAY + blue*CompVision::BLUE_GRAY) as u8;
            let modified_pixel = image::Luma([grey]);
            test_img.put_pixel(x, y, modified_pixel)
        }

        Ok(test_img)
    }

    fn gaussian_blur(&self){
        let (width, height) = self.image.dimensions();
        for (x, y, pixel) in self.image.pixels(){

        }
    }

}



// fn sobel_filter(){
//     println!("Running the Sobel Filter !")
// }
