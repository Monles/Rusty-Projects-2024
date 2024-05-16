use bevy::prelude::*;

#[derive(Resource)]
pub struct SnakeMoveTimer(pub Timer);

#[derive(Resource)]
pub struct Score(pub i32);

#[derive(Resource)]
pub struct LastTailPosition(pub Option<Position>);

#[derive(Resource)]
pub struct SnakeTextures {
    pub head: Handle<Image>,
    pub segment: Handle<Image>,
    pub food: Handle<Image>,
}
