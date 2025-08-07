use crate::components::{Creature, Organism, Predator, Velocity};
use crate::resources::Stats;
use bevy::prelude::*;

/// Mueve criaturas y depredadores, aplica consumo de energía, envejecimiento y muerte.
pub fn move_entities(
    time: Res<Time>,
    mut commands: Commands,
    mut stats: ResMut<Stats>,
    mut params: ParamSet<(
        Query<(Entity, &Velocity, &mut Transform, &mut Organism, &mut Creature)>,
        Query<(Entity, &Velocity, &mut Transform, &mut Organism, &mut Predator)>,
    )>,
) {
    // 🟢 Movimiento y lógica para criaturas (herbívoras)
    for (entity, velocity, mut transform, mut organism, mut creature) in params.p0().iter_mut() {
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();

        organism.energy -= 1.0 * time.delta_secs();
        organism.age += time.delta_secs();
        creature.time_since_reproduction += time.delta_secs();

        // Muerte por edad o agotamiento
        if organism.energy <= 0.0 || organism.age > 60.0 {
            commands.entity(entity).despawn();
            stats.total_deaths += 1;
        }
    }

    // 🔴 Movimiento y lógica para depredadores
    for (entity, velocity, mut transform, mut organism, _predator) in params.p1().iter_mut() {
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();

        organism.energy -= 0.8 * time.delta_secs();
        organism.age += time.delta_secs();

        // Muerte por agotamiento
        if organism.energy <= 0.0 {
            commands.entity(entity).despawn();
            stats.total_deaths += 1;
        }

        // Nota: cooldown de reproducción se actualiza en otro sistema
        // o puedes añadirlo aquí si se necesita
    }
}
