use bevy::prelude::*;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use systems::SystemPlugin;

#[derive(Resource)]
struct WinSize {
    w: f32,
    h: f32
}

#[derive(Resource)]
struct GameShapes {
    // Player
    player_body: Handle<Mesh>,
    player_wing: Handle<Mesh>,
    player_tail: Handle<Mesh>,
    player_base_gun: Handle<Mesh>,
    player_base_gun_inside: Handle<Mesh>,
    player_gun: Handle<Mesh>,
    player_bullet: Handle<Mesh>,

    // Enemy
    enemy_body: Handle<Mesh>,
    enemy_back: Handle<Mesh>,
    enemy_ipon: Handle<Mesh>,
    enemy_eye: Handle<Mesh>,
    enemy_eye_inside: Handle<Mesh>,

    // Explosion
    explosion_ring: Handle<Mesh>
}

#[derive(Resource)]
struct GameColors {
    // Player
    player_body: Handle<ColorMaterial>,
    player_light: Handle<ColorMaterial>,
    player_light_green: Handle<ColorMaterial>,
    player_bullet: Handle<ColorMaterial>,

    // Enemy
    enemy_body: Handle<ColorMaterial>,
    enemy_light: Handle<ColorMaterial>,
    enemy_eye: Handle<ColorMaterial>,
    enemy_eye_inside: Handle<ColorMaterial>,

    // Explosion
    explosion_ring: Handle<ColorMaterial>
}

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2., TimerMode::Repeating))
    }
}

mod components;
mod systems;
mod player;
mod enemy;

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;
const BULLET_SCALE: Vec2 = Vec2::new(5., 10.);
const ENEMY_SCALE: Vec2 = Vec2::new(70., 50.);

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Simple 2D Game in Rust".to_string(),
            resolution: (600., 800.).into(),
            resizable: false,
            ..default()
        }),
        ..default()
    }))
    .add_plugins(SystemPlugin)
    .add_plugins(PlayerPlugin)
    .add_plugins(EnemyPlugin)
    .run();
}