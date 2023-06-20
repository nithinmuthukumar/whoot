use bevy::prelude::*;

use crate::{loading::TextureAssets, GameState};

#[derive(Component)]
pub struct Card;

#[derive(Bundle)]
pub struct CardBundle {
    pub card: Card,
    #[bundle]
    pub sprite: SpriteBundle,
}

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {}
}
//system for dragging cards
//card moving to hand
