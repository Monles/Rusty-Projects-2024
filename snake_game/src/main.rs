use bevy::prelude::*;
use components::*;
use systems::*;
use constants::*;

mod components;
mod systems;
mod constants;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Score::default())
        .add_startup_system(setup)
        .add_systems(Update, (snake_movement, snake_eating, snake_growth, game_over, camera_follow))
        .run();
}
