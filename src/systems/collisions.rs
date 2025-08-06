use bevy::prelude::*;
use crate::components::{Creature, Food};

pub fn food_collision_system(
    mut commands: Commands,
    mut creature_query: Query<(&mut Creature, &Transform)>,
    food_query: Query<(Entity, &Transform), With<Food>>,
) {
    for (mut creature, creature_transform) in creature_query.iter_mut() {
        for (food_entity, food_transform) in food_query.iter() {
            let dist = creature_transform.translation.truncate().distance(food_transform.translation.truncate());
            if dist < 15.0 {
                creature.energy += 20.0;
                let _ = commands.entity(food_entity).despawn();
                break;
            }
        }
    }
}
