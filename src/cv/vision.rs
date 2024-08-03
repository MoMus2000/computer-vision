use std::f32::consts::PI;
use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader, Luma, Rgba, Rgb};
use anyhow::{Error, Ok};
use rayon::vec;
use std::collections::HashMap;
use rand::{thread_rng, Rng};

pub struct CompVision{
    pub image: DynamicImage
}

impl CompVision{

    //Standard Grayscale Conversion: This formula is widely used in various image processing 
    //applications and standards, including video encoding and image processing libraries. 
    //It provides a good approximation of perceived brightness, which is why it's commonly 
    //used in practice.

    const RED_GRAY : f32 = 0.299;
    const GREEN_GRAY: f32 = 0.587;
    const BLUE_GRAY : f32 = 0.114;

    pub fn new(image_path: &str) -> Result<CompVision, Error>{
        let img = ImageReader::open(image_path)?.decode()?; Ok(CompVision{image: img})
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

    pub fn to_grayscale(img: DynamicImage) -> Result<ImageBuffer<Luma<u8>, Vec<u8>>, Error>{
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

    pub fn posterize(img: DynamicImage, level: usize) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Error>{
        let (width, height) = img.dimensions();

        let factor = 256 / level as u32;

        let mut test_img = image::ImageBuffer::new(width, height);

        for x in 0.. width {
            for y in 0 .. height {
                let pixel = img.get_pixel(x, y);

                let r = ((pixel.0[0] as u32 / factor) * factor) as u8;
                let g = ((pixel.0[1] as u32 / factor) * factor) as u8;
                let b = ((pixel.0[2] as u32 / factor) * factor) as u8;

                test_img.put_pixel(x, y, Rgb([r, g, b]));

            }
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

    pub fn edge_detection_sobel(img: DynamicImage) -> Result<ImageBuffer<Luma<u8>, Vec<u8>>, Error> {
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


    pub fn kmeans(img: DynamicImage, palette: usize)-> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Error> {
        let (width, height) = img.dimensions();
        let mut test_img = image::ImageBuffer::new(width, height);

        let mut map : HashMap<Rgba<u8>, usize> = HashMap::new();

        for x in 0 .. width{
            for y in 0 .. height{
                let pixel = img.get_pixel(x, y);
                if map.contains_key(&pixel){
                    continue;
                }
                else{
                    map.insert(pixel, 1);
                }
            }
        }

        CompVision::create_cluster(map, palette);

        Ok(test_img)
    }

    fn create_cluster(map: HashMap<Rgba<u8>, usize>, palette: usize){
        let vectors = CompVision::map_to_vec(map);
        let centroids = CompVision::initialize_centroids(&vectors, palette);
        assert_eq!(centroids.len() , palette);
        for i in 0 .. 10 {
            let clusters = CompVision::assign_clusters(&centroids, &vectors);
        }
    }

    fn assign_clusters(centroids: &Vec<Vec3d>, pixels: &Vec<Vec3d>) -> Vec<usize>{
        let mut assigned_cluster = vec![0; pixels.len()];

        for (i, data) in pixels.iter().enumerate(){
            let mut min_dist = f32::MAX;
            for(j, centroid) in centroids.iter().enumerate(){
                let dist = data.calcluate_distance(*centroid);
                if dist < min_dist{
                    min_dist = dist;
                    assigned_cluster[i]=j;
                }
            }
        }

        assigned_cluster

    }

    fn update_centroids(data:Vec<Vec3d>, clusters: Vec<Vec3d>, k: usize) {
    }

    fn map_to_vec(map: HashMap<Rgba<u8>, usize>) -> Vec<Vec3d>{
        let mut vector = Vec::<Vec3d>::new();
        for key in map.keys(){
        let split_pixel = CompVision::split_rbga(*key);
           vector.push(Vec3d::new(split_pixel.0 as f32, split_pixel.1 as f32, split_pixel.2 as f32));
        }
        vector
    }

    fn initialize_centroids(data: &Vec<Vec3d>, palette: usize) -> Vec<Vec3d>{
        let mut centroids = Vec::<Vec3d>::new();
        let mut rng = rand::thread_rng();
        for _ in 0 .. palette {
            let index = rng.gen_range(0..data.len());
            centroids.push(data[index].clone());
        }
        centroids
    }

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

#[cfg(test)]
mod tests{
    use super::CompVision;
    use anyhow::Error;


    #[test]
    pub fn test_k_means() -> Result<(), Error>{
        let img = CompVision::new("/Users/mmuhammad/Desktop/8-gTumJhJckqsGlr3.png")?;

        let res = CompVision::kmeans(img.image, 10)?;

        Ok(()) 
    }

}


#[derive(Debug, Clone, Copy)]
pub struct Vec3d {
    pub x : f32,
    pub y : f32,
    pub z: f32
}

impl Vec3d{
    pub fn new(x: f32, y: f32, z: f32) -> Vec3d{
        Vec3d { x, y, z }
    }

    pub fn calcluate_distance(&self, vec: Vec3d) -> f32{
        (self.x + vec.x).powf(2.0) + (self.y + vec.y).powf(2.0) + (self.z+vec.z).powf(2.0).powf(0.5)
    }

}