#[derive(Clone, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn get_blended_pixel_tuple(pix1: (u8, u8, u8, u8), pix2: (u8, u8, u8, u8)) -> (u8, u8, u8, u8) {
        let pix2_alpha_f: f32 = pix2.3 as f32 / 255.0;
        let pix1_alpha_f: f32 = pix1.3 as f32 / 255.0;
        let alpha_inverse = 1f32 - pix2_alpha_f;
        let output_alpha = alpha_inverse * pix1_alpha_f + pix2_alpha_f;

        let red = (((alpha_inverse * pix1_alpha_f * pix1.0 as f32) + (pix2_alpha_f * pix2.0 as f32)) / output_alpha) as u8;
        let green = (((alpha_inverse * pix1_alpha_f * pix1.1 as f32) + (pix2_alpha_f * pix2.1 as f32)) / output_alpha) as u8;
        let blue = (((alpha_inverse * pix1_alpha_f * pix1.2 as f32) + (pix2_alpha_f * pix2.2 as f32)) / output_alpha) as u8;
        let alpha = (output_alpha * 255.0) as u8;

        (red, green, blue, alpha)
    }

    pub fn get_blended_pixel(bottom: Color, top: Color) -> Color {
        let top_alpha_f: f32 = top.alpha as f32 / 255.0;
        let bottom_alpha_f: f32 = bottom.alpha as f32 / 255.0;
        let alpha_inverse = 1f32 - top_alpha_f;
        let output_alpha = alpha_inverse * bottom_alpha_f + top_alpha_f;

        let red = (((alpha_inverse * bottom_alpha_f * bottom.red as f32) + (top_alpha_f * top.red as f32)) / output_alpha) as u8;
        let green = (((alpha_inverse * bottom_alpha_f * bottom.green as f32) + (top_alpha_f * top.green as f32)) / output_alpha) as u8;
        let blue = (((alpha_inverse * bottom_alpha_f * bottom.blue as f32) + (top_alpha_f * top.blue as f32)) / output_alpha) as u8;
        let alpha = (output_alpha * 255.0) as u8;

        Color {
            red,
            green,
            blue,
            alpha,
        }
    }
}

// impl Clone for Color {
//     fn clone(&self) -> Self {
//         *self
//     }
// }

pub const VALUES_PER_PIXEL: i32 = 4;
