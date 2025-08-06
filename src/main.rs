mod components;
mod resources;
mod utils;
mod systems;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use resources::Stats;

fn main() {
    App::new()
        .insert_resource(Stats::default())
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin::default()))
        .add_systems(Startup, systems::setup)
        .add_systems(Update, (
            systems::move_creatures,
            systems::food_collision_system,
            systems::reproduction_system,
            systems::boundary_bounce_system,
            systems::spawn_random_food,
            systems::update_hud,
            systems::update_fps,
            systems::update_states,
            systems::seek_food_system,
        ))
        .run();
}
