pub fn get_image_data(
    binary_data: &Vec<u8>,
    width: i32,
    height: i32,
    values_per_pixel: i32,
) -> Vec<u8> {
    let num_pixels = width * height;
    let vector_size = num_pixels * values_per_pixel;
    let mut image_data: Vec<u8> = vec![0; vector_size as usize];

    let mut index = 0;
    let mut height_level = height - 1;

    let width_offset = values_per_pixel * width;
    let mut height_offset = height - (2 * height_level) - 1;
    let mut iterator_offset = width_offset * height_offset;

    for i in 0..image_data.len() {
        let iterator = i as i32;
        // the iterator corresponds to bytes, whereas the index
        // corresponds to pixels. a pixel has 3 bytes (if RGB) or 4 bytes (if RGBA).
        // every time the iterator surpasses values_per_pixel, we increment the index.

        if iterator % values_per_pixel == 0 {
            let red_value = if index < binary_data.len() {
                binary_data[index]
            } else {
                0
            };

            // every time the index surpasses the width, we go down one height level
            if index > 1 && index as i32 % width == 0 {
                height_level -= 1;

                // we recalculate height offset, and iterator offset
                // only when the height level changes
                height_offset = height - (2 * height_level) - 1;
                iterator_offset = width_offset * height_offset;
            }

            let pixel_index = iterator - iterator_offset;
            let pixel_index: usize = pixel_index as usize;
            // png_encode_mini takes an array of u8 where
            // every 3 or 4 elements represents a pixel
            // and outputs in the following order:
            // [
            //   6  7  8
            //   3  4  5
            //   0  1  2
            // ]
            // but we want:
            // [
            //   0  1  2
            //   3  4  5
            //   6  7  8
            // ]
            // so we calculate a pixel index which is based off an offset
            // from the height level and the width.

            image_data[pixel_index] = 0; // R
            image_data[pixel_index + 1] = red_value; // G
            image_data[pixel_index + 2] = 0; // B
            image_data[pixel_index + 3] = 255; // A
            index += 1;
        }
    }

    image_data
}


pub fn get_image_data_from_sprites(sprite_vec: Vec<super::Sprite>, width: i32, height: i32) -> Vec<u8> {
    let num_pixels: i128 = width as i128 * height as i128;
    let vector_size = num_pixels * super::pixels::VALUES_PER_PIXEL as i128;
    let mut image_data: Vec<u8> = vec![0; vector_size as usize];


    for sprite in sprite_vec {
        let num_pixels_in_sprite: i32 = sprite.pixel_data.len() as i32;
        let first_pixel_color = sprite.pixel_data[0].clone();

        let mut height_level = height - sprite.y_offset - 1;
        // - 1 because we want it to be 0 indexed
        let mut height_offset = height_level * width;
        
        for i in 0..sprite.pixel_data_max_len {
            let pixel_color: &super::pixels::Pixel = if i > num_pixels_in_sprite { &sprite.pixel_data[0] } else { &first_pixel_color };
            // if user wants entire sprite to be a single colored square they can just supply one color
            // which would be first_pixel_color. otherwise if they ever call set_pixel_data, then
            // sprite.pixel_data.len() becomes set to sprite.pixel_data_max_len

            let width_offset = i % sprite.width;
            let width_index = sprite.x_offset + width_offset;


            if i > 0 && width_offset == 0 {
                height_level -= 1;
                // every time i surpasses the width, we decrease the
                // height level by 1
                height_offset = height_level * width;
                // recalculate height offset only when height level changes
            }


            if height_level < 0 || height_level >= height {
                // height level must be less than image height and non negative
                // otherwise pixel out of bounds, so skip it.
                continue;
            }
            if width_index < 0 || width_index >= width {
                // width index must be greater than 0, and less than img_width
                // otherwise the current pixel of the sprite is out of bounds,
                //and therefor we dont modify the image data for this pixel
                continue;
            }

            let pixel_index = height_offset + width_index;
            let pixel_index = pixel_index * super::pixels::VALUES_PER_PIXEL;
            let pixel_index: usize = pixel_index as usize;

            let bottom_pixel = (
                image_data[pixel_index],
                image_data[pixel_index + 1],
                image_data[pixel_index + 2],
                image_data[pixel_index + 3],
            );

            let top_pixel = (
                pixel_color.red,
                pixel_color.green,
                pixel_color.blue,
                pixel_color.alpha,
            );

            let (
                new_red,
                new_green,
                new_blue,
                new_alpha,
            ) = super::pixels::Pixel::get_blended_pixel_tuple(bottom_pixel, top_pixel);

            image_data[pixel_index] = new_red;
            image_data[pixel_index + 1] = new_green;
            image_data[pixel_index + 2] = new_blue;
            image_data[pixel_index + 3] = new_alpha;
        }
    }


    image_data
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_image_data_from_sprites_works() {
        use std::fs;

        let blue = super::super::pixels::Pixel::new(0, 0, 255, 255);
        let yellow_transparent = super::super::pixels::Pixel::new(255, 255, 0, 125);

        let blue_background = super::super::Sprite::new(0, 0, 3, 3, &blue);
        let yellow_center = super::super::Sprite::new(1, 1, 1, 1, &yellow_transparent);

        let sprite_list = vec![blue_background, yellow_center];

        let image_data = super::get_image_data_from_sprites(sprite_list, 3, 3);

        let expected_image_data = vec![0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 255, 125, 125, 129, 255,0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 255];

        assert_eq!(image_data, expected_image_data);
        // let mut file_handler = fs::File::create("plswork.png").unwrap();

        // match png_encode_mini::write_rgba_from_u8(&mut file_handler, &image_data, 3, 3) {
        //     Ok(_) => println!("Image written"),
        //     Err(e) => println!("Error {:?}", e),
        // }
    }
}
