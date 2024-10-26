use crate::utility::Utility;
use crate::Interval;

// Color
#[derive(Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color {
            r,
            g,
            b,
        }
    }
    
    pub fn random() -> Self {
        let r: f64 = Utility::random_float_range(Interval::new(0.0, 1.0));
        let g: f64 = Utility::random_float_range(Interval::new(0.0, 1.0));
        let b: f64 = Utility::random_float_range(Interval::new(0.0, 1.0));
        Color {
            r,
            g,
            b,
        }
    }

    pub fn linear_to_gamma( // corrects colors to consider gamma space alterations
        linear_component: f64
    ) -> f64 { 
        if linear_component > 0.0 {
            return linear_component.sqrt();
        }
        return 0.0;
    }
}

// Color Palette
pub struct ColorPalette {
    pub colors: Vec<Color>,
    pub color_count: u32,
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