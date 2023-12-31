mod board;
mod camera;
mod card;
mod deck;
mod hand;
mod loading;
mod utils;
use bevy::prelude::*;
use board::BoardPlugin;
use camera::CameraPlugin;
use card::CardPlugin;
use deck::{DeckAction, DeckPlugin};
use hand::HandPlugin;
use leafwing_input_manager::prelude::*;
use loading::LoadingPlugin;

pub struct GamePlugin;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_plugin(CameraPlugin)
            .add_plugins((DeckPlugin, HandPlugin, CardPlugin))
            .add_plugins(LoadingPlugin)
            .add_plugins(BoardPlugin);
    }
}
