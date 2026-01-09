use pixel_art_generator::{generate_pixels, renderer, GenerationMode};

fn main() {
    let width = 16;
    let height = 16;
    
    println!("--- Pixel Art Generator Phase 3 Demo ---\n");

    // 1. Random Noise
    println!("1. Random Noise (Seed: None)");
    let grid = generate_pixels(width, height, GenerationMode::RandomNoise, None);
    renderer::render_ascii(&grid);

    // 2. Checkerboard
    println!("\n2. Checkerboard");
    let grid = generate_pixels(width, height, GenerationMode::Checkerboard, None);
    renderer::render_ascii(&grid);

    // 3. Symmetrical (Mirrored) with Seed
    let seed = 12345;
    println!("\n3. Symmetrical (Seed: {})", seed);
    let grid = generate_pixels(width, height, GenerationMode::Symmetrical, Some(seed));
    renderer::render_ascii(&grid);
    
    // Save Symmetrical pattern to PNG
    let png_path = "output.png";
    let scale = 20;
    println!("Saving Symmetrical pattern to '{}' with scale {}...", png_path, scale);
    renderer::render_png(&grid, scale, png_path);

    // 4. Symmetrical (Mirrored) with Same Seed (Verification of determinism)
    println!("\n4. Symmetrical (Same Seed: {}) - Should match above", seed);
    let grid = generate_pixels(width, height, GenerationMode::Symmetrical, Some(seed));
    renderer::render_ascii(&grid);
}
