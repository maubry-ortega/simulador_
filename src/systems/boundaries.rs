use crate::components::Velocity;
use bevy::prelude::*;

pub fn boundary_bounce_system(
    mut query: Query<(&mut Transform, &mut Velocity)>,
    windows: Query<&Window>,
) {
    let window = windows.single().expect("Window not found");

    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    for (mut transform, mut velocity) in query.iter_mut() {
        let pos = &mut transform.translation;
        if pos.x > half_width || pos.x < -half_width {
            velocity.0.x *= -1.0;
            pos.x = pos.x.clamp(-half_width, half_width);
        }
        if pos.y > half_height || pos.y < -half_height {
            velocity.0.y *= -1.0;
            pos.y = pos.y.clamp(-half_height, half_height);
        }
    }
}
