use bevy::prelude::*;
use crate::components::{Creature, State, Velocity, Food};

pub fn update_states(
    mut query: Query<(&Transform, &mut State, &Creature)>,
    _food_query: Query<&Transform, With<Food>>,
) {
    for (transform, mut state, creature) in query.iter_mut() {
        let new_state = if creature.energy < 50.0 {
            State::SeekingFood
        } else if creature.energy > 120.0 && creature.time_since_reproduction > 5.0 {
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

pub fn seek_food_system(
    mut creatures: Query<(&Transform, &mut Velocity, &State), With<Creature>>,
    foods: Query<&Transform, With<Food>>,
) {
    for (creature_transform, mut velocity, state) in creatures.iter_mut() {
        if *state != State::SeekingFood {
            continue;
        }

        if let Some(closest_food) = foods.iter()
            .min_by(|a, b| {
                let dist_a = a.translation.truncate().distance_squared(creature_transform.translation.truncate());
                let dist_b = b.translation.truncate().distance_squared(creature_transform.translation.truncate());
                dist_a.total_cmp(&dist_b)
            }) {

            let direction = (closest_food.translation - creature_transform.translation).truncate().normalize_or_zero();
            velocity.0 = direction * 50.0;
        }
    }
}
