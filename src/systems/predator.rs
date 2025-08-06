use bevy::prelude::*;
use crate::components::{Predator, Creature};

pub fn predator_hunting_system(
    mut commands: Commands,
    mut predators: Query<(&Transform, &mut Predator)>,
    creatures: Query<(Entity, &Transform), With<Creature>>,
) {
    for (pred_transform, mut predator) in predators.iter_mut() {
        for (creature_entity, creature_transform) in creatures.iter() {
            let distance = pred_transform.translation.truncate()
                .distance(creature_transform.translation.truncate());

            if distance < 20.0 {
                // Comer criatura
                commands.entity(creature_entity).despawn();
                predator.energy += 40.0;
                break; // Solo una presa por frame
            }
        }
    }
}
