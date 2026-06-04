use image::GenericImageView;

fn main() {
    println!("generating images");
    generate_custom_fractals();
}

fn generate_custom_fractals() {
    let width = 800;
    let height = 800;

    custom_fractal(width, height, "red-green.png");
    custom_fractal(width, height, "red-blue.png");
    custom_fractal(width, height, "blue-green.png");
    custom_fractal(200, 200, "blue-green-200x200.png");
}

fn custom_fractal(width: u32, height: u32, image_file_name: &str) {
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(width, height);

    let colour_factor = 0.5;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (colour_factor * x as f32) as u8;
        let g = (colour_factor * x as f32) as u8;
        let b = (colour_factor * y as f32) as u8;

        if image_file_name.contains("red-green") {
            *pixel = image::Rgb([r, g, 0]);
        }

        if image_file_name.contains("red-blue") {
            *pixel = image::Rgb([r, 0, b]);
        }

        if image_file_name.contains("blue-green") {
            *pixel = image::Rgb([0, g, b]);
        }
    }

    let save_file_path = format!("test-images/{}", image_file_name);

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save(save_file_path).unwrap();
}
