mod get_image_data;

pub use get_image_data::get_image_data;

const VALUES_PER_PIXEL: i32 = 4;

pub struct Sprite {
    x_offset: i32,
    y_offset: i32,
    width: i32,
    height: i32,
    pixel_data: Vec<u8>,
    pixel_data_max_len: i32,
}

impl Sprite {
    pub fn new(x_offset: i32, y_offset: i32, width: i32, height: i32, color: u8) -> Sprite {
        Sprite {
            x_offset,
            y_offset,
            width,
            height,
            pixel_data_max_len: width * height * VALUES_PER_PIXEL,
            pixel_data: vec![color; 1],
            // make a vector with only 1 element by default.
            // if users want to customize the pixel data, they can call set_pixel_data
            // which will modify the vector.
        }
    }

    pub fn set_pixel_data(&mut self, data: &Vec<u8>) {
        let data_len: i32 = data.len() as i32;
        if data_len > self.pixel_data_max_len {
            // instead of panicking, just give a console warning
            // and truncate their data to fit the pixel_data_max_len
            println!("WARNING: setting pixel data with a length of {} but the sprite only has room for {}", data_len, self.pixel_data_max_len);
        }

        for i in 0..self.pixel_data_max_len {
            if i == data_len {
                // in case user provides a data with length smaller than pixel_data_max_len
                // make sure to break here to prevent accessing elements that don't exist
                break
            }

            self.pixel_data.push(data[i as usize]);
        }
    }
}