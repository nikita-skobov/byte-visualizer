use std::fs;

mod helpers;
mod image_data;

fn main() {
    use std::time::{Duration, Instant};

    let (file_path, output_name) = helpers::get_args();
    let start = Instant::now();
    let binary_data = helpers::get_file_as_binary(&file_path);
    println!(
        "Time elapsed from loading file to memory: {:?}",
        start.elapsed()
    );

    let (width, height) = helpers::get_dimensions_from_len(binary_data.len() as f64);
    let mut file_handler = fs::File::create(output_name).unwrap();

    let pixel_size = 2;
    let mut sprite_list: Vec<image_data::Sprite> = Vec::new();
    let total_width: i32 = width as i32 * pixel_size;
    let total_height: i32 = height as i32 * pixel_size;

    println!(
        "there will be {} new pixels and sprites created",
        binary_data.len()
    );

    let mut xoff = 0;
    let mut yoff = 0;
    let start = Instant::now();
    for byte in binary_data {
        let color = image_data::Pixel::new(0, byte, 0, 255);
        let big_pixel = image_data::Sprite::new(xoff, yoff, pixel_size, pixel_size, &color);
        sprite_list.push(big_pixel);

        xoff += pixel_size;
        if xoff > (total_width - pixel_size) {
            xoff = 0;
            yoff += pixel_size;
        }
    }

    println!(
        "Time elapsed from iterating through binary data: {:?}",
        start.elapsed()
    );

    let start = Instant::now();
    let image_data =
        image_data::get_image_data_from_sprites(sprite_list, total_width, total_height);
    println!(
        "Time elapsed from generating u8 vector from image data: {:?}",
        start.elapsed()
    );

    let start = Instant::now();
    match image_data::write_rgba_from_u8(
        &mut file_handler,
        &image_data,
        total_width as u32,
        total_height as u32,
    ) {
        Ok(_) => println!("Image written"),
        Err(e) => println!("Error {:?}", e),
    }
    println!(
        "Time elapsed from writing image data to file: {:?}",
        start.elapsed()
    );
}
