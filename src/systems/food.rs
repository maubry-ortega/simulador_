use crate::{components::Food, resources::Stats};
use bevy::prelude::*;
use rand::prelude::*;

pub fn spawn_food(commands: &mut Commands) {
    let mut rng = rand::rng();
    for _ in 0..20 {
        commands.spawn((
            Sprite {
                color: Color::srgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::splat(10.0)),
                ..default()
            },
            Transform::from_xyz(
                rng.random_range(-300.0..=300.0),
                rng.random_range(-200.0..=200.0),
                0.0,
            ),
            GlobalTransform::default(),
            Visibility::Visible,
            Food,
        ));
    }
}

pub fn spawn_random_food(mut commands: Commands, mut stats: ResMut<Stats>, time: Res<Time>) {
    stats.time_since_spawn += time.delta_secs();
    if stats.time_since_spawn >= 5.0 {
        stats.time_since_spawn = 0.0;
        spawn_food(&mut commands);
    }
}
