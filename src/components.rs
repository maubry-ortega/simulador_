use bevy::prelude::*;

/// Información evolutiva de la criatura
#[derive(Component)]
pub struct Creature {
    pub energy: f32,
    pub age: f32,
    pub time_since_reproduction: f32,
    pub generation: u32,
}

/// Dirección y velocidad de movimiento
#[derive(Component)]
pub struct Velocity(pub Vec2);

/// Representa comida en el mundo
#[derive(Component)]
pub struct Food;

/// Información genética heredable
#[derive(Clone, Component)]
pub struct Genes {
    pub speed: f32,
    pub size: f32,
    pub color: Color,
}

/// Estado de comportamiento para IA simple
#[derive(Component, PartialEq, Eq)]
pub enum State {
    Wandering,
    SeekingFood,
    Reproducing,
}

/// Texto de FPS en pantalla
#[derive(Component)]
pub struct FpsText;
