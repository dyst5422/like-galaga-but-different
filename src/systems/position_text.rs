use crate::components::player::Player;
use bevy::prelude::*;
use bevy_rapier2d::prelude::RigidBodyPosition;

pub struct PositionText;

pub fn text_initialization(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(TextBundle {
            text: Text::with_section(
                // accepts a String or any type that converts into a String, such as &str
                "",
                TextStyle {
                    font: asset_server.load("fonts/Roboto-Regular.ttf"),
                    font_size: 3.0,
                    ..Default::default()
                },
                // you can still use Default
                Default::default(),
            ),
            ..Default::default()
        })
        .insert(PositionText);
}

pub fn update_position_text(
    mut position_query: Query<&RigidBodyPosition, With<Player>>,
    mut text_query: Query<&mut Text, With<PositionText>>,
) {
    for mut text in text_query.iter_mut() {
        for position in position_query.iter_mut() {
            text.sections[0].value = format!(
                "[{}, {}]",
                position.position.translation.vector.x, position.position.translation.vector.y
            );
        }
    }
}
