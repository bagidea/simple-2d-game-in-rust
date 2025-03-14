use crate::{
    components::{
        Bullet,
        Movable,
        Player,
        Velocity
    },
    GameColors,
    GameShapes,
    WinSize
};

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player_system);

        app.add_systems(Update, (
            player_keyboard_event_system,
            player_fire_system
        ));
    }
}

fn spawn_player_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    game_colors: Res<GameColors>,
    game_shapes: Res<GameShapes>
) {
    let player: Entity = commands.spawn((
        Mesh2d(game_shapes.player_body.clone()),
        MeshMaterial2d(game_colors.player_body.clone())
    ))
    .insert(Player)
    .insert(Velocity { x: 0., y: 0.})
    .insert(Movable { auto_despawn: false })
    .id();

    let wing: Entity = commands.spawn((
        Mesh2d(game_shapes.player_wing.clone()),
        MeshMaterial2d(game_colors.player_light.clone()),
        Transform::from_xyz(0., -25., -0.1)
    ))
    .id();

    let tail: Entity = commands.spawn((
        Mesh2d(game_shapes.player_tail.clone()),
        MeshMaterial2d(game_colors.player_light.clone()),
        Transform::from_xyz(0., 0., -0.1)
    ))
    .id();

    let base_gun: Entity = commands.spawn((
        Mesh2d(game_shapes.player_base_gun.clone()),
        MeshMaterial2d(game_colors.player_light_green.clone()),
        Transform::from_xyz(0., 10., 0.2)
    ))
    .id();

    let base_gun_inside: Entity = commands.spawn((
        Mesh2d(game_shapes.player_base_gun_inside.clone()),
        MeshMaterial2d(game_colors.enemy_body.clone()),
        Transform::from_xyz(0., 10., 0.3)
    ))
    .id();

    let gun: Entity = commands.spawn((
        Mesh2d(game_shapes.player_gun.clone()),
        MeshMaterial2d(game_colors.player_bullet.clone()),
        Transform::from_xyz(0., 0., 0.1)
    ))
    .id();

    commands.entity(player)
    .add_children(&[wing, tail, base_gun, base_gun_inside, gun])
    .insert(Transform::from_xyz(0., -win_size.h / 2. + 100., 0.));
}

fn player_keyboard_event_system(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if input.pressed(KeyCode::ArrowLeft) {
            -1.
        } else if input.pressed(KeyCode::ArrowRight) {
            1.
        } else {
            0.
        }
    }
}

fn player_fire_system(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    query: Query<&Transform, With<Player>>,
    game_colors: Res<GameColors>,
    game_shapes: Res<GameShapes>
) {
    if let Ok(transform) = query.get_single() {
        if input.just_pressed(KeyCode::Space) {
            commands.spawn((
                Mesh2d(game_shapes.player_bullet.clone()),
                MeshMaterial2d(game_colors.player_bullet.clone()),
                Transform::from_xyz(transform.translation.x, transform.translation.y + 35., -0.1)
            ))
            .insert(Bullet)
            .insert(Velocity { x: 0., y: 1. })
            .insert(Movable { auto_despawn: true });
        }
    }
}