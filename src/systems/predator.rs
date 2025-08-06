use crate::components::{Creature, Genes, Predator, State, Velocity};
use bevy::prelude::*;

pub fn predator_hunting_system(
    mut commands: Commands,
    mut predators: Query<(&Transform, &mut Predator, &mut Velocity, &State)>,
    creatures: Query<(Entity, &Transform, &Genes), With<Creature>>,
) {
    for (pred_transform, mut predator, mut velocity, state) in predators.iter_mut() {
        if *state != State::ReproducingSeason || predator.energy >= 80.0 {
            continue; // No caza si no tiene hambre
        }

        if let Some((closest_entity, closest_transform, _genes)) =
            creatures.iter().min_by(|(_, a, _), (_, b, _)| {
                let da = pred_transform
                    .translation
                    .truncate()
                    .distance_squared(a.translation.truncate());
                let db = pred_transform
                    .translation
                    .truncate()
                    .distance_squared(b.translation.truncate());
                da.total_cmp(&db)
            })
        {
            let direction = (closest_transform.translation - pred_transform.translation)
                .truncate()
                .normalize_or_zero();
            velocity.0 = direction * 90.0; // velocidad alta del depredador

            let distance = pred_transform
                .translation
                .truncate()
                .distance(closest_transform.translation.truncate());

            if distance < 25.0 {
                commands.entity(closest_entity).despawn();
                predator.energy += 40.0;
            }
        }
    }
}

pub fn predator_seek_prey_system(
    mut predators: Query<(&Transform, &mut Velocity, &Predator), With<Predator>>,
    creatures: Query<&Transform, With<Creature>>,
) {
    for (pred_transform, mut velocity, predator) in predators.iter_mut() {
        if predator.energy > 80.0 {
            continue; // No tiene hambre
        }

        if let Some(closest) = creatures
            .iter()
            .filter(|c_transform| {
                pred_transform
                    .translation
                    .truncate()
                    .distance(c_transform.translation.truncate())
                    < 150.0
            })
            .min_by(|a, b| {
                let dist_a = a
                    .translation
                    .truncate()
                    .distance_squared(pred_transform.translation.truncate());
                let dist_b = b
                    .translation
                    .truncate()
                    .distance_squared(pred_transform.translation.truncate());
                dist_a.total_cmp(&dist_b)
            })
        {
            let dir = (closest.translation - pred_transform.translation)
                .truncate()
                .normalize_or_zero();
            velocity.0 = dir * 90.0; // velocidad más alta que criaturas
        }
    }
}
