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

        self.pixel_data.resize(self.pixel_data_max_len as usize, self.pixel_data[0]);

        for i in 0..self.pixel_data_max_len {
            if i == data_len {
                // in case user provides a data with length smaller than pixel_data_max_len
                // make sure to break here to prevent accessing elements that don't exist
                break;
            }

            let index: usize = i as usize;
            self.pixel_data[index] = data[index];
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn new_works() {
        super::Sprite::new(1, 1, 10, 10, 0);
    }

    #[test]
    fn only_has_one_pixel_value() {
        let my_sprite = super::Sprite::new(20, 20, 10, 10, 100);
        assert_eq!(my_sprite.pixel_data.len(), 1);
    }

    #[test]
    fn set_pixel_data_doesnt_error_if_data_is_larger_than_pixel_data_max_len() {
        let mut my_sprite = super::Sprite::new(0, 0, 1, 1, 29);

        my_sprite.set_pixel_data(&vec![0; 5]);
        // width * height * values_per_pixel = 1 * 1 * 4 = 4
        // we are setting pixel data with a vector of 5 elements, whereas
        // the pixel_data_max_len is 4
    }

    #[test]
    fn set_pixel_data_sets_len_to_max() {
        let mut my_sprite = super::Sprite::new(0, 0, 3, 2, 29);
        assert_eq!(my_sprite.pixel_data.len(), 1);
        // max len = 3 * 2 * 4 = 24

        my_sprite.set_pixel_data(&vec![0; 10]);
        // set pixel data should resize the vector
        // to be of length 24 even though we are only passing
        // a vector of length 10
        assert_eq!(my_sprite.pixel_data.len(), 24);
    }
}
