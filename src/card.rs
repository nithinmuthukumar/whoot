use bevy::prelude::*;
use leafwing_input_manager::{prelude::InputManagerPlugin, Actionlike};

use crate::{loading::TextureAssets, GameState};

#[derive(Component)]
pub struct Card;

#[derive(Bundle)]
pub struct CardBundle {
    pub card: Card,
    #[bundle]
    pub sprite: SpriteBundle,
    pub ordinal: Ordinal,
}
#[derive(Component)]
pub struct Draggable {
    pub selected: bool,
}
#[derive(Component)]
pub struct Ordinal(pub usize);

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {}
}

//system for dragging cards
//card moving to hand
