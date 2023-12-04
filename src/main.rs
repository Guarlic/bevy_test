use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::WindowTheme;
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

static mut SCORE: i32 = 0;

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
        .add_systems(
            Startup,
            (
                setup_camera,
                setup_score,
                setup_player,
                setup_enemy_timer,
                setup_laser_timer,
            ),
        )
        .add_systems(
            Update,
            (
                player_move,
                update_enemy,
                update_laser,
                update_score,
                enemy_move,
                game_over_collision,
            ),
        )
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Laser;

#[derive(Component)]
struct TimerStruct(Timer);

#[derive(Component)]
struct EnemyTimer;

#[derive(Component)]
struct LaserTimer;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    sleep(Duration::from_secs(1));

    println!("[Bevy Test] Game start");
}

fn setup_score(mut commands: Commands, asset: Res<AssetServer>) {
    unsafe {
        commands.spawn((
            TextBundle::from_section(
                format!("Score: {}", SCORE),
                TextStyle {
                    font: asset.load("font.ttf"),
                    font_size: 50.,
                    ..default()
                },
            ),
            ScoreText,
        ));
    }
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

fn setup_enemy_timer(mut commands: Commands) {
    commands.spawn((
        TimerStruct(Timer::from_seconds(5., TimerMode::Repeating)),
        EnemyTimer,
    ));
}

fn setup_laser_timer(mut commands: Commands) {
    commands.spawn((
        TimerStruct(Timer::from_seconds(25., TimerMode::Repeating)),
        LaserTimer,
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

                player_transform.translation.x = player_transform.translation.x.clamp(-450., 450.);
                player_transform.translation.y = player_transform.translation.y.clamp(-300., 300.);
            }
        }
    }
}

fn update_enemy(
    time: Res<Time>,
    mut timer_query: Query<&mut TimerStruct, With<EnemyTimer>>,
    player_query: Query<&Transform, With<Player>>,
    mut commands: Commands,
) {
    for mut timer in timer_query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            for player_transform in player_query.iter() {
                let player_x = player_transform.translation.x;

                let mut random = rand::thread_rng();

                let enemy_size_x = random.gen_range(30..=100) as f32;
                let enemy_size_y = random.gen_range(30..=100) as f32;

                let enemy_direction_temp = random.gen_range(1..=2);
                let enemy_direction = match enemy_direction_temp {
                    1 => Direction::Up,
                    _ => Direction::Down,
                };

                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(enemy_size_x, enemy_size_y)),
                            color: Color::rgb(0.6, 0.8, 0.8).into(),
                            ..default()
                        },
                        transform: Transform::from_xyz(player_x, 150., 0.),
                        ..default()
                    },
                    enemy_direction,
                    Enemy,
                ));

                println!(
                    "[Bevy Test] Spawned enemy; size: {} x {}",
                    enemy_size_x, enemy_size_y
                );

                unsafe {
                    SCORE += 1;

                    println!("[Bevy Test] Score: +1! ({})", SCORE);
                }

                let timer_duration = random.gen_range(3..5);

                timer.0.set_duration(Duration::from_secs(timer_duration));
                timer.0.reset();
            }
        }
    }
}

fn update_laser(
    time: Res<Time>,
    mut timer_query: Query<&mut TimerStruct, With<LaserTimer>>,
    player_query: Query<&Transform, With<Player>>,
    laser_query: Query<Entity, With<Laser>>,
    mut commands: Commands,
) {
    for mut timer in timer_query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            for player_transform in player_query.iter() {
                for laser in laser_query.iter() {
                    commands.entity(laser).despawn();
                }

                let player_y = player_transform.translation.y;

                let mut random = rand::thread_rng();

                let laser_plus_minus = random.gen_range(1..=2);

                let laser_y = match laser_plus_minus {
                    1 => player_y + 50.,
                    _ => player_y - 50.,
                };

                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(950., 20.)),
                            color: Color::rgb(0.9, 0.6, 0.6).into(),
                            ..default()
                        },
                        transform: Transform::from_xyz(0., laser_y, 0.),
                        ..default()
                    },
                    Laser,
                ));

                println!("[Bevy Test] Spawned laser!");
            }
        }
    }
}

fn update_score(mut text_query: Query<&mut Text, With<ScoreText>>) {
    for mut text in text_query.iter_mut() {
        unsafe {
            text.sections[0].value = format!("Score: {}", SCORE);
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
    laser_query: Query<(&Transform, &Sprite), With<Laser>>,
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
                unsafe {
                    println!("[Bevy Test] Game over; score: {}", SCORE);
                }

                sleep(Duration::from_millis(1500));

                exit_events.send(AppExit);
            }
        }

        for (laser_transform, laser_sprite) in laser_query.iter() {
            let collision = collide(
                player_transform.translation,
                player_sprite.custom_size.unwrap(),
                laser_transform.translation,
                laser_sprite.custom_size.unwrap(),
            );

            if let Some(_) = collision {
                unsafe {
                    println!("[Bevy Test] Game over; score: {}", SCORE);
                }

                sleep(Duration::from_millis(1500));

                exit_events.send(AppExit);
            }
        }
    }
}
