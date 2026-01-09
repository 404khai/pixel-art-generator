use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Represents a single pixel in the grid.
/// Currently supports basic binary states: Empty or Filled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pixel {
    Empty,
    Filled,
}

impl Pixel {
    /// Returns a character representation of the pixel for ASCII output.
    pub fn as_char(&self) -> char {
        match self {
            Pixel::Empty => ' ',
            Pixel::Filled => '█',
        }
    }
}

/// Defines the algorithm used to generate the pixel pattern.
#[derive(Debug, Clone, Copy)]
pub enum GenerationMode {
    /// Randomly fills pixels with a 50% chance.
    RandomNoise,
    /// Generates a pattern that is mirrored horizontally.
    Symmetrical,
    /// Creates a classic checkerboard pattern.
    Checkerboard,
}

/// Generates a 2D grid of pixels based on the specified mode and parameters.
///
/// # Arguments
///
/// * `width` - The width of the grid.
/// * `height` - The height of the grid.
/// * `mode` - The generation algorithm to use.
/// * `seed` - Optional seed for deterministic generation. If None, a random seed is used.
///
/// # Returns
///
/// A vector of vectors representing the grid (rows of pixels).
pub fn generate_pixels(
    width: usize,
    height: usize,
    mode: GenerationMode,
    seed: Option<u64>,
) -> Vec<Vec<Pixel>> {
    // Initialize the random number generator.
    // If a seed is provided, use it to create a deterministic RNG (ChaCha8).
    // Otherwise, use entropy from the system.
    let mut rng = match seed {
        Some(s) => ChaCha8Rng::seed_from_u64(s),
        None => ChaCha8Rng::from_os_rng(),
    };

    let mut grid = Vec::with_capacity(height);

    match mode {
        GenerationMode::RandomNoise => {
            for _y in 0..height {
                let mut row = Vec::with_capacity(width);
                for _x in 0..width {
                    // 50% chance of being filled
                    let pixel = if rng.random_bool(0.5) {
                        Pixel::Filled
                    } else {
                        Pixel::Empty
                    };
                    row.push(pixel);
                }
                grid.push(row);
            }
        }
        GenerationMode::Checkerboard => {
            for y in 0..height {
                let mut row = Vec::with_capacity(width);
                for x in 0..width {
                    // Fill if x + y is even (standard checkerboard)
                    let pixel = if (x + y) % 2 == 0 {
                        Pixel::Filled
                    } else {
                        Pixel::Empty
                    };
                    row.push(pixel);
                }
                grid.push(row);
            }
        }
        GenerationMode::Symmetrical => {
            for _y in 0..height {
                let mut row = vec![Pixel::Empty; width];
                
                // Calculate the midpoint. For odd widths, the middle column is unique.
                let midpoint = (width + 1) / 2;

                for x in 0..midpoint {
                    // Generate pixel for the left side
                    let pixel = if rng.random_bool(0.5) {
                        Pixel::Filled
                    } else {
                        Pixel::Empty
                    };
                    
                    // Set left side
                    row[x] = pixel;
                    
                    // Set mirrored right side
                    // width - 1 - x gives the corresponding index from the right
                    row[width - 1 - x] = pixel;
                }
                grid.push(row);
            }
        }
    }

    grid
}
