use bevy::prelude::*;
use crate::components::*;
use rand::Rng;

const GRID_SIZE: f32 = 64.0; // Adjusted to 64x64 pixels
const GRID_WIDTH: i32 = 12; // 800 / 64
const GRID_HEIGHT: i32 = 9; // 600 / 64
const SNAKE_SIZE: f32 = 64.0;
const SNAKE_MOVEMENT_INTERVAL: f32 = 0.15;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    // Load textures
    let snake_head_handle = asset_server.load("textures/snake_head_texture.png");
    let food_handle = asset_server.load("textures/food_texture.png");

    commands.insert_resource(SnakeMoveTimer(Timer::from_seconds(SNAKE_MOVEMENT_INTERVAL, TimerMode::Repeating)));
    commands.insert_resource(SnakeTextures { head: snake_head_handle.clone(), food: food_handle.clone() });
    commands.insert_resource(LastTailPosition::default());
    commands.insert_resource(SnakeSegments(vec![]));
    commands.insert_resource(Score::default());

    spawn_snake(&mut commands, snake_head_handle);
    spawn_food(&mut commands, food_handle);

    // Setup UI
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(
                "Score: 0",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                }
            ),
            ..Default::default()
        })
        .insert(ScoreText);
    });
}

fn spawn_snake(commands: &mut Commands, head_texture: Handle<Image>) {
    let segment = commands.spawn(SpriteBundle {
        texture: head_texture,
        transform: Transform {
            scale: Vec3::splat(SNAKE_SIZE / 64.0), // Adjust scale to match 64x64
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(SnakeHead)
    .insert(Position { x: GRID_WIDTH / 2, y: GRID_HEIGHT / 2 })
    .insert(SnakeSegment)
    .id();

    commands.insert_resource(SnakeSegments(vec![segment]));
}

fn spawn_food(commands: &mut Commands, texture: Handle<Image>) {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..GRID_WIDTH);
    let y = rng.gen_range(0..GRID_HEIGHT);
    commands.spawn(SpriteBundle {
        texture,
        transform: Transform {
            translation: Vec3::new(x as f32 * GRID_SIZE, y as f32 * GRID_SIZE, 0.0),
            scale: Vec3::splat(SNAKE_SIZE / 64.0), // Adjust scale to match 64x64
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Food)
    .insert(Position { x, y });
}

pub struct SnakeGamePlugin;

impl Plugin for SnakeGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_systems(
                Update,
                (
                    snake_movement,
                    snake_eating.after(snake_movement),
                    snake_growth.after(snake_eating),
                    game_over,
                    camera_follow,
                    update_score,
                )
            );
    }
}

fn snake_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut timer: ResMut<SnakeMoveTimer>,
    mut head_query: Query<(&mut Transform, &mut Position), With<SnakeHead>>,
    mut segment_query: Query<(&mut Transform, &mut Position), (With<SnakeSegment>, Without<SnakeHead>)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        if let Ok((mut head_transform, mut head_position)) = head_query.get_single_mut() {
            let mut direction = (0, 0);
            if keyboard_input.pressed(KeyCode::Up) {
                direction.1 += 1;
            } else if keyboard_input.pressed(KeyCode::Down) {
                direction.1 -= 1;
            } else if keyboard_input.pressed(KeyCode::Left) {
                direction.0 -= 1;
            } else if keyboard_input.pressed(KeyCode::Right) {
                direction.0 += 1;
            }

            let new_head_position = Position {
                x: (head_position.x + direction.0).clamp(0, GRID_WIDTH - 1),
                y: (head_position.y + direction.1).clamp(0, GRID_HEIGHT - 1),
            };

            let mut segment_positions: Vec<Position> = segment_query.iter_mut().map(|(_, pos)| pos.clone()).collect();
            if !segment_positions.is_empty() {
                segment_positions.insert(0, head_position.clone());
                segment_positions.pop();
                for (i, (mut transform, mut position)) in segment_query.iter_mut().enumerate() {
                    *position = segment_positions[i].clone();
                    transform.translation.x = position.x as f32 * GRID_SIZE;
                    transform.translation.y = position.y as f32 * GRID_SIZE;
                }
            }

            *head_position = new_head_position.clone();
            head_transform.translation.x = new_head_position.x as f32 * GRID_SIZE;
            head_transform.translation.y = new_head_position.y as f32 * GRID_SIZE;
        }
    }
}

fn snake_eating(
    mut commands: Commands,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
    mut last_tail_position: ResMut<LastTailPosition>,
    textures: Res<SnakeTextures>,
    mut score: ResMut<Score>,
) {
    for head_position in head_positions.iter() {
        for (entity, food_position) in food_positions.iter() {
            if head_position == food_position {
                commands.entity(entity).despawn();
                last_tail_position.0 = Some(food_position.clone());
                spawn_food(&mut commands, textures.food.clone());
                score.0 += 1;
            }
        }
    }
}

fn snake_growth(
    mut commands: Commands,
    mut segments: ResMut<SnakeSegments>,
    mut last_tail_position: ResMut<LastTailPosition>,
    textures: Res<SnakeTextures>,
) {
    if let Some(tail_position) = &last_tail_position.0 {
        let segment = commands.spawn(SpriteBundle {
            texture: textures.head.clone(),
            transform: Transform {
                translation: Vec3::new(tail_position.x as f32 * GRID_SIZE, tail_position.y as f32 * GRID_SIZE, 0.0),
                scale: Vec3::splat(SNAKE_SIZE / 64.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SnakeSegment)
        .insert(tail_position.clone())
        .id();

        segments.0.push(segment);
        last_tail_position.0 = None; // Correctly set the last tail position to None
    }
}

fn game_over(
    mut commands: Commands,
    mut segments: ResMut<SnakeSegments>,
    head_positions: Query<&Position, With<SnakeHead>>,
    segment_positions: Query<&Position, With<SnakeSegment>>,
    textures: Res<SnakeTextures>,
) {
    if segments.0.len() > 1 {
        if let Some(head_pos) = head_positions.iter().next() {
            for segment_pos in segment_positions.iter() {
                if head_pos == segment_pos {
                    for entity in segments.0.iter() {
                        commands.entity(*entity).despawn();
                    }
                    segments.0.clear();
                    spawn_snake(&mut commands, textures.head.clone());
                    break;
                }
            }
        }
    }
}

fn camera_follow(
    mut camera_query: ParamSet<(
        Query<&mut Transform, With<Camera>>,
        Query<&Transform, With<SnakeHead>>,
    )>,
) {
    if let Some(head_transform) = camera_query.p1().iter().next() {
        let head_transform = *head_transform; // Clone the head transform to avoid borrowing issues
        for mut camera_transform in camera_query.p0().iter_mut() {
            camera_transform.translation.x = head_transform.translation.x;
            camera_transform.translation.y = head_transform.translation.y;
        }
    }
}

fn update_score(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Score: {}", score.0);
    }
}
