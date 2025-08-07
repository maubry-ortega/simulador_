use bevy::prelude::*;
use rand::prelude::*;
use crate::components::*;

pub fn spawn_creature(commands: &mut Commands, generation: u32) {
    let mut rng = rand::rng();

    let speed = rng.gen_range(20.0..60.0);
    let size = rng.gen_range(10.0..30.0);
    let color = color_from_generation(generation);
    let dir = Vec2::from_angle(rng.gen_range(0.0..=std::f32::consts::TAU)) * speed;

    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::splat(size)),
            ..default()
        },
        Transform::from_xyz(
            rng.gen_range(-300.0..=300.0),
            rng.gen_range(-200.0..=200.0),
            0.0,
        ),
        GlobalTransform::default(),
        Visibility::Visible,
        Velocity(dir),
        Organism {
            energy: 100.0,
            age: 0.0,
            generation,
        },
        Creature {
            time_since_reproduction: 0.0,
        },
        Genes { speed, size, color },
        State::Wandering,
    ));
}

pub fn spawn_predator(commands: &mut Commands, generation: u32) {
    let mut rng = rand::rng();

    let speed = 80.0;
    let size = 40.0;
    let color = Color::RED;
    let dir = Vec2::from_angle(rng.gen_range(0.0..=std::f32::consts::TAU)) * speed;

    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::splat(size)),
            ..default()
        },
        Transform::from_xyz(
            rng.gen_range(-300.0..=300.0),
            rng.gen_range(-200.0..=200.0),
            0.0,
        ),
        GlobalTransform::default(),
        Visibility::Visible,
        Velocity(dir),
        Organism {
            energy: 100.0,
            age: 0.0,
            generation,
        },
        Predator {
            reproduction_cooldown: 0.0,
        },
        State::ReproducingSeason,
    ));
}
