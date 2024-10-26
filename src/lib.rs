pub mod utility;
pub mod color;

use crate::utility::{Interval, IntervalU32};
use crate::color::{Color, ColorPalette};

use std::fs::File;
use std::io::Write;

// Image Setup
#[derive(Clone)]
pub struct ImageDim {
    pub x: IntervalU32,
    pub y: IntervalU32,
}

impl ImageDim {
    pub fn new(x_max: u32, y_max: u32) -> Self {
        let x: IntervalU32 = IntervalU32::new(0, x_max);
        let y: IntervalU32 = IntervalU32::new(0, y_max);
        ImageDim {
            x,
            y,
        }
    }
}

pub struct MandlebrotImage {
    pub file_name: String,
    pub image_dim: ImageDim,
    pub color_palette: ColorPalette,
    pub color_count: u32,
}

impl MandlebrotImage {
    pub fn new(
        file_name: String,
        image_dim: ImageDim,
        color_palette: ColorPalette,
        color_count: u32,
    ) -> Self {
        MandlebrotImage {
            file_name,
            image_dim,
            color_palette,
            color_count,
        }
    }
    
    pub fn plot_pixel(
        color: Color,
        file: &mut File,
    ) -> std::io::Result<()> {
        // Pixel Algo
        let r = Color::linear_to_gamma(color.r);
        let g = Color::linear_to_gamma(color.g);
        let b = Color::linear_to_gamma(color.b);
    
        let intensity: Interval = Interval::new(0.0, 0.999);
        let ir: u32 = (255.999 * intensity.clamp(color.r)) as u32;
        let ig: u32 = (255.999 * intensity.clamp(color.g)) as u32;
        let ib: u32 = (255.999 * intensity.clamp(color.b)) as u32;
    
        let pixel_triplets = format!("{} {} {} \n",ir , ig, ib);
        file.write_all(pixel_triplets.as_bytes())?;
        
        Ok(())
    }

    pub fn build_file(&self) -> std::io::Result<()> {
        // Setup
        let mut file = File::create(self.file_name.clone())?;
        file.write_all(b"P3\n")?;
        let img_dim = format!("{:?} {:?}\n", self.image_dim.x.max, self.image_dim.y.max);
        file.write_all(img_dim.as_bytes())?;
        file.write_all(b"255\n")?;
            
        // Pixel Algo
        let scale_mandle_x: Interval = Interval::new(-2.0, 0.47);
        let scale_mandle_y: Interval = Interval::new(-1.12, 1.12);

        let image_width: u32 = self.image_dim.x.max;
        let image_height: u32 = self.image_dim.y.max;
        
        for p_y in 0..image_height {
            println!("Scanline's remaining: {:?} ", (image_height - p_y));
            for p_x in 0..image_width {
                let x0 = scale_mandle_x.min + (p_x as f64 / image_width as f64) * (scale_mandle_x.max - scale_mandle_x.min);
                let y0 = scale_mandle_y.min + (p_y as f64 / image_height as f64) * (scale_mandle_y.max - scale_mandle_y.min);
                
                let mut iter: u32 = 0;
                let mut x: f64 = 0.0;
                let mut y: f64 = 0.0;

                // // Expensive Brute Logic: escape time algorithm
                // while x * x + y * y <= 2.0 * 2.0 && iter < self.color_count - 1 {
                //     let xtemp: f64 = x * x - y * y + x0;
                //     y = 2.0 * x * y + y0;
                //     x = xtemp;
                //     iter += 1;
                // }

                // // Optimised Brute Logic: escape time algorithm
                // let mut x2: f64 = 0.0;
                // let mut y2: f64 = 0.0;
                // let mut w: f64 = 0.0;
                
                // while x2 + y2 <= 4.0 && iter < self.color_count -1 {
                //     let x: f64 = x2 - y2 + x0;
                //     let y: f64 = w - x2 - y2 + y0;
                //     x2 = x * x;
                //     y2 = y * y;
                //     w = (x + y) * (x + y);
                //     iter += 1;
                // }

                // Further Optimised Brute Logic: escape time algorithm
                let mut x2: f64 = 0.0;
                let mut y2: f64 = 0.0;
                
                while x2 + y2 <= 4.0 && iter < self.color_count -1 {
                    y = 2.0 * x * y + y0;
                    x = x2 - y2 + x0;
                    x2 = x * x;
                    y2 = y * y;
                    iter += 1;
                }

                let color = self.color_palette.colors[iter as usize].clone();
                MandlebrotImage::plot_pixel(color, &mut file)?;
            }
        }
    
        println!("Generation finished.");
        Ok(())
    }
}
