use mandlebrot_set::color::{Color, ColorPalette};
use mandlebrot_set::{ImageDim, MandlebrotImage};

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
