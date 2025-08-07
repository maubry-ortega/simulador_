use bevy::prelude::*;

/// Estadísticas generales del simulador
#[derive(Resource, Default)]
pub struct Stats {
    pub total_reproductions: usize,
    pub total_deaths: usize,
    pub max_generation: u32,
    pub simulation_time: f32,
}
