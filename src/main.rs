extern crate png_encode_mini;

use std::fs;

mod helpers;
mod image_data;

fn main() {
    let (file_path, output_name) = helpers::get_args();
    let binary_data = helpers::get_file_as_binary(&file_path);
    let (width, height) = helpers::get_dimensions_from_len(binary_data.len() as f64);
    let mut file_handler = fs::File::create(output_name).unwrap();
    let values_per_pixel = 4;

    let image_data = image_data::get_image_data(
        &binary_data,
        width as i32,
        height as i32,
        values_per_pixel as i32,
    );

    match png_encode_mini::write_rgba_from_u8(&mut file_handler, &image_data, width, height) {
        Ok(_) => println!("Image written"),
        Err(e) => println!("Error {:?}", e),
    }

    println!(
        "data len: {}, width: {}, height: {}, width*height: {}",
        binary_data.len(),
        width,
        height,
        width * height
    );
}
