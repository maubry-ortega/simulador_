use bevy::prelude::*;
use crate::{components::{Creature, Food, FpsText}, resources::Stats};

pub fn update_hud(
    stats: Res<Stats>,
    creatures: Query<&Creature>,
    foods: Query<(), With<Food>>,
    mut texts: Query<&mut Text>,
) {
    let mut hud_text = texts.iter_mut().last().unwrap();
    let total = creatures.iter().count();
    let avg_gen = if total > 0 {
        creatures.iter().map(|c| c.generation).sum::<u32>() as f32 / total as f32
    } else {
        0.0
    };
    *hud_text = Text::new(format!(
        "Criaturas: {}\nComida: {}\nReproducciones: {}\nMax Gen: {}\nAvg Gen: {:.1}",
        total,
        foods.iter().count(),
        stats.total_reproductions,
        stats.max_generation,
        avg_gen
    ));
}

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
