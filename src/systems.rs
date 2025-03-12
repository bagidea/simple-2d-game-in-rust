use crate::{components::{Bullet, Enemy, Explosion, ExplosionAnimationTime, ExplosionColor, ExplosionTimer, ExplosionToSpawn, Movable, Player, Velocity}, EnemySpawnTimer, GameColors, GameShapes, WinSize, BASE_SPEED, BULLET_SCALE, ENEMY_SCALE, TIME_STEP};
use bevy::{core_pipeline::{bloom::Bloom, tonemapping::Tonemapping}, math::bounding::{Aabb2d, IntersectsVolume}, prelude::*, sprite::AlphaMode2d, utils::hashbrown::HashSet, window::PrimaryWindow};

pub struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_system);

        app.add_systems(Update, (
            movement_system,
            enemy_hittest_laser,
            explosion_spawn_system,
            explosion_animation_system
        ));
    }
}

fn setup_system(
    mut commands: Commands,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        window.position = WindowPosition::Centered(MonitorSelection::Current);

        let win_size: WinSize = WinSize { w: window.width(), h: window.height() };
        commands.insert_resource(win_size);
    }

    commands.spawn((
        Camera2d,
        Camera {
            hdr: true,
            ..default()
        },
        Tonemapping::AcesFitted,
        Bloom::default()
    ));

    let game_shapes: GameShapes = GameShapes {
        // Player
        player_body: meshes.add(Capsule2d::new(10., 30.)),
        player_wing: meshes.add(CircularSegment::new(45., 1.25)),
        player_tail: meshes.add(Triangle2d::new(
            Vec2::Y * -10.,
            Vec2::new(-20., -30.),
            Vec2::new(20., -30.)
        )),
        player_base_gun: meshes.add(Circle::new(8.)),
        player_base_gun_inside: meshes.add(Circle::new(5.)),
        player_gun: meshes.add(Triangle2d::new(
            Vec2::Y * 40.,
            Vec2::new(-8., 10.),
            Vec2::new(8., 10.)
        )),
        player_bullet: meshes.add(Rectangle::new(BULLET_SCALE.x, BULLET_SCALE.y)),

        // Enemy
        enemy_body: meshes.add(Circle::new(25.)),
        enemy_back: meshes.add(Rectangle::new(60., 10.)),
        enemy_ipon: meshes.add(Rectangle::new(10., 30.)),
        enemy_eye: meshes.add(Circle::new(20.)),
        enemy_eye_inside: meshes.add(Circle::new(10.)),

        // Explosion
        explosion_ring: meshes.add(Annulus::new(100., 80.))
    };

    commands.insert_resource(game_shapes);

    let game_colors: GameColors = GameColors {
        // Player
        player_body: materials.add(Color::srgb(1., 1., 1.)),
        player_light: materials.add(Color::srgb(0., 0., 2.)),
        player_light_green: materials.add(Color::srgb(0., 3., 0.)),
        player_bullet: materials.add(Color::srgb(0., 0., 5.)),
        
        // Enemy
        enemy_body: materials.add(Color::srgb(1., 0., 0.)),
        enemy_light: materials.add(Color::srgb(2., 0., 0.)),
        enemy_eye: materials.add(Color::srgb(0.5, 0.5, 0.5)),
        enemy_eye_inside: materials.add(ColorMaterial {
            color: Color::srgba(5., 0., 0., 1.),
            alpha_mode: AlphaMode2d::Blend,
            ..default()
        }),

        // Explosion
        explosion_ring: materials.add(ColorMaterial {
            color: Color::srgba(0., 0., 10., 1.),
            alpha_mode: AlphaMode2d::Blend,
            ..default()
        })
    };

    commands.insert_resource(game_colors);

    // Enemy Spawn Timer
    commands.insert_resource(EnemySpawnTimer::default());
}

fn movement_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable, Option<&Player>)>
) {
    for (entity, velocity, mut transform, movable, player) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        let win_width_half: f32 = win_size.w / 2.;
        let win_height_half: f32 = win_size.h / 2.;

        if let Some(_player) = player {
            let wall_left: f32 = -win_width_half + 50.;
            let wall_right: f32 = win_width_half - 50.;

            translation.x = if translation.x <= wall_left {
                wall_left
            }
            else if translation.x >= wall_right {
                wall_right
            } else {
                translation.x
            }
        }

        if movable.auto_despawn {
            const MARGIN: f32 = 300.;

            if translation.y < -win_height_half - MARGIN ||
                translation.y > win_height_half + MARGIN
            {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn enemy_hittest_laser(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    let mut check_despawn: HashSet<Entity> = HashSet::new();

    for (bullet_entiry, bullet_transform) in bullet_query.iter() {
        if check_despawn.contains(&bullet_entiry) { continue; }

        for (enemy_entiry, enemy_transform) in enemy_query.iter() {
            if check_despawn.contains(&enemy_entiry) || check_despawn.contains(&bullet_entiry) { continue; }

            let collision = Aabb2d::new(
                bullet_transform.translation.truncate(),
                BULLET_SCALE / 2.
            )
            .intersects(&Aabb2d::new(
                enemy_transform.translation.truncate(),
                ENEMY_SCALE / 2.
            ));

            if collision {
                // Despawn Enemy
                commands.entity(enemy_entiry).despawn_recursive();
                check_despawn.insert(enemy_entiry);

                // Despawn Bullet
                commands.entity(bullet_entiry).despawn();
                check_despawn.insert(bullet_entiry);

                // Spawn Explosion
                commands.spawn(ExplosionToSpawn(enemy_transform.translation + Vec3::Z * 0.5, crate::components::ExplosionColor::BLUE, 0.1, 0.6));
                commands.spawn(ExplosionToSpawn(enemy_transform.translation + Vec3::Z * 0.5, crate::components::ExplosionColor::RED, 0.15, 0.5));
            }
        }
    }
}

fn explosion_spawn_system(
    mut commands: Commands,
    query: Query<(Entity, &ExplosionToSpawn)>,
    game_shapes: Res<GameShapes>,
    game_colors: Res<GameColors>
) {
    for (explosion_spawn_entity, explosion_to_spawn) in query.iter() {
        commands.spawn((
            Mesh2d(game_shapes.explosion_ring.clone()),
            MeshMaterial2d(match explosion_to_spawn.1 {
                ExplosionColor::BLUE => game_colors.explosion_ring.clone(),
                ExplosionColor::RED => game_colors.enemy_eye_inside.clone()
            }),
            Transform::from_translation(explosion_to_spawn.0).with_scale(Vec3::new(0.01, 0.01, 1.))
        ))
        .insert(Explosion)
        .insert(ExplosionTimer::default())
        .insert(ExplosionAnimationTime(explosion_to_spawn.2, explosion_to_spawn.3));

        commands.entity(explosion_spawn_entity).despawn();
    }
}

fn explosion_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &MeshMaterial2d<ColorMaterial>, &mut ExplosionTimer, &ExplosionAnimationTime), With<Explosion>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    for (entity, mut transform, material, mut timer, explosion_animation_time) in query.iter_mut() {
        if timer.0.tick(time.delta()).finished() {
            commands.entity(entity).despawn_recursive();
        } else {
            transform.scale += Vec3::splat(explosion_animation_time.0);

            if let Some(_material) = materials.get_mut(material) {
                let mut color = _material.color.to_srgba();
                color.alpha = (color.alpha * explosion_animation_time.1).max(0.0);

                let alpha_mode: AlphaMode2d = _material.alpha_mode;

                commands.entity(entity).insert(MeshMaterial2d(materials.add(ColorMaterial {
                        color: Color::srgba(color.red, color.green, color.blue, color.alpha),
                        alpha_mode: alpha_mode,
                        ..default()
                    }),
                ));
            }
        }
    }
}