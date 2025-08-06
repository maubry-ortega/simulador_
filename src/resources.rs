use bevy::prelude::*;

/// Estadísticas generales del simulador
#[derive(Resource, Default)]
pub struct Stats {
    pub total_reproductions: usize,
    pub time_since_spawn: f32,
    pub max_generation: u32,
}
