use pixel_art_generator::{generate_pixels, GenerationMode, Pixel};

fn main() {
    let width = 16;
    let height = 16;
    
    println!("--- Pixel Art Generator Phase 1 Demo ---\n");

    // 1. Random Noise
    println!("1. Random Noise (Seed: None)");
    let grid = generate_pixels(width, height, GenerationMode::RandomNoise, None);
    print_grid(&grid);

    // 2. Checkerboard
    println!("\n2. Checkerboard");
    let grid = generate_pixels(width, height, GenerationMode::Checkerboard, None);
    print_grid(&grid);

    // 3. Symmetrical (Mirrored) with Seed
    let seed = 12345;
    println!("\n3. Symmetrical (Seed: {})", seed);
    let grid = generate_pixels(width, height, GenerationMode::Symmetrical, Some(seed));
    print_grid(&grid);

    // 4. Symmetrical (Mirrored) with Same Seed (Verification of determinism)
    println!("\n4. Symmetrical (Same Seed: {}) - Should match above", seed);
    let grid = generate_pixels(width, height, GenerationMode::Symmetrical, Some(seed));
    print_grid(&grid);
}

fn print_grid(grid: &Vec<Vec<Pixel>>) {
    // Top border
    println!("+{}+", "-".repeat(grid[0].len()));
    
    for row in grid {
        print!("|");
        for pixel in row {
            print!("{}", pixel.as_char());
        }
        println!("|");
    }
    
    // Bottom border
    println!("+{}+", "-".repeat(grid[0].len()));
}
