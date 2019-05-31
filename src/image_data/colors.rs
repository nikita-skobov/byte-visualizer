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
}

// impl Clone for Color {
//     fn clone(&self) -> Self {
//         *self
//     }
// }

pub const VALUES_PER_PIXEL: i32 = 4;
