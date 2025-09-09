use crate::vector::Vec3;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec3>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Image {
            width,
            height,
            pixels: vec![Vec3::zero(); (width * height) as usize],
        }
    }
    
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Vec3) {
        if x < self.width as usize && y < self.height as usize {
            let index = y * self.width as usize + x;
            self.pixels[index] = color;
        }
    }
    
    pub fn get_pixel(&self, x: usize, y: usize) -> Vec3 {
        if x < self.width as usize && y < self.height as usize {
            let index = y * self.width as usize + x;
            self.pixels[index]
        } else {
            Vec3::zero()
        }
    }
    
    pub fn output_ppm(&self) {
        println!("P3");
        println!("{} {}", self.width, self.height);
        println!("255");
        
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.get_pixel(x as usize, y as usize);
                let r = (pixel.x.clamp(0.0, 1.0) * 255.0) as u8;
                let g = (pixel.y.clamp(0.0, 1.0) * 255.0) as u8;
                let b = (pixel.z.clamp(0.0, 1.0) * 255.0) as u8;
                println!("{} {} {}", r, g, b);
            }
        }
    }
    
    pub fn save_ppm(&self, filename: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;
        
        let mut file = File::create(filename)?;
        writeln!(file, "P3")?;
        writeln!(file, "{} {}", self.width, self.height)?;
        writeln!(file, "255")?;
        
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.get_pixel(x as usize, y as usize);
                let r = (pixel.x.clamp(0.0, 1.0) * 255.0) as u8;
                let g = (pixel.y.clamp(0.0, 1.0) * 255.0) as u8;
                let b = (pixel.z.clamp(0.0, 1.0) * 255.0) as u8;
                writeln!(file, "{} {} {}", r, g, b)?;
            }
        }
        
        Ok(())
    }
}
