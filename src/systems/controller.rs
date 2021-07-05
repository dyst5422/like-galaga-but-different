use bevy::prelude::*;
use crate::components::player::Player;

pub fn controller(input: Res<Input<KeyCode>>, transforms: Query<&Transform, With<Player>>) {
    let a = input.pressed(KeyCode::A);
    if a {
        for item in query.iter() {
            item.translation.clamp(Vec2::from((0.0, 0.0)), Vec2::from((GAME_WIDTH, GAME_HEIGHT)));
            let scaled_amount = 1.2 * mv_amount as f32;

        }
    }

    let d = input.pressed(KeyCode::D);
}
