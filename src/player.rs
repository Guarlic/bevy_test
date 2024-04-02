use bevy::prelude::*;
use crate::components::*;

pub fn setup(mut commands: Commands) {
    let player = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(50., 50.)),
            ..default()
        },
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    };

    commands.spawn((player, Player));
}

pub fn update(
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
