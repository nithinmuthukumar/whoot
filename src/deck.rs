use bevy::prelude::*;
use leafwing_input_manager::{
    prelude::{ActionState, InputManagerPlugin, InputMap},
    Actionlike, InputManagerBundle,
};

use crate::{
    card::{Card, CardBundle},
    hand::Hand,
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
            .add_system(draw_card.in_set(OnUpdate(GameState::Playing)))
            .add_plugin(InputManagerPlugin::<DeckAction>::default());
    }
}

//spawn deck when deck plugin is made
fn spawn_deck(mut commands: Commands, textures: Res<TextureAssets>) {
    println!("DECK SPAWNED");
    commands.spawn((
        InputManagerBundle::<DeckAction> {
            action_state: ActionState::default(),
            input_map: InputMap::new([(KeyCode::Space, DeckAction::Draw)]),
        },
        Deck { cards: 60 },
        SpriteBundle {
            texture: textures.card_blue.clone(),
            transform: Transform {
                translation: Vec3::new(-400., 200., 0.),
                ..default()
            },
            ..default()
        },
    ));
}
fn draw_card(
    mut cmd: Commands,
    textures: Res<TextureAssets>,
    mut query: Query<(&ActionState<DeckAction>, &Transform, &mut Deck)>,
    mut hand: Query<(Entity, &mut Hand)>,
) {
    let (action_state, transform, mut deck) = query.single_mut();

    if action_state.just_pressed(DeckAction::Draw) {
        let (entity, mut hand) = hand.single_mut();

        deck.cards -= 1;
        let mut card_transform = transform.clone();
        card_transform.translation.z = 1.;

        let card_id = cmd
            .spawn(CardBundle {
                card: Card,
                sprite: SpriteBundle {
                    texture: textures.card_red.clone(),
                    transform: card_transform,
                    ..default()
                },
            })
            .id();
        cmd.entity(entity).push_children(&[card_id]);
        hand.size += 1;
        println!("Created");
    }
}
