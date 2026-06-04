use std::fs;

fn main() {
    println!("generating images");
    generate_custom_rgb();
}

enum ColourNames {
    Red,
    Green,
    Blue,
    RedBlue,
    RedGreen,
    BlueGreen,
    RedGreenBlue,
    Black,
    White
}

fn generate_custom_rgb() {
    let width = 800;
    let height = 800;
    let colour_factor = 0.5;

    custom_rgb(width, height, "red-green.png", colour_factor, ColourNames::RedGreen);
    custom_rgb(width, height, "red-blue.png", colour_factor, ColourNames::RedBlue);
    custom_rgb(width, height, "blue-green.png", colour_factor, ColourNames::BlueGreen);
    custom_rgb(width, height, "red.png", colour_factor, ColourNames::Red);
    custom_rgb(width, height, "blue.png", colour_factor, ColourNames::Blue);
    custom_rgb(width, height, "green.png", colour_factor, ColourNames::Green);
    custom_rgb(width, height, "red-green-blue.png", colour_factor, ColourNames::RedGreenBlue);
    custom_rgb(width, height, "white.png", colour_factor, ColourNames::White);
    custom_rgb(width, height, "black.png", colour_factor, ColourNames::Black);
    custom_rgb(200, 200, "blue-green-200x200.png", colour_factor, ColourNames::BlueGreen);

    let list_of_factors = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];

    for factor in list_of_factors {
        custom_rgb(800, 800, format!("red-green-{}.png", factor).as_str(), factor, ColourNames::RedGreen);
        custom_rgb(800, 800, format!("red-blue-{}.png", factor).as_str(), factor, ColourNames::RedBlue);
        custom_rgb(800, 800, format!("blue-green-{}.png", factor).as_str(), factor, ColourNames::BlueGreen);
        custom_rgb(800, 800, format!("red-{}.png", factor).as_str(), factor, ColourNames::Red);
        custom_rgb(800, 800, format!("blue-{}.png", factor).as_str(), factor, ColourNames::Blue);
        custom_rgb(800, 800, format!("green-{}.png", factor).as_str(), factor, ColourNames::Green);
        custom_rgb(800, 800, format!("red-green-blue-{}.png", factor).as_str(), factor, ColourNames::RedGreenBlue);
    }

        custom_rgb(800, 800, format!("red-green-blue-{}.png", 2).as_str(), 2.0, ColourNames::RedGreenBlue);
        custom_rgb(800, 800, format!("red-green-blue-{}.png", 5).as_str(), 5.0, ColourNames::RedGreenBlue);
}


fn custom_rgb(width: u32, height: u32, image_file_name: &str, colour_factor: f32, colour_names: ColourNames) {
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(width, height);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {

        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        match colour_names {
            ColourNames::Red => {
                r = (colour_factor * x as f32) as u8;
            }
            ColourNames::Blue => {
                b = (colour_factor * y as f32) as u8;
            }
            ColourNames::Green => {
                g = (colour_factor * x as f32) as u8;
            }
            ColourNames::RedGreen => {
                r = (colour_factor * x as f32) as u8;
                g = (colour_factor * x as f32) as u8;
            }
            ColourNames::RedBlue => {
                r = (colour_factor * x as f32) as u8;
                b = (colour_factor * y as f32) as u8;
            }
            ColourNames::BlueGreen => {
                b = (colour_factor * y as f32) as u8;
                g = (colour_factor * x as f32) as u8;
            }
            ColourNames::RedGreenBlue => {
                r = (colour_factor * x as f32) as u8;
                b = (colour_factor * y as f32) as u8;
                g = (colour_factor * x as f32) as u8;
            }
            ColourNames::White => {
                r = 255;
                b = 255;
                g = 255;
            }
            ColourNames::Black => {
                r = 0;
                b = 0;
                g = 0;
            }
        }

        *pixel = image::Rgb([r, g, b]);
    }

    let dir_name = get_dir_name(colour_names);

    let dir_path =  format!("test-images/{}", dir_name);

    fs::create_dir(dir_path.as_str()).ok();

    let save_file_path = format!("{}/{}", dir_path, image_file_name);

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save(save_file_path).unwrap();
}

fn get_dir_name(colour_names: ColourNames) -> String {
    match colour_names {
        ColourNames::Red => {
            "red".to_lowercase()
        }
        ColourNames::Blue => {
            "blue".to_lowercase()
        }
        ColourNames::Green => {
            "green".to_lowercase()
        }
        ColourNames::RedGreen => {
            "red-green".to_lowercase()
        }
        ColourNames::RedBlue => {
            "red-blue".to_lowercase()
        }
        ColourNames::BlueGreen => {
            "blue-green".to_lowercase()
        }
        ColourNames::RedGreenBlue => {
            "red-green-blue".to_lowercase()
        }
        ColourNames::White => {
            "white".to_lowercase()
        }
        ColourNames::Black => {
            "black".to_lowercase()
        }
    }
}