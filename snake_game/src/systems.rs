use bevy::prelude::*;
use rand::prelude::*;

use crate::components::*;
use crate::constants::*;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Add score text
    commands.spawn((
        TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        ScoreText,
    ));

    let head_material = materials.add(ColorMaterial::from(Color::rgb(0.7, 0.7, 0.7)));
    let food_material = materials.add(ColorMaterial::from(Color::rgb(1.0, 0.0, 0.0)));

    commands.insert_resource(SnakeTextures {
        head: head_material.clone().into(),
        food: food_material.clone().into(),
    });

    commands.insert_resource(SnakeMoveTimer(Timer::from_seconds(SNAKE_MOVEMENT_INTERVAL, TimerMode::Repeating)));

    let initial_position = Position { x: 0, y: 0 };

    let snake_head = commands
        .spawn((
            SpriteBundle {
                material: head_material.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        initial_position.x as f32 * GRID_SIZE,
                        initial_position.y as f32 * GRID_SIZE,
                        0.0,
                    ),
                    ..default()
                },
                ..default()
            },
            SnakeHead,
            initial_position,
        ))
        .id();

    commands.insert_resource(SnakeSegments(vec![snake_head]));

    spawn_food(&mut commands, food_material.clone());
}

pub fn snake_movement(
    time: Res<Time>,
    mut timer: ResMut<SnakeMoveTimer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Position), With<SnakeHead>>,
    mut segment_query: Query<&mut Position, With<SnakeSegment>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let (mut head_transform, mut head_position) = query.single_mut();
        let mut segment_positions = segment_query.iter_mut().map(|pos| *pos).collect::<Vec<_>>();

        segment_positions.insert(0, *head_position);

        if let Some(_last) = segment_positions.pop() {
            if keyboard_input.pressed(KeyCode::Left) {
                head_position.x -= 1;
            } else if keyboard_input.pressed(KeyCode::Right) {
                head_position.x += 1;
            } else if keyboard_input.pressed(KeyCode::Down) {
                head_position.y -= 1;
            } else if keyboard_input.pressed(KeyCode::Up) {
                head_position.y += 1;
            }

            for (mut pos, segment_pos) in segment_query.iter_mut().zip(segment_positions.iter()) {
                *pos = *segment_pos;
            }

            head_transform.translation.x = head_position.x as f32 * GRID_SIZE;
            head_transform.translation.y = head_position.y as f32 * GRID_SIZE;
        }
    }
}

pub fn snake_eating(
    mut commands: Commands,
    mut score: ResMut<Score>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut score_query: Query<&mut Text, With<ScoreText>>,
    textures: Res<SnakeTextures>,
) {
    for head_pos in head_positions.iter() {
        for (entity, food_pos) in food_positions.iter() {
            if head_pos == food_pos {
                // Increase score
                score.0 += 1;

                // Update the score text
                for mut text in score_query.iter_mut() {
                    text.sections[0].value = format!("Score: {}", score.0);
                }

                // Despawn the food
                commands.entity(entity).despawn();

                // Save the last tail position before the snake grows
                last_tail_position.0 = Some(*food_pos);

                // Spawn new food
                let new_food_position = Position::random();
                commands.spawn(SpriteBundle {
                    material: textures.food.clone().into(),
                    transform: Transform {
                        translation: Vec3::new(
                            new_food_position.x as f32 * GRID_SIZE,
                            new_food_position.y as f32 * GRID_SIZE,
                            0.0,
                        ),
                        ..default()
                    },
                    ..default()
                })
                .insert(Food)
                .insert(new_food_position);

                break;
            }
        }
    }
}

pub fn snake_growth(
    mut commands: Commands,
    segments: ResMut<SnakeSegments>,
    last_tail_position: Res<LastTailPosition>,
    textures: Res<SnakeTextures>,
) {
    if let Some(tail_position) = last_tail_position.0 {
        commands.spawn((
            SpriteBundle {
                material: textures.head.clone().into(),
                transform: Transform {
                    translation: Vec3::new(
                        tail_position.x as f32 * GRID_SIZE,
                        tail_position.y as f32 * GRID_SIZE,
                        0.0,
                    ),
                    ..default()
                },
                ..default()
            },
            SnakeSegment,
            tail_position,
        ));
    }
}

pub fn game_over(
    mut commands: Commands,
    mut segments: ResMut<SnakeSegments>,
    head_positions: Query<&Position, With<SnakeHead>>,
    segment_positions: Query<&Position, With<SnakeSegment>>,
    textures: Res<SnakeTextures>,
) {
    if let Some(head_pos) = head_positions.iter().next() {
        for segment_pos in segment_positions.iter() {
            if head_pos == segment_pos {
                // Game over, reset the snake
                for entity in segments.0.iter() {
                    commands.entity(*entity).despawn();
                }

                segments.0.clear();

                let initial_position = Position { x: 0, y: 0 };
                let snake_head = commands
                    .spawn((
                        SpriteBundle {
                            material: textures.head.clone().into(),
                            transform: Transform {
                                translation: Vec3::new(
                                    initial_position.x as f32 * GRID_SIZE,
                                    initial_position.y as f32 * GRID_SIZE,
                                    0.0,
                                ),
                                ..default()
                            },
                            ..default()
                        },
                        SnakeHead,
                        initial_position,
                    ))
                    .id();

                segments.0.push(snake_head);

                break;
            }
        }
    }
}

pub fn camera_follow(
    mut camera_query: ParamSet<(
        Query<&mut Transform, With<Camera>>,
        Query<&Transform, With<SnakeHead>>,
    )>,
) {
    if let Some(head_transform) = camera_query.p1().iter().next() {
        let head_translation = head_transform.translation;
        for mut camera_transform in camera_query.p0().iter_mut() {
            camera_transform.translation.x = head_translation.x;
            camera_transform.translation.y = head_translation.y;
        }
    }
}

fn spawn_food(commands: &mut Commands, material: Handle<Image>) {
    let position = Position::random();
    commands.spawn(SpriteBundle {
        material: material.into(),
        transform: Transform {
            translation: Vec3::new(position.x as f32 * GRID_SIZE, position.y as f32 * GRID_SIZE, 0.0),
            ..default()
        },
        ..default()
    })
    .insert(Food)
    .insert(position);
}
