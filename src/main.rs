use bevy::prelude::*;
use bevy::window::WindowMode;
use unfair_advantage_lib::*;

fn main() {
    App::new()
        // Configure the game window
        .insert_resource(WindowDescriptor {
            width: 1920.0,
            height: 1080.0,
            title: "Unfair Snek".to_string(),
            mode: WindowMode::Windowed,
            scale_factor_override: Some(1.0),
            ..Default::default()
        })
        // Standard Bevy functionality
        .add_plugins(DefaultPlugins)
        // Add plugins here
        .add_plugin(UnfairAdvantagePlugin)
        .run();
}
