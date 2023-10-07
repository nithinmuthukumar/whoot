use bevy::prelude::*;
use leafwing_input_manager::{prelude::InputManagerPlugin, Actionlike};

use crate::{loading::TextureAssets, GameState};

#[derive(Component)]
pub struct Card {
    pub front: Handle<Image>,
    pub back: Handle<Image>,
    pub face_up: bool,
}
// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum CardAction {
    Flip,
}

#[derive(Bundle)]
pub struct CardBundle {
    pub card: Card,
    pub sprite: SpriteBundle,
    pub ordinal: Ordinal,
}
#[derive(Component)]
pub struct CardFace {
    pub is_front: bool,
}
pub fn card_face(q_cards: Query<(&Visibility, &Card, &Children)>) {}
#[derive(Component)]
pub struct Ordinal(pub usize);

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (card_face).run_if(in_state(GameState::Playing)))
            .add_plugins(InputManagerPlugin::<CardAction>::default());
    }
}

//system for dragging cards
//card moving to hand
