use crate::{
    components::{Creature, Genes, Organism, Predator, State, Velocity},
    resources::Stats,
    utils::factory::{spawn_child_creature, spawn_child_predator},
};
use bevy::prelude::*;
use rand::prelude::*;

/// Las criaturas se reproducen si tienen suficiente energía y tiempo desde la última reproducción.
pub fn reproduction_system(
    mut commands: Commands,
    mut stats: ResMut<Stats>,
    mut query: Query<(&mut Creature, &mut Organism, &Transform, &Velocity, &Genes)>,
) {
    let mut rng = rand::rng();

    for (mut creature, mut organism, transform, velocity, genes) in query.iter_mut() {
        if organism.energy > 120.0 && creature.time_since_reproduction > 5.0 {
            organism.energy -= 40.0;
            creature.time_since_reproduction = 0.0;

            let child_gen = organism.generation + 1;
            let vx = velocity.0.x + rng.random_range(-5.0..=5.0);
            let vy = velocity.0.y + rng.random_range(-5.0..=5.0);

            spawn_child_creature(
                &mut commands,
                transform.translation + Vec3::new(10.0, 10.0, 0.0),
                Vec2::new(vx, vy),
                genes,
                child_gen,
            );

            stats.total_reproductions += 1;
            stats.max_generation = stats.max_generation.max(child_gen);
        }
    }
}

/// Los depredadores se reproducen si hay al menos dos, están en temporada, y su cooldown terminó.
pub fn predator_reproduction_system(
    mut commands: Commands,
    mut query: Query<(&Transform, &Velocity, &mut Predator, &mut Organism, &State)>,
) {
    let predators: Vec<_> = query.iter_mut().collect();
    let mut spawned = 0;

    if predators.len() >= 2 {
        for (transform, velocity, mut predator, organism, state) in predators {
            if *state == State::ReproducingSeason
                && predator.reproduction_cooldown <= 0.0
                && spawned < 1
            {
                let new_gen = organism.generation + 1;

                spawn_child_predator(
                    &mut commands,
                    transform.translation + Vec3::new(10.0, 10.0, 0.0),
                    velocity.0,
                    new_gen,
                );

                predator.reproduction_cooldown = 10.0;
                spawned += 1;
            }
        }
    }
}

/// Disminuye el cooldown de reproducción con el tiempo.
pub fn update_predator_cooldowns(time: Res<Time>, mut query: Query<&mut Predator>) {
    for mut predator in query.iter_mut() {
        predator.reproduction_cooldown -= time.delta_secs();
    }
}

/// Decide si los depredadores están en temporada de reproducción.
pub fn update_predator_states(mut query: Query<(&mut State, &Organism), With<Predator>>) {
    for (mut state, organism) in query.iter_mut() {
        let new_state = if organism.energy > 110.0 {
            State::ReproducingSeason
        } else {
            State::Wandering
        };

        *state = new_state;
    }
}

/// Spawnea criaturas iniciales al comenzar la simulación.
pub fn spawn_initial_creatures(commands: &mut Commands) {
    use crate::utils::factory::spawn_creature;

    for _ in 0..10 {
        spawn_creature(commands, 0);
    }
}

/// Spawnea depredadores iniciales.
pub fn spawn_initial_predators(commands: &mut Commands) {
    use crate::utils::factory::spawn_predator;

    for _ in 0..2 {
        spawn_predator(commands, 0);
    }
}
