use crate::{
    components::FpsText,
    systems::{
        plant::spawn_initial_plants,
        reproduction::{spawn_initial_creatures, spawn_initial_predators},
    },
};
use bevy::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Cámara
    commands.spawn(Camera2d);

    // FPS Text
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

    // HUD general
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

    // Spawns iniciales
    spawn_initial_creatures(&mut commands);
    spawn_initial_predators(&mut commands);
    spawn_initial_plants(&mut commands);
}
