use bevy::prelude::*;

#[derive(Component)]
pub struct Organism {
    pub energy: f32,
    pub age: f32,
    pub generation: u32,
}

#[derive(Component)]
pub struct Creature {
    pub time_since_reproduction: f32,
}

#[derive(Component)]
pub struct Predator {
    pub reproduction_cooldown: f32,
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Clone, Component)]
pub struct Genes {
    pub speed: f32,
    pub size: f32,
    pub color: Color,
}

#[derive(Component, PartialEq, Eq, Debug)]
pub enum State {
    Wandering,
    SeekingFood,
    Reproducing,
    ReproducingSeason,
}

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct Plant {
    pub age: f32,
    pub max_age: f32,
    pub size: f32,
    pub reproduction_timer: f32,
}
