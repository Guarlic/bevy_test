use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_player, setup_enemy))
        .add_systems(Update, (player_move, enemy_move, game_over_collision))
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(50., 50.)),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Player,
    ));
}

fn setup_enemy(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100., 30.)),
                color: Color::rgb(255., 255., 0.),
                ..default()
            },
            transform: Transform::from_xyz(100., 2., 0.),
            ..default()
        },
        Direction::Up,
        Enemy,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100., 30.)),
                color: Color::rgb(255., 255., 0.),
                ..default()
            },
            transform: Transform::from_xyz(-100., 100., 0.),
            ..default()
        },
        Direction::Down,
        Enemy,
    ));
}

fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    for mut player_transform in player_query.iter_mut() {
        let movements = [
            (KeyCode::Up, Vec3::new(0., 1., 0.)),
            (KeyCode::Down, Vec3::new(0., -1., 0.)),
            (KeyCode::Left, Vec3::new(-1., 0., 0.)),
            (KeyCode::Right, Vec3::new(1., 0., 0.)),
        ];

        for (key, direction) in movements.iter() {
            if keys.pressed(*key) {
                const PLAYER_SPEED: f32 = 200.;
                let distance = PLAYER_SPEED * time.delta_seconds();

                player_transform.translation += *direction * distance;
            }
        }
    }
}

fn enemy_move(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Direction, &mut Transform), With<Enemy>>,
) {
    for (mut enemy_direction, mut enemy_transform) in enemy_query.iter_mut() {
        const ENEMY_SPEED: f32 = 80.;
        let distance = ENEMY_SPEED * time.delta_seconds();

        match *enemy_direction {
            Direction::Up => enemy_transform.translation.y += distance,
            Direction::Down => enemy_transform.translation.y -= distance,
        }

        if enemy_transform.translation.y > 200. {
            *enemy_direction = Direction::Down;
        } else if enemy_transform.translation.y < -200. {
            *enemy_direction = Direction::Up;
        }
    }
}

fn game_over_collision(
    mut exit_events: ResMut<Events<AppExit>>,
    player_query: Query<(&Transform, &Sprite), With<Player>>,
    enemy_query: Query<(&Transform, &Sprite), With<Enemy>>,
) {
    for (player_transform, player_sprite) in player_query.iter() {
        for (enemy_transform, enemy_sprite) in enemy_query.iter() {
            let collision = collide(
                player_transform.translation,
                player_sprite.custom_size.unwrap(),
                enemy_transform.translation,
                enemy_sprite.custom_size.unwrap(),
            );

            if let Some(_) = collision {
                println!("Game Over!");
                sleep(Duration::from_secs(1));

                exit_events.send(AppExit);
            }
        }
    }
}
