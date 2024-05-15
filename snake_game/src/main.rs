mod components;
mod systems;

use bevy::prelude::*;
use systems::SnakeGamePlugin;

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake Game".to_string(),
                resolution: (800.0, 600.0).into(),
                resizable: false,
                ..Default::default()
            }),
            ..default()
        }))
        .add_plugin(SnakeGamePlugin)
        .run();
}
