use bevy::prelude::*;
use leafwing_input_manager::{prelude::InputManagerPlugin, Actionlike};

use crate::{loading::TextureAssets, GameState};

#[derive(Component)]
pub struct Card;

#[derive(Bundle)]
pub struct CardBundle {
    pub card: Card,
    pub sprite: SpriteBundle,
    pub ordinal: Ordinal,
}
#[derive(Component)]
pub struct Pickable {
    pub selected: bool,
    pub hovered: bool,
    pub target: Option<Vec2>,
}
impl Default for Pickable {
    fn default() -> Self {
        Self {
            selected: false,
            hovered: false,
            target: None,
        }
    }
}
#[derive(Component)]
pub struct Ordinal(pub usize);

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {}
}

//system for dragging cards
//card moving to hand
