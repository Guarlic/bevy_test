use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::SCORE;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        TimerStruct(Timer::from_seconds(25., TimerMode::Repeating)),
        LaserTimer,
    ));
}

pub fn update(
    time: Res<Time>,
    mut timer_query: Query<&mut TimerStruct, With<LaserTimer>>,
    laser_query: Query<Entity, With<Laser>>,
    player_query: Query<&Transform, With<Player>>,
    mut commands: Commands,
) {
    for mut timer in timer_query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            for laser in laser_query.iter() {
                commands.entity(laser).despawn();
            }

            for player_transform in player_query.iter() {
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
                            custom_size: Some(Vec2::new(1000., 20.)),
                            color: Color::rgb(0.9, 0.6, 0.6).into(),
                            ..default()
                        },
                        transform: Transform::from_xyz(0., laser_y, 0.),
                        ..default()
                    },
                    Enemy,
                    Laser,
                ));

                println!("[Bevy Test] Spawned laser!");

                unsafe {
                    SCORE += 2;

                    println!("[Bevy Test] Score: +2! ({})", SCORE);
                }
            }
        }
    }
}