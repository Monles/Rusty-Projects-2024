mod components;
mod systems;

use bevy::prelude::*;
use systems::SnakeGamePlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake Game".to_string(),
            width: 800.0,
            height: 600.0,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(SnakeGamePlugin)
        .run();
}
