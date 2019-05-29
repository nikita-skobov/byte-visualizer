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
