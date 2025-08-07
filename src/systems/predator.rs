use crate::components::{Creature, Genes, Organism, Predator, State, Velocity};
use crate::resources::Stats;
use bevy::prelude::*;

/// Sistema de caza: los depredadores buscan presas si tienen hambre y están en temporada de reproducción.
pub fn predator_hunting_system(
    mut commands: Commands,
    mut stats: ResMut<Stats>,
    mut predators: Query<(
        &Transform,
        &mut Velocity,
        &mut Organism,
        &Predator,
        &State,
    )>,
    creatures: Query<(Entity, &Transform, &Genes), With<Creature>>,
) {
    for (pred_transform, mut velocity, mut predator_org, _predator, state) in predators.iter_mut() {
        // Solo cazan si tienen hambre y están en temporada de reproducción
        if *state != State::ReproducingSeason || predator_org.energy >= 80.0 {
            continue;
        }

        // Busca la criatura más cercana dentro del rango
        if let Some((closest_entity, closest_transform, _genes)) = creatures
            .iter()
            .filter(|(_, transform, _)| {
                pred_transform
                    .translation
                    .truncate()
                    .distance(transform.translation.truncate())
                    < 150.0
            })
            .min_by(|(_, a, _), (_, b, _)| {
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
            // Dirección hacia la presa
            let direction = (closest_transform.translation - pred_transform.translation)
                .truncate()
                .normalize_or_zero();
            velocity.0 = direction * 90.0;

            // Si está lo suficientemente cerca, devora a la presa
            let distance = pred_transform
                .translation
                .truncate()
                .distance(closest_transform.translation.truncate());

            if distance < 25.0 {
                commands.entity(closest_entity).despawn();
                predator_org.energy = (predator_org.energy + 40.0).min(150.0); // Recupera energía
                stats.total_deaths += 1;
            }
        } else {
            // No hay presas cerca: queda inmóvil
            velocity.0 = Vec2::ZERO;
        }
    }
}
