use crate::{
    components::{Creature, Genes, Organism, Predator, State, Velocity},
    resources::Stats,
    utils::mutate_color,
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

            let child_genes = Genes {
                speed: (genes.speed + rng.random_range(-5.0..=5.0)).clamp(10.0, 100.0),
                size: (genes.size + rng.random_range(-2.0..=2.0)).clamp(5.0, 50.0),
                color: mutate_color(&genes.color),
            };

            commands.spawn((
                Sprite {
                    color: child_genes.color,
                    custom_size: Some(Vec2::splat(child_genes.size)),
                    ..default()
                },
                Transform::from_translation(transform.translation + Vec3::new(10.0, 10.0, 0.0)),
                GlobalTransform::default(),
                Visibility::Visible,
                Creature {
                    time_since_reproduction: 0.0,
                },
                Organism {
                    energy: 100.0,
                    age: 0.0,
                    generation: child_gen,
                },
                Velocity(Vec2::new(vx, vy)),
                child_genes,
                State::Wandering,
            ));

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

                commands.spawn((
                    Sprite {
                        color: Color::srgb(1.0, 0.0, 0.0),
                        custom_size: Some(Vec2::splat(40.0)),
                        ..default()
                    },
                    Transform::from_translation(transform.translation + Vec3::new(10.0, 10.0, 0.0)),
                    GlobalTransform::default(),
                    Visibility::Visible,
                    Predator {
                        reproduction_cooldown: 10.0,
                    },
                    Organism {
                        energy: 100.0,
                        age: 0.0,
                        generation: new_gen,
                    },
                    Velocity(velocity.0),
                    State::Wandering,
                ));

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
    let mut rng = rand::rng();
    for _ in 0..10 {
        commands.spawn((
            Sprite {
                color: Color::srgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::splat(20.0)),
                ..default()
            },
            Transform::from_xyz(
                rng.random_range(-400.0..400.0),
                rng.random_range(-300.0..300.0),
                0.0,
            ),
            GlobalTransform::default(),
            Visibility::Visible,
            Creature {
                time_since_reproduction: 0.0,
            },
            Organism {
                energy: 100.0,
                age: 0.0,
                generation: 0,
            },
            Velocity(Vec2::new(
                rng.random_range(-30.0..30.0),
                rng.random_range(-30.0..30.0),
            )),
            Genes {
                speed: 50.0,
                size: 20.0,
                color: Color::srgb(0.0, 1.0, 0.0),
            },
            State::Wandering,
        ));
    }
}

/// Spawnea depredadores iniciales.
pub fn spawn_initial_predators(commands: &mut Commands) {
    let mut rng = rand::rng();
    for _ in 0..2 {
        commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::splat(40.0)),
                ..default()
            },
            Transform::from_xyz(
                rng.random_range(-400.0..400.0),
                rng.random_range(-300.0..300.0),
                0.0,
            ),
            GlobalTransform::default(),
            Visibility::Visible,
            Predator {
                reproduction_cooldown: 5.0,
            },
            Organism {
                energy: 100.0,
                age: 0.0,
                generation: 0,
            },
            Velocity(Vec2::new(
                rng.random_range(-50.0..50.0),
                rng.random_range(-50.0..50.0),
            )),
            State::Wandering,
        ));
    }
}
