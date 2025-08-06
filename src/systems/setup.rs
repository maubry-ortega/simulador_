use crate::{
    components::{Creature, FpsText, Genes, Predator, State, Velocity},
    systems::food::spawn_food,
    utils::color_from_generation,
};
use bevy::prelude::*;
use rand::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // HUD
    commands
        .spawn((
            Text::new("FPS: "),
            TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                ..default()
            },
        ))
        .with_child((
            TextSpan::from(""),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            FpsText,
        ));

    commands.spawn((
        Text::new(""),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 20.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));

    spawn_initial_creatures(&mut commands);
    spawn_initial_predators(&mut commands);
    spawn_food(&mut commands);
}

pub fn spawn_initial_creatures(commands: &mut Commands) {
    let mut rng = rand::rng();

    for _ in 0..10 {
        let angle = rng.random_range(0.0..=std::f32::consts::TAU);
        let speed = rng.random_range(20.0..=60.0);
        let size = rng.random_range(10.0..=30.0);
        let color = color_from_generation(0);
        let dir = Vec2::from_angle(angle) * speed;

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
            Creature {
                energy: 100.0,
                age: 0.0,
                time_since_reproduction: 0.0,
                generation: 0,
            },
            Velocity(dir),
            Genes { speed, size, color },
            State::Wandering,
        ));
    }
}

pub fn spawn_initial_predators(commands: &mut Commands) {
    let mut rng = rand::rng();

    for _ in 0..2 {
        let angle = rng.random_range(0.0..=std::f32::consts::TAU);
        let speed = 90.0;
        let dir = Vec2::from_angle(angle) * speed;

        commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::splat(40.0)),
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
            Predator {
                energy: 100.0,
                reproduction_cooldown: 0.0,
                generation: 0,
            },
            State::ReproducingSeason,
        ));
    }
}
