use std::fs;

fn main() {
    println!("generating images");
    generate_custom_rgb();
}

fn generate_custom_rgb() {
    let width = 800;
    let height = 800;
    let colour_factor = 0.5;

    custom_rgb(width, height, "red-green.png", colour_factor);
    custom_rgb(width, height, "red-blue.png", colour_factor);
    custom_rgb(width, height, "blue-green.png", colour_factor);
    custom_rgb(200, 200, "blue-green-200x200.png", colour_factor);

    let list_of_factors = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];

    for factor in list_of_factors {
        custom_rgb(800, 800, format!("red-green-{}.png", factor).as_str(), factor);
        custom_rgb(800, 800, format!("red-blue-{}.png", factor).as_str(), factor);
        custom_rgb(800, 800, format!("blue-green-{}.png", factor).as_str(), factor);
    }

}

fn custom_rgb(width: u32, height: u32, image_file_name: &str, colour_factor: f32) {
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(width, height);

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

    let mut dir_name = "red-green";

    if image_file_name.contains("red-green") {
        dir_name = "red-green";
    }

    if image_file_name.contains("red-blue") {
        dir_name = "red-blue";
    }

    if image_file_name.contains("blue-green") {
        dir_name = "blue-green";
    }

    let dir_path =  format!("test-images/{}", dir_name);

    fs::create_dir(dir_path.as_str()).ok();

    let save_file_path = format!("{}/{}", dir_path, image_file_name);

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save(save_file_path).unwrap();
}
