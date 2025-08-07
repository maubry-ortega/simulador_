use bevy::prelude::*;
use rand::prelude::*;
use crate::components::{Creature, Genes, Organism, Predator, State, Velocity};
use crate::utils::{color_from_generation, mutate_color};

pub fn spawn_creature(commands: &mut Commands, generation: u32) {
    let mut rng = rand::rng();

    let speed = rng.random_range(20.0..60.0);
    let size = rng.random_range(10.0..30.0);
    let color = color_from_generation(generation);
    let dir = Vec2::from_angle(rng.random_range(0.0..=std::f32::consts::TAU)) * speed;

    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::splat(size)),
            ..default()
        },
        Transform::from_xyz(
            rng.random_range(-300.0..=300.0),
            rng.random_range(-200.0..=200.0),
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

pub fn spawn_child_creature(
    commands: &mut Commands,
    position: Vec3,
    velocity: Vec2,
    parent_genes: &Genes,
    generation: u32
) {
    let mut rng = rand::rng();

    let child_genes = Genes {
        speed: (parent_genes.speed + rng.random_range(-5.0..=5.0)).clamp(10.0, 100.0),
        size: (parent_genes.size + rng.random_range(-2.0..=2.0)).clamp(5.0, 50.0),
        color: mutate_color(&parent_genes.color),
    };

    commands.spawn((
        Sprite {
            color: child_genes.color,
            custom_size: Some(Vec2::splat(child_genes.size)),
            ..default()
        },
        Transform::from_translation(position),
        GlobalTransform::default(),
        Visibility::Visible,
        Creature {
            time_since_reproduction: 0.0,
        },
        Organism {
            energy: 100.0,
            age: 0.0,
            generation,
        },
        Velocity(velocity),
        child_genes,
        State::Wandering,
    ));
}

pub fn spawn_child_predator(
    commands: &mut Commands,
    position: Vec3,
    velocity: Vec2,
    generation: u32,
) {
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::splat(40.0)),
            ..default()
        },
        Transform::from_translation(position),
        GlobalTransform::default(),
        Visibility::Visible,
        Velocity(velocity),
        Organism {
            energy: 100.0,
            age: 0.0,
            generation,
        },
        Predator {
            reproduction_cooldown: 10.0,
        },
        State::Wandering,
    ));
}

pub fn spawn_predator(commands: &mut Commands, generation: u32) {
    let mut rng = rand::rng();

    let speed = 80.0;
    let size = 40.0;
    let color = Color::srgb(1.0, 0.0, 0.0);
    let dir = Vec2::from_angle(rng.random_range(0.0..=std::f32::consts::TAU)) * speed;

    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::splat(size)),
            ..default()
        },
        Transform::from_xyz(
            rng.random_range(-300.0..=300.0),
            rng.random_range(-200.0..=200.0),
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
