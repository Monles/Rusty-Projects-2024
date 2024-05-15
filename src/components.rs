use bevy::prelude::*;

#[derive(Component)]
pub struct Food;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct SnakeHead;

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Default, Resource)]
pub struct LastTailPosition(pub Option<Position>);

#[derive(Resource)]
pub struct SnakeMoveTimer(pub Timer);

#[derive(Resource)]
pub struct SnakeTextures {
    pub head: Handle<Image>,
    pub food: Handle<Image>,
}

#[derive(Resource)]
pub struct SnakeSegments(pub Vec<Entity>);

pub const SNAKE_MOVEMENT_INTERVAL: f32 = 0.3;
pub const SNAKE_SIZE: f32 = 32.0; // Size for the snake and food sprites
pub const GRID_SIZE: f32 = 32.0; // Each grid cell size
pub const GRID_WIDTH: i32 = 25; // Number of cells horizontally
pub const GRID_HEIGHT: i32 = 19; // Number of cells vertically
