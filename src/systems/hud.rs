use crate::{
    components::{Creature, FpsText, Organism, Predator, Plant},
    resources::Stats,
};
use bevy::prelude::*;

/// Actualiza el texto del HUD con estadísticas vivas del ecosistema.
pub fn update_hud(
    mut stats: ResMut<Stats>,
    time: Res<Time>,
    creatures: Query<(&Organism, &Creature)>,
    predators: Query<&Predator>,
    plants: Query<(), With<Plant>>,
    mut texts: Query<&mut Text>,
) {
    // Actualizar tiempo de simulación
    stats.simulation_time += time.delta_secs();

    if let Some(mut text) = texts.iter_mut().last() {
        let total_creatures = creatures.iter().count();
        let total_predators = predators.iter().count();
        let plant_count = plants.iter().count();

        let avg_gen = if total_creatures > 0 {
            creatures.iter().map(|(org, _)| org.generation).sum::<u32>() as f32
                / total_creatures as f32
        } else {
            0.0
        };

        *text = Text::new(format!(
            "🧬 Criaturas: {}\n\
             🦊 Depredadores: {}\n\
             🌿 Plantas: {}\n\
             🔁 Reproducciones: {}\n\
             💀 Muertes: {}\n\
             📈 Máx Gen: {}\n\
             📊 Prom Gen: {:.1}\n\
             ⏱️ Tiempo: {:.1}s",
            total_creatures,
            total_predators,
            plant_count,
            stats.total_reproductions,
            stats.total_deaths,
            stats.max_generation,
            avg_gen,
            stats.simulation_time
        ));
    }
}

/// Actualiza el texto de FPS visible en pantalla.
pub fn update_fps(
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
    mut query: Query<&mut TextSpan, With<FpsText>>,
) {
    for mut span in &mut query {
        if let Some(fps) = diagnostics.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                **span = format!("{value:.2}");
            }
        }
    }
}
