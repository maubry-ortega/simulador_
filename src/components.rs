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

#[derive(Component)]
pub struct Food;

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
    ReproducingSeason, // solo para depredadores
}

#[derive(Component)]
pub struct FpsText;
