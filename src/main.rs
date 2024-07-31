use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader, Luma, Rgba, Rgb};
use anyhow::{Error, Ok};

fn main() -> Result<(), Error>{

    let image = CompVision::new("/Users/mmuhammad/Desktop/projects/comp_vision/images/chicago.JPG")?;

    // let image = image.image.blur(2.0);

    let img = CompVision::box_blur(image.image, 3)?;

    img.save("/Users/mmuhammad/Desktop/projects/comp_vision/images/test_img3.png")?;

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

    fn to_grayscale(img: DynamicImage) -> Result<ImageBuffer<Luma<u8>, Vec<u8>>, Error>{
        let (width, height) = img.dimensions();

        let mut test_img = image::ImageBuffer::new(width, height);

        for (x, y, pixel) in img.pixels(){
            let (red, green, blue, _) = CompVision::split_rbg(pixel);
            let grey = (red*CompVision::RED_GRAY + green*CompVision::GREEN_GRAY + blue*CompVision::BLUE_GRAY) as u8;
            let modified_pixel = image::Luma([grey]);
            test_img.put_pixel(x, y, modified_pixel)
        }

        Ok(test_img)
    }

    fn box_blur(img: DynamicImage, radius: u32) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Error> {
        let (width, height) = img.dimensions();
        let mut test_img = image::ImageBuffer::new(width, height);

        for y in 0 .. height{
            for x in 0 .. width {
                let mut sum_r = 0u16;
                let mut sum_g = 0u16;
                let mut sum_b = 0u16;
                let mut count = 0u16;
                
                for dy in -(radius as isize)..=(radius as isize){
                    for dx in -(radius as isize)..=(radius as isize){
                        let nx = dx + x as isize;
                        let ny = dy + y as isize;

                        // Bounds check
                        if ny >= 0 && ny < height as isize && nx >= 0 && nx < width as isize {
                            let pixel = img.get_pixel(nx as u32, ny as u32);
                            let (red, green, blue, _) = CompVision::split_rbg(pixel);
                            sum_r += red as u16;
                            sum_g += green as u16;
                            sum_b += blue as u16;

                            count += 1;
                        }

                    }
                }


                let red = (sum_r / count) as u8;
                let green = (sum_g / count) as u8;
                let blue = (sum_b / count) as u8;

                let rgb = Rgb([red, green, blue]);

                test_img.put_pixel(x, y, rgb);
            }
        }

        Ok(test_img)
    }

}



// fn sobel_filter(){
//     println!("Running the Sobel Filter !")
// }
