mod camera;
mod card;
mod deck;
mod loading;
use bevy::prelude::*;
use camera::CameraPlugin;
use card::CardPlugin;
use deck::{DeckAction, DeckPlugin};
use leafwing_input_manager::prelude::*;
use loading::LoadingPlugin;

pub struct GamePlugin;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(CameraPlugin)
            .add_plugin(InputManagerPlugin::<DeckAction>::default())
            .add_plugin(LoadingPlugin)
            .add_plugin(CardPlugin)
            .add_plugin(DeckPlugin);
    }
}
