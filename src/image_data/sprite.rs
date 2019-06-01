use super::pixels;

pub struct Sprite {
    pub x_offset: i32,
    pub y_offset: i32,
    pub width: i32,
    pub height: i32,
    pub pixel_data: Vec<pixels::Pixel>,
    pub pixel_data_max_len: i32,
}

impl Sprite {
    pub fn new(
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
        color: &pixels::Pixel,
    ) -> Sprite {
        let pixel_data = vec![color.clone()];
        // make a vector with only 1 element by default.
        // if users want to customize the pixel data, they can call set_pixel_data
        // which will modify the vector.

        Sprite {
            x_offset,
            y_offset,
            width,
            height,
            pixel_data_max_len: width * height,
            pixel_data,
        }
    }

    pub fn set_pixel_data(&mut self, data: Vec<pixels::Pixel>) {
        let data_len: i32 = data.len() as i32;
        if data_len > self.pixel_data_max_len {
            // instead of panicking, just give a console warning
            // and truncate their data to fit the pixel_data_max_len
            println!("WARNING: setting pixel data with a length of {} but the sprite only has room for {}", data_len, self.pixel_data_max_len);
        }

        let current_color = self.pixel_data[0].clone();
        self.pixel_data
            .resize(self.pixel_data_max_len as usize, current_color);

        let mut index = 0;
        for color_value in data {
            if index == self.pixel_data_max_len {
                break;
                // in case user provides a data with length greater than pixel_data_max_len
                // make sure to break here to prevent accessing elements that don't exist
            }

            self.pixel_data[index as usize] = color_value;
            index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn new_works() {
        let red = super::pixels::Pixel::new(255, 0, 0, 255);
        super::Sprite::new(1, 1, 10, 10, &red);
    }

    #[test]
    fn only_has_one_pixel_value() {
        let red = super::pixels::Pixel::new(255, 0, 0, 255);
        let my_sprite = super::Sprite::new(20, 20, 10, 10, &red);
        assert_eq!(my_sprite.pixel_data.len(), 1);
    }

    #[test]
    fn set_pixel_data_doesnt_error_if_data_is_larger_than_pixel_data_max_len() {
        let red = super::pixels::Pixel::new(255, 0, 0, 255);
        let mut my_sprite = super::Sprite::new(0, 0, 1, 1, &red);

        my_sprite.set_pixel_data(vec![red; 5]);
        // width * height = 1 * 1 = 1
        // we are setting pixel data with a vector of 5 elements, whereas
        // the pixel_data_max_len is 1
    }

    #[test]
    fn set_pixel_data_sets_len_to_max() {
        let red = super::pixels::Pixel::new(255, 0, 0, 255);
        let mut my_sprite = super::Sprite::new(0, 0, 3, 2, &red);
        assert_eq!(my_sprite.pixel_data.len(), 1);
        // max len = 3 * 2 = 6

        my_sprite.set_pixel_data(vec![red; 4]);
        // set pixel data should resize the vector
        // to be of length 6 even though we are only passing
        // a vector of length 4
        assert_eq!(my_sprite.pixel_data.len(), 6);
    }
}
