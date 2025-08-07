use crate::components::{Food, Organism, Predator};
use crate::resources::Stats;
use bevy::prelude::*;
use rand::prelude::*;

/// Spawnea 20 unidades de comida en ubicaciones aleatorias
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

/// Genera comida nueva cada 5 segundos
pub fn spawn_random_food(mut commands: Commands, mut stats: ResMut<Stats>, time: Res<Time>) {
    stats.time_since_spawn += time.delta_secs();
    if stats.time_since_spawn >= 5.0 {
        stats.time_since_spawn = 0.0;
        spawn_food(&mut commands);
    }
}

/// Sistema de colisión para que criaturas herbívoras coman comida
/// Solo afecta a entidades con `Organism` y que NO tengan `Predator`
pub fn food_collision_system(
    mut commands: Commands,
    mut organisms: Query<(&mut Organism, &Transform), Without<Predator>>,
    foods: Query<(Entity, &Transform), With<Food>>,
) {
    for (mut organism, transform) in organisms.iter_mut() {
        for (food_entity, food_transform) in foods.iter() {
            let distance = transform
                .translation
                .truncate()
                .distance(food_transform.translation.truncate());

            if distance < 15.0 {
                organism.energy = (organism.energy + 30.0).min(150.0);
                commands.entity(food_entity).despawn();
                break; // Solo una comida por frame
            }
        }
    }
}
