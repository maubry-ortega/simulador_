use bevy::prelude::*;
use crate::components::{Creature, Velocity};

pub fn move_creatures(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &Velocity, &mut Transform, &mut Creature)>,
) {
    for (entity, velocity, mut transform, mut creature) in query.iter_mut() {
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
        creature.energy -= 0.1 * time.delta_secs();
        creature.age += time.delta_secs();
        creature.time_since_reproduction += time.delta_secs();

        if creature.energy <= 0.0 || creature.age > 60.0 {
            commands.entity(entity).despawn();
        }
    }
}
