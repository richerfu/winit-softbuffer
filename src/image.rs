use image::{error::ImageError, GenericImageView};

use crate::color::Color;
use crate::rectangle::Rectangle;

#[derive(Debug, Clone)]
pub struct Image {
    pub width: f32,
    pub height: f32,
    pub data: Vec<u32>,
}

impl Image {
    pub fn new(width: f32, height: f32, data: Vec<u32>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }
    pub fn load(uri: &str) -> Result<Self, ImageError> {
        let i = match uri {
            "graphics/green.png" => include_bytes!("../graphics/green.png").as_slice(),
            "graphics/purple.png" => include_bytes!("../graphics/purple.png").as_slice(),
            "graphics/orange.png" => include_bytes!("../graphics/orange.png").as_slice(),
            _ => unreachable!(),
        };

        let img = image::load_from_memory(i)?;

        let width = img.width();
        let height = img.height();

        let mut data = Vec::new();

        for pixel in img.pixels() {
            let (_, _, srgba) = pixel;
            let [red, green, blue, alpha] = srgba.0;

            data.push(Color::rgba(red, green, blue, alpha));
        }

        Ok(Image::new(width as f32, height as f32, data))
    }
    pub fn crop(&mut self, rectangle: Rectangle) {
        let width = self.width as usize;
        let height = self.height as usize;

        let x = rectangle.x as usize;
        let y = rectangle.y as usize;
        let w = rectangle.width as usize;
        let h = rectangle.height as usize;

        let xw = x + w;
        let yh = y + h;

        if xw > width || yh > height {
            eprintln!("Crop image: invalid dimensions");
        } else {
            let mut data = Vec::new();

            for j in y..yh {
                for i in x..xw {
                    data.push(self.data[i + j * width]);
                }
            }

            self.width = rectangle.width;
            self.height = rectangle.height;
            self.data = data;
        }
    }
}
