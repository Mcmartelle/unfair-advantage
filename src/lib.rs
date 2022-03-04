// #![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::doc_markdown)]
#![doc = include_str!("../README.md")]


use bevy::app::AppExit;
use bevy::core::Time;
use bevy::prelude::*;
use heron::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};

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
            .with_system(slayer_death)
                // .with_system(size_scaling)
        )
        .add_system_set(SystemSet::on_exit(AppState::InOnePlayerGame).with_system(cleanup_game))
        .add_system_set(SystemSet::on_enter(AppState::InTwoPlayerGame).with_system(setup_two_player_game))
        .add_system_set(
            SystemSet::on_update(AppState::InTwoPlayerGame)
            // .with_system(slayer_controls)
            .with_system(slayer_animator)
        )
        .add_system_set(SystemSet::on_exit(AppState::InTwoPlayerGame).with_system(cleanup_game))
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(AudioPlugin)
        .insert_resource(Gravity::from(Vec3::new(0.0, -300.0, 0.0)));
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

fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    // ui camera
    audio.play_looped(asset_server.load("music/main_menu_theme.ogg"));
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
    audio: Res<Audio>
) {
    audio.stop();
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
    audio: Res<Audio>
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    audio.play_looped(asset_server.load("music/game_theme.ogg"));
    let slayer_texture_handle = asset_server.load("slayer_run.png");
    let slayer_texture_atlas = TextureAtlas::from_grid(slayer_texture_handle, Vec2::new(64.0, 64.0), 8, 1);
    let slayer_texture_atlas_handle = texture_atlases.add(slayer_texture_atlas);

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("snake_den.png"),
        ..Default::default()
    });

    // The Ground
    commands.spawn_bundle((
        Transform::from_translation(Vec3::new(0.0, -500.0, 0.0)),
        GlobalTransform::default(),
        RigidBody::Static,
        CollisionShape::Cuboid {
            half_extends: Vec2::new(1500.0, 50.0).extend(0.0) / 2.0,
            border_radius: None,
        },
    ));

    let slayer_size = Vec2::new(64.0, 64.0);
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: slayer_texture_atlas_handle,
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        ..Default::default()
    })
    .insert(AnimationTimer(Timer::from_seconds(0.04, true)))
    .insert(AttackCooldown(Timer::from_seconds(0.6, false)))
    .insert(Facing::Right)
    .insert(SwordDirection::NotAttacking)
    .insert(FeetState::InAir)
    .insert(SlayerAnim::Jump)
    .insert(RigidBody::Dynamic)
    .insert(CollisionShape::Cuboid {
        half_extends: slayer_size.extend(0.0) / 2.0,
        border_radius: None,
    })
    .insert(RotationConstraints::lock())
    .insert(Velocity::default())
    .insert(CollisionLayers::new(Layer::Slayer, Layer::SnakeHead))
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
    entities: Query<Entity, Without<Camera>>,
    audio: Res<Audio>
) {
    audio.stop();
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

const SPEED: f32 = 300.0;
fn slayer_controls(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    input: Res<Input<KeyCode>>,
    mut slayer_info: Query<
        (Entity,
        &mut Velocity,
        &Facing,
        &AttackCooldown),
        With<Slayer>
    >,
) {
    for (entity, mut velocity, facing, attack_cooldown) in slayer_info.iter_mut() {
        let x = if input.pressed(KeyCode::A) {
            -1.0
        } else if input.pressed(KeyCode::D) {
            1.0
        } else {
            0.0
        };
    
        let y = if input.pressed(KeyCode::S) {
            -1.0
        } else if input.pressed(KeyCode::W) {
            1.0
        } else {
            0.0
        };

        if input.just_pressed(KeyCode::B) && attack_cooldown.0.finished() { // Attack button
            if x > 0.0 {
                commands.entity(entity).insert(SwordDirection::Right);
            } else if x < 0.0 {
                commands.entity(entity).insert(SwordDirection::Left);
            } else if y > 0.0 {
                commands.entity(entity).insert(SwordDirection::Up);
            } else if y < 0.0 {
                commands.entity(entity).insert(SwordDirection::Down);
            } else {
                let _sword_direction = match facing {
                    Facing::Left => SwordDirection::Left,
                    Facing::Right => SwordDirection::Right,
                };
                commands.entity(entity).insert(_sword_direction);
            }
        } else if attack_cooldown.0.finished() {
            commands.entity(entity).insert(SwordDirection::NotAttacking);
        }
        if input.pressed(KeyCode::P) || input.pressed(KeyCode::Escape) {
            state.push(AppState::PauseMenu).unwrap();
        }

        let target_velocity = Vec2::new(x, y).normalize_or_zero() * SPEED;

        if target_velocity != Vec2::ZERO {
            velocity.linear = target_velocity.extend(0.0);
        }
    }
}

#[derive(Component)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct AttackCooldown(Timer);

// fn slayer_anim_selector(
//     commands: Commands,
//     texture_atlases: Res<Assets<TextureAtlas>>,
//     mut query: Query<(
//         Entity,
//         &Facing,
//         &FeetState,
//         &mut SlayerAnim,
//     )>
// ) {

// }

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

fn slayer_death (
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    events
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();
            if is_slayer(layers_1) && is_snake_head(layers_2) {
                Some(entity_1)
            } else if is_slayer(layers_2) && is_snake_head(layers_1) {
                Some(entity_2)
            } else {
                None
            }
        })
        .for_each(|slayer_entity| {
            // audio.play(asset_server.load("sfx/slayer_death.ogg"));
            audio.play(asset_server.load("sfx/snake_chomp.ogg"));
            commands.entity(slayer_entity).despawn()
        });
}

fn is_slayer(layers: CollisionLayers) -> bool {
    layers.contains_group(Layer::Slayer) && !layers.contains_group(Layer::SnakeHead)
}

fn is_snake_head(layers: CollisionLayers) -> bool {
    !layers.contains_group(Layer::Slayer) && layers.contains_group(Layer::SnakeHead)
}

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
const GROUND_COLOR: Color = Color::rgb(0.3, 0.8, 0.3);

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

#[derive(Component)]
struct SnakeSegment {
    direction: Direction,
}

struct SnakeDeathEvent;
struct SlayerDeathEvent;
struct SnakeSplitEvent;
struct GameOverEvent;

#[derive(Default)]
struct LastTailPosition(Option<Position>);

#[derive(Default)]
struct SnakeSegments(Vec<Entity>);

struct SnakeTimer(Timer);

impl SnakeTimer {
    pub fn new() -> Self {
        Self (Timer::from_seconds(0.4, true))
    }
}

impl Default for SnakeTimer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Component)]
enum Facing {
    Left,
    Right,
}

#[derive(Component)]
enum SwordDirection {
    Up,
    Down,
    Left,
    Right,
    NotAttacking,
}



#[derive(Component)]
enum FeetState {
    OnGround,
    InAir,
}

#[derive(Component)]
enum SlayerAnim {
    Idle,
    Run,
    Jump,
    AttackForward,
    AttackDown,
    AttackUp,
}

#[derive(Default)]
struct Game {
    snakes: Vec<Vec<Entity>>,
    snake_movement_timer: Timer,
    lives: u8,
}

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

#[derive(PhysicsLayer)]
enum Layer {
    Slayer,
    SnakeHead,
}

fn spawn_snake(
    mut commands: Commands,
    mut segments: ResMut<SnakeSegments>,
    asset_server: Res<AssetServer>
) {
    let snake_sprite_size = Vec2::new(64.0, 64.0);
    segments.0 = vec![
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("snake_head.png"),
                ..Default::default()
            })
            .insert(SnakeHead {
                direction: Direction::Right,
            })
            .insert(Position { x: 8, y: 9 })
            .insert(RigidBody::KinematicPositionBased)
            .insert(CollisionShape::Cuboid {
                half_extends: snake_sprite_size.extend(0.0) / 2.0,
                border_radius: None,
            })
            .insert(CollisionLayers::new(Layer::SnakeHead, Layer::Slayer))
            .id(),
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    flip_x: true,
                    ..Default::default()
                },
                texture: asset_server.load("snake_section.png"),
                ..Default::default()
            })
            .insert(SnakeSegment {
                direction: Direction::Right,
            })
            .insert(Position { x: 7, y: 9 })
            .insert(RigidBody::KinematicPositionBased)
            .insert(CollisionShape::Cuboid {
                half_extends: snake_sprite_size.extend(0.0) / 2.0,
                border_radius: None,
            })
            .id(),
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("snake_section.png"),
                ..Default::default()
            })
            .insert(SnakeSegment {
                direction: Direction::Right,
            })
            .insert(Position { x: 6, y: 9 })
            .insert(RigidBody::KinematicPositionBased)
            .insert(CollisionShape::Cuboid {
                half_extends: snake_sprite_size.extend(0.0) / 2.0,
                border_radius: None,
            })
            .id(),
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("snake_section.png"),
                ..Default::default()
            })
            .insert(SnakeSegment {
                direction: Direction::Right,
            })
            .insert(Position { x: 5, y: 9 })
            .insert(RigidBody::KinematicPositionBased)
            .insert(CollisionShape::Cuboid {
                half_extends: snake_sprite_size.extend(0.0) / 2.0,
                border_radius: None,
            })
            .id(),
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("snake_section.png"),
                ..Default::default()
            })
            .insert(SnakeSegment {
                direction: Direction::Right,
            })
            .insert(Position { x: 4, y: 9 })
            .insert(RigidBody::KinematicPositionBased)
            .insert(CollisionShape::Cuboid {
                half_extends: snake_sprite_size.extend(0.0) / 2.0,
                border_radius: None,
            })
            .id(),
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("snake_section.png"),
                ..Default::default()
            })
            .insert(SnakeSegment {
                direction: Direction::Right,
            })
            .insert(Position { x: 3, y: 9 })
            .insert(RigidBody::KinematicPositionBased)
            .insert(CollisionShape::Cuboid {
                half_extends: snake_sprite_size.extend(0.0) / 2.0,
                border_radius: None,
            })
            .id(),
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("snake_section.png"),
                ..Default::default()
            })
            .insert(SnakeSegment {
                direction: Direction::Right,
            })
            .insert(Position { x: 2, y: 9 })
            .insert(RigidBody::KinematicPositionBased)
            .insert(CollisionShape::Cuboid {
                half_extends: snake_sprite_size.extend(0.0) / 2.0,
                border_radius: None,
            })
            .id(),
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("snake_tail.png"),
                ..Default::default()
            })
            .insert(SnakeSegment {
                direction: Direction::Right,
            })
            .insert(Position { x: 2, y: 8 })
            .insert(RigidBody::KinematicPositionBased)
            .insert(CollisionShape::Cuboid {
                half_extends: snake_sprite_size.extend(0.0) / 2.0,
                border_radius: None,
            })
            .id(),
    ];
}

struct SnakeFlipFlop {
    flip_x: bool
}

impl Default for SnakeFlipFlop {
    fn default() -> Self {
        Self { flip_x: true }
    }
}

fn snake_movement(
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_over_writer: EventWriter<GameOverEvent>,
    mut snake_split_writer: EventWriter<SnakeSplitEvent>,
    segments: ResMut<SnakeSegments>,
    mut snake_timer: ResMut<SnakeTimer>,
    time: Res<Time>,
    mut heads: Query<(Entity, &SnakeHead, &mut Transform, &mut Sprite)>,
    mut positions: Query<&mut Position>,
    mut snake_segments: Query<&mut SnakeSegment>,
    mut snake_flip_flop: Local<SnakeFlipFlop>,
) {
    snake_timer.0.tick(time.delta());
    if snake_timer.0.just_finished() {
        if let Some((head_entity, head, mut head_transform, mut head_sprite)) = heads.iter_mut().next() {
            let segment_positions = segments
                .0
                .iter()
                .map(|e| *positions.get_mut(*e).unwrap())
                .collect::<Vec<Position>>();
            // let segment_directions = segments
            //     .0
            //     .iter()
            //     .map(|e| *snake_segments.get_mut(*e).unwrap())
            //     .collect::<Vec<SnakeSegment>>();
            let mut head_pos = positions.get_mut(head_entity).unwrap();
            match &head.direction {
                Direction::Left => {
                    head_pos.x -= 1;
                    head_transform.rotation = Quat::from_rotation_z(f32::to_radians(270.0));
                }
                Direction::Right => {
                    head_pos.x += 1;
                    head_transform.rotation = Quat::from_rotation_z(f32::to_radians(90.0));
                }
                Direction::Up => {
                    head_pos.y += 1;
                    head_transform.rotation = Quat::from_rotation_z(f32::to_radians(180.0));
                }
                Direction::Down => {
                    head_pos.y -= 1;
                    head_transform.rotation = Quat::from_rotation_z(f32::to_radians(0.0));
                }
            };
            head_sprite.flip_x = snake_flip_flop.flip_x;
            snake_flip_flop.flip_x = !snake_flip_flop.flip_x;
            if head_pos.x < 0 {
                head_pos.x = ARENA_WIDTH as i32 - 1;
            } else if head_pos.x as u32 >= ARENA_WIDTH {
                head_pos.x = 0;
            } else if head_pos.y < 0 {
                head_pos.y = ARENA_HEIGHT as i32 - 1;
            } else if head_pos.y as u32 >= ARENA_HEIGHT {
                head_pos.y = 0;
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
    asset_server: Res<AssetServer>,
) {
    if reader.iter().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake(commands, segments_res, asset_server);
    }
}

// fn size_scaling(windows: Res<Windows>, mut q: Query<(&SnakeSize, &mut Transform)>) {
//     let window = windows.get_primary().unwrap();
//     for (sprite_size, mut transform) in q.iter_mut() {
//         transform.scale = Vec3::new(
//             sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
//             sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
//             1.0,
//         );
//     }
// }

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_game: f32) -> f32 {
        let tile_size = 64.0;
        let bound_window = tile_size * bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, ARENA_HEIGHT as f32),
            2.0,
        );
    }
}