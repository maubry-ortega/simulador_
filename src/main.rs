mod components;
mod resources;
mod systems;
mod utils;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, log::LogPlugin, prelude::*};
use resources::Stats;

fn main() {
    App::new()
        .insert_resource(Stats::default())
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                level: bevy::log::Level::INFO,
                filter: "wgpu=error,bevy_render=info".into(),
                custom_layer: |_| None,
            }),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, systems::setup)
        .add_systems(
            Update,
            (
                // Movimiento y colisiones
                systems::move_entities,
                systems::avoid_entity_overlap_system,
                systems::boundary_bounce_system,

                // IA y comportamiento
                systems::seek_food_system,
                systems::update_states,
                systems::avoid_predators_system,

                // Plantas y alimentación
                systems::plant_growth_and_reproduction_system,
                systems::herbivore_plant_collision_system,

                // Depredadores
                systems::predator_hunting_system,
                systems::update_predator_cooldowns,
                systems::update_predator_states,

                // Reproducción
                systems::reproduction_system,
                systems::predator_reproduction_system,

                // HUD
                systems::update_hud,
                systems::update_fps,
            ),
        )
        .run();
}
