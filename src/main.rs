use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::WindowTheme;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Test".into(),
                resolution: (1000., 700.).into(),
                window_theme: Some(WindowTheme::Dark),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, setup_player, setup_timer))
        .add_systems(
            Update,
            (player_move, update_enemy, enemy_move, game_over_collision),
        )
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct TimerStruct(Timer);

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    sleep(Duration::from_secs(1));
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

fn setup_timer(mut commands: Commands) {
    commands.spawn(TimerStruct(Timer::from_seconds(5., TimerMode::Repeating)));
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

                player_transform.translation.x = player_transform.translation.x.clamp(-450., 450.);
                player_transform.translation.y = player_transform.translation.y.clamp(-300., 300.);
            }
        }
    }
}

fn update_enemy(
    time: Res<Time>,
    mut timer_query: Query<&mut TimerStruct>,
    player_query: Query<&Transform, With<Player>>,
    mut commands: Commands,
) {
    for mut timer in timer_query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            for player_transform in player_query.iter() {
                let player_x = player_transform.translation.x;

                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(100., 30.)),
                            color: Color::rgb(255., 255., 0.),
                            ..default()
                        },
                        transform: Transform::from_xyz(player_x, 150., 0.),
                        ..default()
                    },
                    Direction::Down,
                    Enemy,
                ));
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

        if enemy_transform.translation.y > 300. {
            *enemy_direction = Direction::Down;
        } else if enemy_transform.translation.y < -300. {
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
                sleep(Duration::from_secs(1));

                exit_events.send(AppExit);
            }
        }
    }
}
