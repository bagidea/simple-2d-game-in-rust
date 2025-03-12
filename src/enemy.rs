use crate::{components::{Enemy, Movable, Velocity}, EnemySpawnTimer, GameColors, GameShapes, WinSize};
use bevy::prelude::*;
use rand::{rng, rngs::ThreadRng, Rng};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_enemy_system);
    }
}

fn spawn_enemy_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    game_colors: Res<GameColors>,
    game_shapes: Res<GameShapes>
) {
    if timer.0.tick(time.delta()).just_finished() {
        let enemy: Entity = commands.spawn((
            Mesh2d(game_shapes.enemy_body.clone()),
            MeshMaterial2d(game_colors.enemy_body.clone())
        ))
        .insert(Enemy)
        .insert(Velocity { x: 0., y: -0.2})
        .insert(Movable { auto_despawn: true })
        .id();

        let back: Entity = commands.spawn((
            Mesh2d(game_shapes.enemy_back.clone()),
            MeshMaterial2d(game_colors.enemy_body.clone()),
            Transform::from_xyz(0., 0., -0.1)
        ))
        .id();

        let offset: f32 = 35.;

        let mut spawn_ipon = |offset: f32| {
            commands.spawn((
                Mesh2d(game_shapes.enemy_ipon.clone()),
                MeshMaterial2d(game_colors.enemy_light.clone()),
                Transform::from_xyz(offset, 0., 0.1)
            ))
            .id()
        };

        let ipon_left: Entity = spawn_ipon(-offset);
        let ipon_right: Entity = spawn_ipon(offset);

        let eye: Entity = commands.spawn((
            Mesh2d(game_shapes.enemy_eye.clone()),
            MeshMaterial2d(game_colors.enemy_eye.clone()),
            Transform::from_xyz(0., 0., 0.1)
        ))
        .id();

        let eye_inside: Entity = commands.spawn((
            Mesh2d(game_shapes.enemy_eye_inside.clone()),
            MeshMaterial2d(game_colors.enemy_eye_inside.clone()),
            Transform::from_xyz(0., -10., 0.2)
        ))
        .id();

        let mut rng: ThreadRng = rng();
        let win_width_half: f32 = win_size.w / 2.;

        commands.entity(enemy)
        .add_children(&[back, ipon_left, ipon_right, eye, eye_inside])
        .insert(Transform::from_xyz(
            rng.random_range(-win_width_half + 50.0..win_width_half - 50.),
            win_size.h / 2. + 100.,
            -0.5
        ));
    }
}