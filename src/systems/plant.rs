use bevy::prelude::*;
use crate::components::{Plant, Organism, Creature};
use rand::prelude::*;

/// Spawnea algunas plantas al inicio.
pub fn spawn_initial_plants(commands: &mut Commands) {
    let mut rng = rand::rng();
    for _ in 0..30 {
        commands.spawn((
            Sprite {
                color: Color::srgb(0.1, 0.7, 0.1),
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
            Plant {
                age: 0.0,
                max_age: rng.random_range(30.0..60.0),
                size: 10.0,
                reproduction_timer: 0.0,
            },
        ));
    }
}

/// Sistema que envejece, hace crecer, reproducir y morir a las plantas.
pub fn plant_growth_and_reproduction_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Plant, &mut Sprite, &Transform)>,
) {
    let mut rng = rand::rng();
    for (entity, mut plant, mut sprite, transform) in query.iter_mut() {
        plant.age += time.delta_secs();
        plant.reproduction_timer += time.delta_secs();

        if plant.size < 15.0 {
            plant.size += 2.0 * time.delta_secs();
            sprite.custom_size = Some(Vec2::splat(plant.size));
        }

        if plant.age > plant.max_age {
            commands.entity(entity).despawn();
            continue;
        }

        if plant.reproduction_timer >= 12.0 {
            plant.reproduction_timer = 0.0;
            let offset = Vec3::new(
                rng.random_range(-50.0..=50.0),
                rng.random_range(-50.0..=50.0),
                0.0,
            );
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.1, 0.7, 0.1),
                    custom_size: Some(Vec2::splat(10.0)),
                    ..default()
                },
                Transform::from_translation(transform.translation + offset),
                GlobalTransform::default(),
                Visibility::Visible,
                Plant {
                    age: 0.0,
                    max_age: rng.random_range(30.0..60.0),
                    size: 10.0,
                    reproduction_timer: 0.0,
                },
            ));
        }
    }
}

pub fn herbivore_plant_collision_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &mut Organism), With<Creature>>,
    plants: Query<(Entity, &Transform), With<Plant>>,
) {
    for (creature_entity, creature_transform, mut organism) in query.iter_mut() {
        for (plant_entity, plant_transform) in plants.iter() {
            let distance = creature_transform
                .translation
                .truncate()
                .distance(plant_transform.translation.truncate());

            if distance < 10.0 {
                organism.energy += 40.0;
                commands.entity(plant_entity).despawn();
                break;
            }
        }
    }
}
