use bevy::prelude::*;
use crate::components::Velocity;

const MIN_SEPARATION_DISTANCE: f32 = 15.0;
const SEPARATION_FORCE: f32 = 20.0;

pub fn avoid_entity_overlap_system(
    mut query: Query<(&Transform, &mut Velocity)>,
) {
    let positions: Vec<Vec2> = query
        .iter()
        .map(|(t, _)| t.translation.truncate())
        .collect();

    for (i, (transform_i, mut velocity_i)) in query.iter_mut().enumerate() {
        let pos_i = transform_i.translation.truncate();

        for (j, pos_j) in positions.iter().enumerate() {
            if i == j {
                continue;
            }

            let distance = pos_i.distance(*pos_j);
            if distance < MIN_SEPARATION_DISTANCE && distance > 0.01 {
                let push = (pos_i - *pos_j).normalize_or_zero() * SEPARATION_FORCE / distance;
                velocity_i.0 += push;
            }
        }
    }
}
