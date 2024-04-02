use std::time::Duration;
use bevy::prelude::*;
use rand::Rng;
use crate::SCORE;
use crate::components::*;
use crate::components::Direction;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        TimerStruct(Timer::from_seconds(5., TimerMode::Repeating)),
        BlockTimer,
    ));
}

pub fn update_spawn(
    time: Res<Time>,
    mut timer_query: Query<&mut TimerStruct, With<BlockTimer>>,
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
                    Block,
                ));

                println!(
                    "[Bevy Test] Spawned block; size: {} x {}",
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

pub fn update_move(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Direction, &mut Transform), With<Block>>,
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
