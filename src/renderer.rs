use crate::Pixel;
use image::{ImageBuffer, Rgb, RgbImage};

/// Renders the pixel grid as ASCII art to the standard output.
///
/// This function iterates through the provided grid and prints each pixel
/// using the standard ASCII mapping:
/// - Filled: '█'
/// - Empty: ' '
///
/// It also draws a simple border around the grid.
///
/// # Arguments
///
/// * `grid` - A slice of vectors representing the 2D pixel grid.
pub fn render_ascii(grid: &[Vec<Pixel>]) {
    if grid.is_empty() {
        return;
    }

    let width = grid[0].len();

    // Top border
    println!("+{}+", "-".repeat(width));

    for row in grid {
        print!("|");
        for pixel in row {
            match pixel {
                Pixel::Filled => print!("█"),
                Pixel::Empty => print!(" "),
            }
        }
        println!("|");
    }

    // Bottom border
    println!("+{}+", "-".repeat(width));
}

/// Renders the pixel grid as a PNG image file.
///
/// Each pixel in the grid is scaled to `scale` x `scale` pixels in the output image.
/// - Filled pixels are black.
/// - Empty pixels are white (used as background).
///
/// # Arguments
///
/// * `grid` - A slice of vectors representing the 2D pixel grid.
/// * `scale` - The scaling factor (pixels per grid cell side).
/// * `output_path` - The path where the PNG file should be saved.
pub fn render_png(grid: &[Vec<Pixel>], scale: u32, output_path: &str) {
    if grid.is_empty() {
        return;
    }

    let height = grid.len() as u32;
    let width = grid[0].len() as u32;

    let img_width = width * scale;
    let img_height = height * scale;

    let mut img: RgbImage = ImageBuffer::new(img_width, img_height);

    // Iterate over the image pixels and map them to grid coordinates
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        // Determine which grid cell this image pixel belongs to
        let grid_x = (x / scale) as usize;
        let grid_y = (y / scale) as usize;

        // Get the pixel state from the grid
        let color = match grid[grid_y][grid_x] {
            Pixel::Filled => Rgb([0, 0, 0]),       // Black
            Pixel::Empty => Rgb([255, 255, 255]), // White
        };

        *pixel = color;
    }

    // Save the image
    if let Err(e) = img.save(output_path) {
        eprintln!("Error saving image to {}: {}", output_path, e);
    } else {
        println!("Saved image to {}", output_path);
    }
}
