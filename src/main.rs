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
}

fn custom_fractal(width: u32, height: u32, image_file_name: &str) {
    //! An example of generating julia fractals.
    let imgx = 800;
    let imgy = 800;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    let colour_factor = 0.9;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // let r = (0.3 * x as f32) as u8;
        // let g = (0.3 * x as f32) as u8;
        // let b = (0.3 * y as f32) as u8;

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

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save(image_file_name).unwrap();
}
