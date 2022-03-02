// #![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::doc_markdown)]
#![doc = include_str!("../README.md")]


use bevy::app::AppExit;
use bevy::core::Time;
use bevy::prelude::*;
use nalgebra::Isometry2;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::na::Vector2;

pub mod utils;

/// A plugin
pub struct UnfairAdvantagePlugin;

impl Plugin for UnfairAdvantagePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state(AppState::MainMenu)
        .init_resource::<SnakeTimer>()
        .insert_resource(SnakeSegments::default())
        .insert_resource(LastTailPosition::default())
        .add_event::<GameOverEvent>()
        .add_event::<SnakeSplitEvent>()
        .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup_main_menu))
        .add_system_set(
            SystemSet::on_update(AppState::MainMenu)
                .with_system(menu_button_dynamic_colors)
                .with_system(menu_button_action)
            )
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup_main_menu))
        .add_system_set(SystemSet::on_enter(AppState::PauseMenu).with_system(setup_pause_menu))
        .add_system_set(
            SystemSet::on_update(AppState::PauseMenu)
                .with_system(menu_button_dynamic_colors)
                .with_system(menu_button_action)
        )
        .add_system_set(SystemSet::on_exit(AppState::PauseMenu).with_system(cleanup_pause_menu))
        .add_system_set(SystemSet::on_enter(AppState::InOnePlayerGame)
            .with_system(setup_one_player_game)
            .with_system(spawn_snake)
        )
        .add_system_set(
            SystemSet::on_update(AppState::InOnePlayerGame)
                .with_system(slayer_controls)
                .with_system(slayer_animator)
                .with_system(
                    snake_movement_input
                        .label(SnakeAction::Input)
                        .before(SnakeAction::Movement),
                )
                .with_system(game_over.after(SnakeAction::Movement))
                .with_system(snake_movement.label(SnakeAction::Movement))
                .with_system(position_translation)
                .with_system(size_scaling)
        )
        .add_system_set(SystemSet::on_exit(AppState::InOnePlayerGame).with_system(cleanup_game))
        .add_system_set(SystemSet::on_enter(AppState::InTwoPlayerGame).with_system(setup_two_player_game))
        .add_system_set(
            SystemSet::on_update(AppState::InTwoPlayerGame)
            .with_system(slayer_controls)
            .with_system(slayer_animator)
        )
        .add_system_set(SystemSet::on_exit(AppState::InTwoPlayerGame).with_system(cleanup_game))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    PauseMenu,
    InOnePlayerGame,
    InTwoPlayerGame,
}

#[derive(Component)]
enum MenuButtonAction {
    StartOnePlayerGame,
    StartTwoPlayerGame,
    ExitApp,
    ResumeGame,
    QuitGame,
}

#[derive(Component)]
struct OnMainMenuScreen;

#[derive(Component)]
struct OnPauseMenuScreen;

#[derive(Component)]
struct Slayer;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default()).insert(OnMainMenuScreen);
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: Rect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                border: Rect::all(Val::Px(30.0)),
                size: Size{
                    width: Val::Px(700.0),
                    height: Val::Px(700.0),
                },
                ..Default::default()
            },
            color: Color::TEAL.into(),
            ..Default::default()
        })
        .insert(OnMainMenuScreen)
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(500.0), Val::Px(100.0)),
                        // center button
                        margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(MenuButtonAction::StartOnePlayerGame)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Start 1 Player Game",
                            TextStyle {
                                font: asset_server.load("fonts/GoMono-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(500.0), Val::Px(100.0)),
                        // center button
                        margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(MenuButtonAction::StartTwoPlayerGame)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Start 2 Player Game",
                            TextStyle {
                                font: asset_server.load("fonts/GoMono-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(500.0), Val::Px(100.0)),
                        // center button
                        margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(MenuButtonAction::ExitApp)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Exit App",
                            TextStyle {
                                font: asset_server.load("fonts/GoMono-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
        
        });
}

fn menu_button_dynamic_colors(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn cleanup_main_menu(
    mut commands: Commands,
    to_despawn: Query<Entity, With<OnMainMenuScreen>>,
) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default()).insert(OnPauseMenuScreen);
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: Rect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                border: Rect::all(Val::Px(30.0)),
                size: Size{
                    width: Val::Px(700.0),
                    height: Val::Px(500.0),
                },
                ..Default::default()
            },
            color: Color::TEAL.into(),
            ..Default::default()
        })
        .insert(OnPauseMenuScreen)
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(500.0), Val::Px(100.0)),
                        // center button
                        margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(MenuButtonAction::ResumeGame)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Resume",
                            TextStyle {
                                font: asset_server.load("fonts/GoMono-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(500.0), Val::Px(100.0)),
                        // center button
                        margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(MenuButtonAction::QuitGame)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Quit Game",
                            TextStyle {
                                font: asset_server.load("fonts/GoMono-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
        });  
}

fn cleanup_pause_menu(
    mut commands: Commands,
    to_despawn: Query<Entity, With<OnPauseMenuScreen>>,
) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn menu_button_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<State<AppState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, menu_button_action) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                MenuButtonAction::StartOnePlayerGame => state.set(AppState::InOnePlayerGame).unwrap(),
                MenuButtonAction::StartTwoPlayerGame => state.set(AppState::InTwoPlayerGame).unwrap(),
                MenuButtonAction::ExitApp => app_exit_events.send(AppExit),
                MenuButtonAction::ResumeGame => state.pop().unwrap(),
                MenuButtonAction::QuitGame => state.replace(AppState::MainMenu).unwrap(),
            }
        }
    }
}

fn setup_one_player_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut rapier_config: ResMut<RapierConfiguration>
) {
    let slayer_texture_handle = asset_server.load("slayer_idle.png");
    let slayer_texture_atlas = TextureAtlas::from_grid(slayer_texture_handle, Vec2::new(64.0, 64.0), 6, 1);
    let slayer_texture_atlas_handle = texture_atlases.add(slayer_texture_atlas);

    // rapier_config.gravity = Vector2::new(0.0, -10.0);
    let sprite_size_x = 64.0;
    let sprite_size_y = 64.0;
    rapier_config.scale = 32.0;
    let collider_size_x = sprite_size_x / rapier_config.scale;
    let collider_size_y = sprite_size_y / rapier_config.scale;

    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(100.0, 0.1).into(), // the ground
        position: Isometry2::new(Vector2::new(0.0, -15.0).into(), 0.0).into(),
        ..Default::default()
    };
    commands.spawn_bundle(collider);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: slayer_texture_atlas_handle,
        ..Default::default()
    })
    .insert_bundle(RigidBodyBundle{
        mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        body_type: RigidBodyType::Dynamic.into(),
        ..Default::default()
    })
    .insert_bundle(ColliderBundle {
        position: [collider_size_x / 2.0, collider_size_y / 2.0].into(),
        mass_properties: ColliderMassProps::Density(2.0).into(),
        ..Default::default()
    })
    .insert(ColliderPositionSync::Discrete)
    .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
    .insert(Slayer);
}

fn setup_two_player_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("snake_head.png"),
        ..Default::default()
    });
}

fn cleanup_game(
    mut commands: Commands,
    entities: Query<Entity, Without<Camera>>
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

const SPEED: f32 = 300.0;
fn slayer_controls(
    mut state: ResMut<State<AppState>>,
    input: Res<Input<KeyCode>>,
    rapier_parameters: ResMut<RapierConfiguration>,
    mut slayer_info: Query<&mut RigidBodyVelocityComponent, With<Slayer>>,
) {
    for mut rb_vels in slayer_info.iter_mut() {
        let mut direction = Vector2::zeros();
        if input.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }
        if input.pressed(KeyCode::W) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::S) {
            direction.y -= 1.0;
        }
        if input.pressed(KeyCode::P) || input.pressed(KeyCode::Escape) {
            state.push(AppState::PauseMenu).unwrap();
        }

        if direction != Vector2::zeros() {
            direction /= direction.magnitude() * rapier_parameters.scale;
            rb_vels.linvel = direction * SPEED;
        }

    }
}

#[derive(Component)]
struct AnimationTimer(Timer);

fn slayer_animator(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);
const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

const ARENA_HEIGHT: u32 = 14;
const ARENA_WIDTH: u32 = 28;

#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
pub enum SnakeAction {
    Input,
    Movement,
    Eating,
    Growth,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct SnakeSize {
    width: f32,
    height: f32,
}
impl SnakeSize {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

struct SnakeDeathEvent;
struct SlayerDeathEvent;
struct SnakeSplitEvent;
struct GameOverEvent;

#[derive(Default)]
struct LastTailPosition(Option<Position>);

#[derive(Component)]
struct SnakeSegment;

#[derive(Default)]
struct SnakeSegments(Vec<Entity>);

struct SnakeTimer(Timer);

impl SnakeTimer {
    pub fn new() -> Self {
        Self (Timer::from_seconds(0.15, true))
    }
}

impl Default for SnakeTimer {
    fn default() -> Self {
        Self::new()
    }
}
// #[derive(Default)]
// struct Game {
//     snakes: Vec<Vec<Entity>>,
//     snake_movement_timer: Timer,
// };





#[derive(Component)]
struct Food;

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    segments.0 = vec![
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_HEAD_COLOR,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(SnakeHead {
                direction: Direction::Right,
            })
            .insert(SnakeSegment)
            .insert(Position { x: 8, y: 9 })
            .insert(SnakeSize::square(0.8))
            .id(),
        spawn_segment(commands, Position { x: 7, y: 9 }),
    ];
}

fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(SnakeSize::square(0.65))
        .id()
}


fn snake_movement(
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_over_writer: EventWriter<GameOverEvent>,
    mut snake_split_writer: EventWriter<SnakeSplitEvent>,
    segments: ResMut<SnakeSegments>,
    mut snake_timer: ResMut<SnakeTimer>,
    time: Res<Time>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
) {
    snake_timer.0.tick(time.delta());
    if snake_timer.0.just_finished() {
        if let Some((head_entity, head)) = heads.iter_mut().next() {
            let segment_positions = segments
                .0
                .iter()
                .map(|e| *positions.get_mut(*e).unwrap())
                .collect::<Vec<Position>>();
            let mut head_pos = positions.get_mut(head_entity).unwrap();
            match &head.direction {
                Direction::Left => {
                    head_pos.x -= 1;
                }
                Direction::Right => {
                    head_pos.x += 1;
                }
                Direction::Up => {
                    head_pos.y += 1;
                }
                Direction::Down => {
                    head_pos.y -= 1;
                }
            };
            if head_pos.x < 0
                || head_pos.y < 0
                || head_pos.x as u32 >= ARENA_WIDTH
                || head_pos.y as u32 >= ARENA_HEIGHT
            {
                game_over_writer.send(GameOverEvent);
            }
            if segment_positions.contains(&head_pos) {
                snake_split_writer.send(SnakeSplitEvent);
            }
            segment_positions
                .iter()
                .zip(segments.0.iter().skip(1))
                .for_each(|(pos, segment)| {
                    *positions.get_mut(*segment).unwrap() = *pos;
                });
            last_tail_position.0 = Some(*segment_positions.last().unwrap());
        }
    }
}

fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
) {
    if reader.iter().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake(commands, segments_res);
    }
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&SnakeSize, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}