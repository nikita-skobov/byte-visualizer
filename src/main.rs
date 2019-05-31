extern crate png_encode_mini;

use std::fs;

mod helpers;
mod image_data;

fn main() {
    let (file_path, output_name) = helpers::get_args();
    let binary_data = helpers::get_file_as_binary(&file_path);
    let (width, height) = helpers::get_dimensions_from_len(binary_data.len() as f64);
    let mut file_handler = fs::File::create(output_name).unwrap();

    let pixel_size = 3;
    let mut sprite_list: Vec<image_data::Sprite> = Vec::new();
    let total_width: i32 = width as i32 * pixel_size;
    let total_height: i32 = height as i32 * pixel_size;

    let mut xoff = 0;
    let mut yoff = 0;
    for byte in binary_data {
        let color = image_data::Pixel::new(0, byte, 0, 255);
        let big_pixel = image_data::Sprite::new(xoff, yoff, pixel_size, pixel_size, &color);
        sprite_list.push(big_pixel);

        xoff += pixel_size;
        if xoff > total_width {
            xoff = 0;
            yoff += pixel_size;
        }
    }



    let image_data = image_data::get_image_data_from_sprites(sprite_list, total_width, total_height);

    match png_encode_mini::write_rgba_from_u8(&mut file_handler, &image_data, total_width as u32, total_height as u32) {
        Ok(_) => println!("Image written"),
        Err(e) => println!("Error {:?}", e),
    }
}
