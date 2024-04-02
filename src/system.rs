use bevy::prelude::*;
use std::thread::sleep;
use std::time::Duration;
use bevy::app::AppExit;
use bevy::sprite::collide_aabb::collide;
use crate::components::*;
use crate::SCORE;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    sleep(Duration::from_secs(1));

    println!("[Bevy Test] Game start");
}

pub fn setup_score(mut commands: Commands, asset: Res<AssetServer>) {
    let score = TextBundle::from_section(
        "Score: 0".to_string(),
        TextStyle {
            font: asset.load("font.ttf"),
            font_size: 50.,
            ..default()
        },
    );

    commands.spawn((score, ScoreText));
}

pub fn update_game_over(
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
                unsafe {
                    println!("[Bevy Test] Game over; score: {}", SCORE);
                }

                sleep(Duration::from_millis(1500));

                exit_events.send(AppExit);
            }
        }
    }
}

pub fn update_score(mut text_query: Query<&mut Text, With<ScoreText>>) {
    for mut text in text_query.iter_mut() {
        unsafe {
            text.sections[0].value = format!("Score: {}", SCORE);
        }
    }
}
