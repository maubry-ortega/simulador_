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
                systems::move_entities,
                systems::food_collision_system,
                systems::reproduction_system,
                systems::predator_reproduction_system,
                systems::boundary_bounce_system,
                systems::spawn_random_food,
                systems::predator_hunting_system,
                systems::predator_seek_prey_system,
                systems::update_hud,
                systems::update_fps,
                systems::update_states,
                systems::seek_food_system,
                systems::avoid_predators_system,
            ),
        )
        .run();
}
