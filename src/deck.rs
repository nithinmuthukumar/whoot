use bevy::prelude::*;
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    Actionlike, InputManagerBundle,
};

use crate::{
    card::{Card, CardBundle},
    loading::TextureAssets,
    GameState,
};

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum DeckAction {
    Draw,
}

#[derive(Component)]
pub struct Deck {
    cards: u32,
}

pub struct DeckPlugin;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_deck.in_schedule(OnEnter(GameState::Playing)))
            .add_system(draw_card.in_set(OnUpdate(GameState::Playing)));
    }
}

//spawn deck when deck plugin is made
fn spawn_deck(mut commands: Commands) {
    println!("DECK SPAWNED");
    commands
        .spawn(InputManagerBundle::<DeckAction> {
            action_state: ActionState::default(),
            input_map: InputMap::new([(KeyCode::Space, DeckAction::Draw)]),
        })
        .insert(Deck { cards: 60 });
}
fn draw_card(
    mut cmd: Commands,
    textures: Res<TextureAssets>,
    mut query: Query<(&ActionState<DeckAction>, &mut Deck)>,
) {
    let (action_state, mut deck) = query.single_mut();

    if action_state.just_pressed(DeckAction::Draw) {
        cmd.spawn(CardBundle {
            card: Card,
            sprite: SpriteBundle {
                texture: textures.card_red.clone(),
                transform: Transform {
                    translation: Vec3::new(200., 100., 0.),
                    ..default()
                },
                ..default()
            },
        });
        println!("Created");
        deck.cards -= 1;
    }
}
