use crate::components::{Creature, Plant, Organism, Predator, State, Velocity};
use bevy::prelude::*;

/// Sistema que decide en qué estado está cada criatura (comer, reproducirse o vagar)
pub fn update_states(
    mut query: Query<(&Transform, &mut State, &Creature, &Organism)>,
) {
    for (transform, mut state, creature, organism) in query.iter_mut() {
        let new_state = if organism.energy < 50.0 {
            State::SeekingFood
        } else if organism.energy > 120.0 && creature.time_since_reproduction > 5.0 {
            State::Reproducing
        } else {
            State::Wandering
        };

        if *state != new_state {
            info!(
                "🧠 Cambio de estado en criatura ({:?}): {:?} -> {:?}",
                transform.translation, *state, new_state
            );
            *state = new_state;
        }
    }
}

/// Sistema que mueve criaturas hambrientas hacia la comida más cercana
pub fn seek_food_system(
    mut creatures: Query<(&Transform, &mut Velocity, &State), With<Creature>>,
    plants: Query<&Transform, With<Plant>>,
) {
    for (creature_transform, mut velocity, state) in creatures.iter_mut() {
        if *state != State::SeekingFood {
            continue;
        }

        if let Some(closest_plant) = plants.iter().min_by(|a, b| {
            let pos = creature_transform.translation.truncate();
            let da = a.translation.truncate().distance_squared(pos);
            let db = b.translation.truncate().distance_squared(pos);
            da.total_cmp(&db)
        }) {
            let direction = (closest_plant.translation - creature_transform.translation)
                .truncate()
                .normalize_or_zero();
            velocity.0 = direction * 50.0;
        }
    }
}

/// Sistema que hace que las criaturas huyan de depredadores cercanos
pub fn avoid_predators_system(
    predators: Query<&Transform, With<Predator>>,
    mut creatures: Query<(&Transform, &mut Velocity, &State), With<Creature>>,
) {
    for (creature_transform, mut velocity, state) in creatures.iter_mut() {
        if *state != State::Wandering && *state != State::SeekingFood {
            continue;
        }

        if let Some(closest_predator) = predators.iter().min_by(|a, b| {
            let pos = creature_transform.translation.truncate();
            let da = a.translation.truncate().distance_squared(pos);
            let db = b.translation.truncate().distance_squared(pos);
            da.total_cmp(&db)
        }) {
            let distance = creature_transform
                .translation
                .truncate()
                .distance(closest_predator.translation.truncate());

            if distance < 100.0 {
                // Huir en dirección opuesta
                let direction = (creature_transform.translation - closest_predator.translation)
                    .truncate()
                    .normalize_or_zero();
                velocity.0 = direction * 80.0;
            }
        }
    }
}
