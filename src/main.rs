use bevy::prelude::*;
use bevy::window::WindowTheme;
use rand::Rng;

mod player;
mod system;
mod block;
mod laser;
mod components;

static mut SCORE: i32 = 0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Avoid".into(),
                resolution: (1000., 700.).into(),
                window_theme: Some(WindowTheme::Dark),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (
            system::setup_camera,
            system::setup_score,
            player::setup,
            block::setup,
            laser::setup,
        ))
        .add_systems(Update, (
            system::update_score,
            system::update_game_over,
            player::update,
            block::update_spawn,
            block::update_move,
            laser::update,
        ))
        .run();
}

// #[derive(Component)]
// struct Player;
//
// #[derive(Component)]
// struct Enemy;
//
// #[derive(Component)]
// struct Block;
//
// #[derive(Component)]
// struct Laser;
//
// #[derive(Component)]
// struct TimerStruct(Timer);
//
// #[derive(Component)]
// struct BlockTimer;
//
// #[derive(Component)]
// struct LaserTimer;
//
// #[derive(Component)]
// struct ScoreText;
//
// #[derive(Component)]
// enum Direction {
//     Up,
//     Down,
// }