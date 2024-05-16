use bevy::prelude::*;

#[derive(Component)]
pub struct SnakeHead;

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Component)]
pub struct Food;

#[derive(Component, Clone, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Default, Resource)]
pub struct SnakeMoveTimer(pub Timer);

#[derive(Default, Resource)]
pub struct LastTailPosition(pub Option<Position>);

#[derive(Resource)]
pub struct SnakeTextures {
    pub head: Handle<Image>,
    pub food: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct SnakeSegments(pub Vec<Entity>);

#[derive(Resource, Default)]
pub struct Score(pub u32);

#[derive(Component)]
pub struct ScoreText;
