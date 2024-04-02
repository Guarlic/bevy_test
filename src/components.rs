use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Block;

#[derive(Component)]
pub struct Laser;

#[derive(Component)]
pub struct TimerStruct(pub(crate) Timer);

#[derive(Component)]
pub struct BlockTimer;

#[derive(Component)]
pub struct LaserTimer;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub enum Direction {
    Up,
    Down,
}
