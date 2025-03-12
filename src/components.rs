use bevy::{ecs::component::Component, math::Vec3, time::{Timer, TimerMode}};

#[derive(Component)]
pub struct Player; 

#[derive(Component)]
pub struct Enemy; 

#[derive(Component)]
pub struct Bullet; 

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32
}

#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool
}

#[derive(Component)]
pub struct Explosion;

#[derive(Component)]
pub enum ExplosionColor {
    BLUE,
    RED
}

#[derive(Component)]
pub struct ExplosionToSpawn(
    pub Vec3,
    pub ExplosionColor,
    pub f32,
    pub f32
);

#[derive(Component)]
pub struct ExplosionAnimationTime(
    pub f32,
    pub f32
);

#[derive(Component)]
pub struct ExplosionTimer(pub Timer);

impl Default for ExplosionTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1., TimerMode::Once))
    }
}