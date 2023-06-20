#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use whoot::{GamePlugin, GameState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Whoot".to_string(),
                resolution: (1920., 1080.).into(),
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(GamePlugin)
        .run();
}
