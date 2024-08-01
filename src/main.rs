use std::f32::consts::PI;
use std::fs;

use crate::gui::main_screen;

use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader, Luma, Rgba, Rgb};
use anyhow::{Error, Ok};
use actix_web::{web, App, HttpServer};


mod gui;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(main_screen::index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn process_image(image_path: &str) -> Result<(), Error> {
    if image_path.contains("png"){
        println!("Processing image {}", image_path);
        let path = format!("/Users/mmuhammad/Desktop/projects/comp_vision/images/cycling/{}", image_path);
        let image = CompVision::new(&path)?;
        let res = CompVision::edge_detection_sobel(image.image)?;
        res.save(&path)?;
        println!("Done processing image {}", image_path);
    }
    Ok(())
}

fn get_all_files_in_folder(folder_path: &str) -> Result<Vec<String>, Error>{
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

    fn split_rbga(pixel: Rgba<u8>) -> (f32, f32, f32, f32){
        (pixel.0[0] as f32, pixel.0[1] as f32, pixel.0[2] as f32, pixel.0[3] as f32)
    }

    fn split_rbg(pixel: Rgb<u8>) -> (f32, f32, f32){
        (pixel.0[0] as f32, pixel.0[1] as f32, pixel.0[2] as f32)
    }

    fn convert_to_grayscale(&self) -> Result<ImageBuffer<Luma<u8>, Vec<u8>>, Error>{
        let (width, height) = self.image.dimensions();

        let mut test_img = image::ImageBuffer::new(width, height);

        for (x, y, pixel) in self.image.pixels(){
            let (red, green, blue, _) = CompVision::split_rbga(pixel);
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
            let (red, green, blue, _) = CompVision::split_rbga(pixel);
            let grey = (red*CompVision::RED_GRAY + green*CompVision::GREEN_GRAY + blue*CompVision::BLUE_GRAY) as u8;
            let modified_pixel = image::Luma([grey]);
            test_img.put_pixel(x, y, modified_pixel)
        }

        Ok(test_img)
    }

    fn create_gaussian_kernel_2d(radius: usize, sigma: f32) -> Vec<f32> {
        let size = 2 * radius + 1;
        let mut kernel = vec![0.0; size * size];
        let mut sum = 0.0;
        let mut index = 0;
    
        for y in 0..size {
            for x in 0..size {
                let x = x as f32 - radius as f32;
                let y = y as f32 - radius as f32;

                // Gaussian function 2d equation

                // 1/2PI*sigma^2 * e ^ (-(x^2 + y^2)/(2 * sigma^2))

                let a = 1.0 / (2.0 * PI* sigma*sigma);
                let b = -(x*x + y*y) / (2.0 * sigma*sigma);
                let b = b.exp();

                let value = a * b;

                kernel[index] = value;
                sum += value;
                index += 1;
            }
        }
    
        // Normalize the kernel
        for value in kernel.iter_mut() {
            *value /= sum;
        }
    
        kernel
    }

    fn gaussian_blur(img: DynamicImage, radius: usize, sigma: f32) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Error> {
        let (width, height) = img.dimensions();

        let size = radius*2+1;

        let kernel = CompVision::create_gaussian_kernel_2d(radius, sigma);

        let mut test_img = image::ImageBuffer::new(width, height);

        let radius = 2*radius+1;

        for y in 0 .. height{
            for x in 0 .. width {
                let mut sum_r = 0f32;
                let mut sum_g = 0f32;
                let mut sum_b = 0f32;
                let mut weight_sum  = 0f32;

                for ky in 0..size{
                    for kx in 0..size{
                        let ix = x as isize + (kx as isize - radius as isize);
                        let iy = y as isize + (ky as isize - radius as isize);

                        if ix >= 0 && ix < width as isize && iy >= 0 && iy < height as isize {
                            let pixel = img.get_pixel(ix as u32, iy as u32);
                            let weight = kernel[ky * size + kx];
                            sum_r += pixel[0] as f32 * weight;
                            sum_g += pixel[1] as f32 * weight;
                            sum_b += pixel[2] as f32 * weight;
                            weight_sum += weight;
                        }
                    }
                }


                let r = (sum_r / weight_sum).min(255.0).max(0.0) as u8;
                let g = (sum_g / weight_sum).min(255.0).max(0.0) as u8;
                let b = (sum_b / weight_sum).min(255.0).max(0.0) as u8;
                test_img.put_pixel(x, y, Rgb([r, g, b]));
            }
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
                            let (red, green, blue, _) = CompVision::split_rbga(pixel);
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

    fn edge_detection_gaussian(img: DynamicImage) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Error> {
        let gaussian_a = CompVision::gaussian_blur(img.clone(), 3, 1.0)?;
        let gaussian_b = CompVision::gaussian_blur(img.clone(), 5, 1.4)?;

        let (width, height) = img.dimensions();
        let mut test_img = image::ImageBuffer::new(width, height);

        for x in 0 .. width{
            for y in 0 .. height{
                let pixel_a = gaussian_a.get_pixel(x, y);
                let pixel_b = gaussian_b.get_pixel(x, y);

                let (r_a, g_a, b_a) = CompVision::split_rbg(pixel_a.clone());
                let (r_b, g_b, b_b) = CompVision::split_rbg(pixel_b.clone());



                let r = (r_b - r_a ).min(255.0).max(0.0) as u8;
                let g = (g_b - g_a ).min(255.0).max(0.0) as u8;
                let b = (b_b - b_a ).min(255.0).max(0.0) as u8;

                let rgb = Rgb([r, g, b]);

                test_img.put_pixel(x, y, rgb);
            }
        }
        
        Ok(test_img)
    }

    fn edge_detection_sobel(img: DynamicImage) -> Result<ImageBuffer<Luma<u8>, Vec<u8>>, Error> {
        let (width, height) = img.dimensions();
        let mut test_img = image::ImageBuffer::new(width, height);

        let sobel_kernel_x = vec![[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];
        let sobel_kernel_y = vec![[-1, -2, -1], [0, 0, 0], [1, 2, 1]];

        for y in 1..height-1 {
            for x in 1..width-1 {
                let mut gx = 0;
                let mut gy = 0;
    
                for ky in 0..3 {
                    for kx in 0..3 {
                        let pixel = img.get_pixel(x + kx - 1, y + ky - 1)[0] as i32;
                        gx += pixel * sobel_kernel_x[ky as usize][kx as usize];
                        gy += pixel * sobel_kernel_y[ky as usize][kx as usize];
                    }
                }
    
                let g = ((gx * gx + gy * gy) as f64).sqrt() as u8;
                test_img.put_pixel(x, y, Luma([g]));
            }
        }

        Ok(test_img)
    }


}
