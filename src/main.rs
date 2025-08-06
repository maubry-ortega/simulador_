use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use rand::prelude::*; // Asegúrate de que esto incluye el trait Rng

#[derive(Component)]
struct Creature {
    energy: f32,
    age: f32,
    time_since_reproduction: f32,
    generation: u32,
}

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Food;

#[derive(Resource, Default)]
struct Stats {
    total_reproductions: usize,
    time_since_spawn: f32,
    max_generation: u32,
}

#[derive(Component)]
struct FpsText;

fn main() {
    App::new()
        .insert_resource(Stats::default())
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            move_creatures,
            food_collision_system,
            reproduction_system,
            boundary_bounce_system,
            spawn_random_food,
            update_hud,
            update_fps,
        ))
        .run();
}

fn color_from_generation(r#gen: u32) -> Color {
    let hue = (r#gen as f32 * 40.0) % 360.0;
    Color::hsl(hue, 0.8, 0.6)
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Text::new("FPS: "),
            TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                ..default()
            },
        ))
        .with_child((TextSpan::from(""), TextFont { font_size: 24.0, ..default() }, FpsText));

    commands.spawn((
        Text::new(""),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 20.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));

    spawn_initial_creatures(&mut commands);
    spawn_food(&mut commands);
}

fn move_creatures(
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

fn food_collision_system(
    mut commands: Commands,
    mut creature_query: Query<(&mut Creature, &Transform)>,
    food_query: Query<(Entity, &Transform), With<Food>>,
) {
    for (mut creature, creature_transform) in creature_query.iter_mut() {
        for (food_entity, food_transform) in food_query.iter() {
            let dist = creature_transform.translation.truncate().distance(food_transform.translation.truncate());
            if dist < 15.0 {
                creature.energy += 20.0;
                if commands.get_entity(food_entity).is_ok() {
                    let _ = commands.entity(food_entity).despawn();
                }
                break;
            }
        }
    }
}

fn reproduction_system(
    mut commands: Commands,
    mut stats: ResMut<Stats>,
    mut query: Query<(&mut Creature, &Transform, &Velocity)>,
) {
    let mut rng = rand::rng(); // Mantener rand::rng() según tu código

    for (mut creature, transform, velocity) in query.iter_mut() {
        if creature.energy > 120.0 && creature.time_since_reproduction > 5.0 {
            creature.energy -= 40.0;
            creature.time_since_reproduction = 0.0;

            // Corregido: Usar random_range en lugar de gen_range
            let vx = velocity.0.x + rng.random_range(-5.0..=5.0);
            let vy = velocity.0.y + rng.random_range(-5.0..=5.0);
            let child_gen = creature.generation + 1;

            commands.spawn((
                Sprite {
                    color: color_from_generation(child_gen),
                    custom_size: Some(Vec2::splat(30.0)),
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
            ));

            stats.total_reproductions += 1;
            stats.max_generation = stats.max_generation.max(child_gen);
        }
    }
}

fn boundary_bounce_system(
    mut query: Query<(&mut Transform, &mut Velocity)>,
    windows: Query<&Window>,
) {
    let window = windows.single().expect("no window");
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

fn spawn_initial_creatures(commands: &mut Commands) {
    let mut rng = rand::rng(); // Mantener rand::rng() según tu código
    for _ in 0..10 {
        // Corregido: Usar random_range en lugar de gen_range
        let angle = rng.random_range(0.0..=std::f32::consts::TAU);
        let speed = rng.random_range(20.0..=60.0);
        let dir = Vec2::from_angle(angle) * speed;
        commands.spawn((
            Sprite {
                color: color_from_generation(0),
                custom_size: Some(Vec2::splat(30.0)),
                ..default()
            },
            Transform::from_xyz(rng.random_range(-300.0..=300.0), rng.random_range(-200.0..=200.0), 0.0),
            GlobalTransform::default(),
            Visibility::Visible,
            Creature {
                energy: 100.0,
                age: 0.0,
                time_since_reproduction: 0.0,
                generation: 0,
            },
            Velocity(dir),
        ));
    }
}

fn spawn_food(commands: &mut Commands) {
    let mut rng = rand::rng(); // Mantener rand::rng() según tu código
    for _ in 0..20 {
        commands.spawn((
            Sprite {
                color: Color::srgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::splat(10.0)),
                ..default()
            },
            Transform::from_xyz(rng.random_range(-300.0..=300.0), rng.random_range(-200.0..=200.0), 0.0),
            GlobalTransform::default(),
            Visibility::Visible,
            Food,
        ));
    }
}

fn spawn_random_food(
    mut commands: Commands,
    mut stats: ResMut<Stats>,
    time: Res<Time>,
) {
    stats.time_since_spawn += time.delta_secs();
    if stats.time_since_spawn >= 5.0 {
        stats.time_since_spawn = 0.0;
        spawn_food(&mut commands);
    }
}

fn update_hud(
    stats: Res<Stats>,
    creatures: Query<&Creature>,
    foods: Query<(), With<Food>>,
    mut texts: Query<&mut Text>,
) {
    let mut hud_text = texts.iter_mut().last().unwrap();
    let total = creatures.iter().count();
    let avg_gen = if total > 0 {
        creatures.iter().map(|c| c.generation).sum::<u32>() as f32 / total as f32
    } else {
        0.0
    };
    *hud_text = Text::new(format!(
        "Criaturas: {}\nComida: {}\nReproducciones: {}\nMax Gen: {}\nAvg Gen: {:.1}",
        total,
        foods.iter().count(),
        stats.total_reproductions,
        stats.max_generation,
        avg_gen
    ));
}

fn update_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut TextSpan, With<FpsText>>,
) {
    for mut span in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                **span = format!("{value:.2}");
            }
        }
    }
}
