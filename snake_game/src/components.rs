use bevy::prelude::*;

pub struct SnakeHead;
impl Component for SnakeHead {
    type Storage = bevy::ecs::component::TableStorage;
}

pub struct SnakeSegment;
impl Component for SnakeSegment {
    type Storage = bevy::ecs::component::TableStorage;
}

pub struct Food;
impl Component for Food {
    type Storage = bevy::ecs::component::TableStorage;
}

#[derive(Default)]
pub struct LastTailPosition(pub Option<Position>);
impl Resource for LastTailPosition {}

pub struct SnakeMoveTimer(pub Timer);
impl Resource for SnakeMoveTimer {}

pub struct SnakeTextures {
    pub head: Handle<Image>,
    pub food: Handle<Image>,
}
impl Resource for SnakeTextures {}

#[derive(Default)]
pub struct SnakeSegments(pub Vec<Entity>);
impl Resource for SnakeSegments {}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Default)]
pub struct Score(pub u32);
impl Resource for Score {}
