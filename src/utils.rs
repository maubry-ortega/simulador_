use bevy::prelude::*;
use rand::prelude::*;

/// Asigna un color HSL a una generación
pub fn color_from_generation(r#gen: u32) -> Color {
    let hue = (r#gen as f32 * 40.0) % 360.0;
    Color::hsl(hue, 0.8, 0.6)
}

/// Mutación simple de color (generación aleatoria cercana en HSL)
pub fn mutate_color(_original: &Color) -> Color {
    let mut rng = rand::rng();

    // Tomamos una base aleatoria cercana en HSL
    let hue = rng.random_range(0.0..=360.0);
    let sat = rng.random_range(0.7..=0.9);
    let light = rng.random_range(0.5..=0.7);

    Color::hsl(hue, sat, light)
}
