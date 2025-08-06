use crate::components::{Creature, Predator, Velocity};
use bevy::prelude::*;

pub fn move_entities(
    time: Res<Time>,
    mut commands: Commands,
    mut params: ParamSet<(
        Query<(Entity, &Velocity, &mut Transform, &mut Creature)>,
        Query<(Entity, &Velocity, &mut Transform, &mut Predator)>,
    )>,
) {
    // Movimiento de criaturas
    for (entity, velocity, mut transform, mut creature) in params.p0().iter_mut() {
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
        creature.energy -= 0.1 * time.delta_secs();
        creature.age += time.delta_secs();
        creature.time_since_reproduction += time.delta_secs();

        if creature.energy <= 0.0 || creature.age > 60.0 {
            commands.entity(entity).despawn();
        }
    }

    // Movimiento de depredadores
    for (entity, velocity, mut transform, mut predator) in params.p1().iter_mut() {
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
        predator.energy -= 0.08 * time.delta_secs();
        predator.reproduction_cooldown += time.delta_secs();

        if predator.energy <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
