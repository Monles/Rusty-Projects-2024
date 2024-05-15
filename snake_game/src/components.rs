use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct SnakeHead;

#[derive(Component, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Position {
            x: rng.gen_range(-10..10),
            y: rng.gen_range(-10..10),
        }
    }
}

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Component)]
pub struct Food;

#[derive(Default, Resource)]
pub struct SnakeSegments(pub Vec<Entity>);

#[derive(Default, Resource)]
pub struct LastTailPosition(pub Option<Position>);

#[derive(Default, Resource)]
pub struct Score(pub u32);

#[derive(Component)]
pub struct ScoreText;

#[derive(Resource)]
pub struct SnakeMoveTimer(pub Timer);

#[derive(Resource)]
pub struct SnakeTextures {
    pub head: Handle<Image>,
    pub food: Handle<Image>,
}
