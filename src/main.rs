use std::env;
use std::path::Path;
use image::{DynamicImage, GenericImageView};
use std::fs::File;
use std::io::Write;

fn main() {
    // args1 = image path; args2 ?= scaling factor
    let args: Vec<String> = env::args().collect();

    // Image must have been provided as an argument
    if args.len() < 2 {
        println!("Please input an image and an optional scaling factor to use the program.");
        return;
    }

    // Open image
    let original_img = image::open(&Path::new(&args[1])).expect("input should be valid path to an image file");

    // Scaling parameter
    let scale: u32;
    if args.len() >= 3 {
        scale = args[2].parse().expect("scaling factor should be an unsigned integer");
    } else {
        scale = 1;
    }

    // Resize image
    let img = original_img.resize(
        original_img.width() / scale,
        original_img.height() / scale,
        image::imageops::FilterType::Lanczos3,
    );

    // Convert pixels to ASCII characters
    let ascii = convert_image(img);

    // TODO: Don't hardcode output file name
    let mut file = File::create("output.txt").unwrap();

    // Write output to file and stdout
    // TODO: Give option for output method
    for row in ascii {
        let row_str: String = row.iter().collect();
        writeln!(&mut file, "{}", row_str).unwrap();
        println!("{}", row_str);
    }
}

fn convert_image(img: DynamicImage) -> Vec<Vec<char>> {
    let grayscale: &str = "@%#XoO0*/+=-:. ";

    let w = img.dimensions().0 as usize;
    let h = img.dimensions().1 as usize;

    let mut res = vec![vec![' '; w]; h];

    for y in 0..h {
        for x in 0..w {
            let pixel = img.get_pixel(x as u32, y as u32);
            // Use luminosity method for brightness to grayscale
            let brightness =
                    (0.299 * pixel[0] as f32
                    + 0.587 * pixel[1] as f32
                    + 0.114 * pixel[2] as f32) / 255.0;
            let index = (brightness * (grayscale.len() - 1) as f32) as usize;
            res[y][x] = grayscale.chars().nth(index).unwrap();
        }
    }

    res
}
