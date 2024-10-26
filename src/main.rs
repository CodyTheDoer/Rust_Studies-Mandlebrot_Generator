use std::fs::File;
use std::io::Write;

use mandlebrot_set::utility::{Interval, IntervalU32};

fn main() {
    let file_name = "image.ppm".to_string();                                // Filename
    let image_dim: ImageDim = ImageDim::new(1600, 900);                     // Image Dimensions
    let color_palette: ColorPalette = ColorPalette::init_grayscale(35);    // Init Grayscale Palette black {grayscale 1-9} white
    let color_count = color_palette.color_count;

    let image = MandlebrotImage::new(
        file_name,
        image_dim,
        color_palette,
        color_count,
    );
    
    let _ = MandlebrotImage::build_file(&image);
}

// // // // // Library Logic // // // // // 

// Image
struct MandlebrotImage {
    file_name: String,
    image_dim: ImageDim,
    color_palette: ColorPalette,
    color_count: u32,
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
        // let r = Color::linear_to_gamma(color.r());
        // let g = Color::linear_to_gamma(color.g());
        // let b = Color::linear_to_gamma(color.b());
    
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
                
                let mut x: f64 = 0.0;
                let mut y: f64 = 0.0;
                let mut iter: u32 = 0;

                while x * x + y * y <= 2.0 * 2.0 && iter < self.color_count - 1 {
                    let xtemp: f64 = x * x - y * y + x0;
                    y = 2.0 * x * y + y0;
                    x = xtemp;
                    iter += 1;
                }
                if iter % 100 == 0 {
                }

                let color = self.color_palette.colors[iter as usize].clone();
                MandlebrotImage::plot_pixel(color, &mut file)?;
            }
        }
    
        println!("Generation finished.");
        Ok(())
    }
}

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

// // Pixel
// struct TargetPixel {
//     x: u32,
//     y: u32,
// }

// impl TargetPixel {
//     pub fn new(x: u32, y: u32) -> Self {
//         TargetPixel {
//             x,
//             y,
//         }
//     }

//     pub fn set_x(&self, x: u32) -> Self {
//         let x: u32 = x;
//         TargetPixel {
//             x,
//             y: self.y,
//         }
//     }

//     pub fn set_y(&self, y: u32) -> Self {
//         let y: u32 = y;
//         TargetPixel {
//             x: self.x,
//             y,
//         }
//     }
// }

// Color
#[derive(Clone)]
struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color {
            r,
            g,
            b,
        }
    }
    
    // pub fn random() -> Self {
    //     let r: f64 = Utility::random_float_range(Interval::new(0.0, 1.0));
    //     let g: f64 = Utility::random_float_range(Interval::new(0.0, 1.0));
    //     let b: f64 = Utility::random_float_range(Interval::new(0.0, 1.0));
    //     Color {
    //         r,
    //         g,
    //         b,
    //     }
    // }

    // pub fn linear_to_gamma( // corrects colors to consider gamma space alterations
    //     linear_component: f64
    // ) -> f64 { 
    //     if linear_component > 0.0 {
    //         return linear_component.sqrt();
    //     }
    //     return 0.0;
    // }
}

// Color Palette
struct ColorPalette {
    colors: Vec<Color>,
    color_count: u32,
}

impl ColorPalette {
    pub fn init_grayscale(grayscale_count_max: u32) -> ColorPalette {
        let color_count: u32 = 1 + grayscale_count_max + 1; // Black & Grays & White

        let mut colors: Vec<Color> = Vec::new();

        let black: Color = Color::new(0.0, 0.0, 0.0);
        colors.push(black);

        for i in 0..grayscale_count_max {
            let x_scaled: f64 = i as f64 / grayscale_count_max as f64;
            let color: Color = Color::new(x_scaled, x_scaled, x_scaled);
            colors.push(color);
        }

        let white: Color = Color::new(1.0, 1.0, 1.0);
        colors.push(white);
        
        ColorPalette {
            colors,
            color_count,
        }
    }
}