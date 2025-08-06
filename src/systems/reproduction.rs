use bevy::prelude::*;
use rand::prelude::*;
use crate::{
    components::{Creature, Velocity, Genes, State},
    utils::mutate_color,
    resources::Stats,
};

pub fn reproduction_system(
    mut commands: Commands,
    mut stats: ResMut<Stats>,
    mut query: Query<(&mut Creature, &Transform, &Velocity, &Genes)>,
) {
    let mut rng = rand::rng();

    for (mut creature, transform, velocity, genes) in query.iter_mut() {
        if creature.energy > 120.0 && creature.time_since_reproduction > 5.0 {
            creature.energy -= 40.0;
            creature.time_since_reproduction = 0.0;

            let vx = velocity.0.x + rng.random_range(-5.0..=5.0);
            let vy = velocity.0.y + rng.random_range(-5.0..=5.0);
            let child_gen = creature.generation + 1;

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
                    energy: 100.0,
                    age: 0.0,
                    time_since_reproduction: 0.0,
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
