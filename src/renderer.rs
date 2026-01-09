use crate::Pixel;

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
