use crate::colors::Color; // Importa el módulo colors del archivo colors.rs

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    buffer: Vec<u32>,
    background_color: Color,
    current_color: Color,
    flip_y: bool,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize, flip_y: bool) -> Framebuffer {
        let background_color = Color::from_hex(0x000000); // Default to black
        let current_color = Color::from_hex(0xFFFFFF); // Default to white
        let buffer = vec![background_color.to_hex(); width * height];
        Framebuffer {
            width,
            height,
            buffer,
            background_color,
            current_color,
            flip_y,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(self.background_color.to_hex());
    }

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            let index = if self.flip_y {
                (self.height - y - 1) * self.width + x
            } else {
                y * self.width + x
            };
            self.buffer[index] = self.current_color.to_hex();
        }
    }

    pub fn get_color_at(&self, x: usize, y: usize) -> Option<Color> {
        if x < self.width && y < self.height {
            let index = if self.flip_y {
                (self.height - y - 1) * self.width + x
            } else {
                y * self.width + x
            };
            Some(Color::from_hex(self.buffer[index]))
        } else {
            None
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = Color::from_hex(color);
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = Color::from_hex(color);
    }

    pub fn render_buffer(&self, file_path: &str) -> std::io::Result<()> {
        crate::bmp::write_bmp_file(file_path, &self.buffer, self.width, self.height)
    }
}