use std::fs::File;
use std::io::BufWriter;
use std::ops::Add;
use std::ops::Mul;

pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Pixel {
    pub fn new(red: u8, green: u8, blue: u8) -> Pixel {
        Pixel {
            red, green, blue
        }
    }
}

pub struct Image {
    data: Vec<u8>,
    channel_size: u8,
    pub width: u32,
    pub height: u32,
}

impl Image {
    pub fn new(width: u32, height: u32, channel_size: u8) -> Image {
        if channel_size != 3 && channel_size != 4 {
            panic!("Error: Invalid channel size {}! Please set either 3 or 4 as the channel size.", channel_size);
        }

        let mut img = Image {
            width, height, channel_size,
            data: Vec::new()
        };

        // Initialize the data. All pixel values zero.
        for _i in 0..(width * height * channel_size as u32) {
            img.data.push(0);
        }

        img
    }

    pub fn write_to_file(&self, file_path: &str) {
        let image_file = File::create(&file_path)
        .expect(&format!("Error: Couldn't create file {}!", file_path)[..]);

        let buffer_writer = BufWriter::new(image_file);

        let mut encoder = png::Encoder::new(buffer_writer, self.width, self.height);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_color(match self.channel_size {
            3 => png::ColorType::Rgb,
            4 => png::ColorType::Rgba,
            _ => png::ColorType::Grayscale // Set grayscale if channel size is invalid.
        });
        
        let mut write_header = encoder.write_header().unwrap();
        write_header.write_image_data(&self.data[..]).unwrap();
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: Pixel) -> Result<(), &str> {
        if x > self.width {
            return Err("Invalid x value! x value should be inbetween 0 and image width - 1!");
        }
        if y > self.height {
            return Err("Invalid y value! y value should be inbetween 0 and image height - 1!")
        }
        self.data[(y * self.width * self.channel_size as u32 + x * self.channel_size as u32) as usize] = pixel.red;
        self.data[(y * self.width * self.channel_size as u32 + x * self.channel_size as u32 + 1) as usize] = pixel.green;
        self.data[(y * self.width * self.channel_size as u32 + x * self.channel_size as u32 + 2) as usize] = pixel.blue;
        Ok(())
    }
}

impl Mul<f32> for Pixel {
    type Output = Pixel;
    fn mul(self, rhs: f32) -> Pixel {
        // Clamp the values of pixels between 0 and 255.
        let red: u8 = f32::clamp(self.red as f32 * rhs, 0.0, 255.0) as u8;
        let green: u8 = f32::clamp(self.green as f32 * rhs, 0.0, 255.0) as u8;
        let blue: u8 = f32::clamp(self.blue as f32 * rhs, 0.0, 255.0) as u8;

        Pixel {
            red,
            green,
            blue
        }
    }
}

impl Add<Pixel> for Pixel {
    type Output = Pixel;
    fn add(self, rhs: Pixel) -> Pixel {
        Pixel {
            red: if (self.red as u16 + rhs.red as u16) > 255 {
                255
            }
            else {
                self.red + rhs.red
            },

            green: if (self.green as u16 + rhs.green as u16) > 255 {
                255
            }
            else {
                self.green + rhs.green
            },

            blue: if (self.blue as u16 + rhs.blue as u16) > 255 {
                255
            }
            else {
                self.blue + rhs.blue
            }
        }
    }
}

impl Mul<Pixel> for f32 {
    type Output = Pixel;
    fn mul(self, rhs: Pixel) -> Pixel {
        rhs * self
    }
}