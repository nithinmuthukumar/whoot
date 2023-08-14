use bevy::prelude::*;
use leafwing_input_manager::{
    prelude::{ActionState, InputManagerPlugin, InputMap},
    Actionlike, InputManagerBundle,
};

use crate::{
    card::{Card, CardBundle, Ordinal, Pickable},
    hand::{Hand, UpdatePosition},
    loading::TextureAssets,
    GameState,
};

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum DeckAction {
    Draw,
}

#[derive(Component)]
pub struct Deck {
    size: usize,
}

pub struct DeckPlugin;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_deck)
            .add_systems(
                Update,
                (position_cards, draw_card).run_if(in_state(GameState::Playing)),
            )
            .add_plugins(InputManagerPlugin::<DeckAction>::default());
    }
}

//spawn deck when deck plugin is made
fn spawn_deck(mut cmd: Commands, textures: Res<TextureAssets>) {
    let deck_id = cmd
        .spawn((
            InputManagerBundle::<DeckAction> {
                action_state: ActionState::default(),
                input_map: InputMap::new([(KeyCode::Space, DeckAction::Draw)]),
            },
            Deck { size: 60 },
            SpatialBundle {
                transform: Transform {
                    translation: Vec3::new(-400., -150., 0.),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    for i in 0..60 {
        let card_id = cmd
            .spawn(CardBundle {
                card: Card,
                sprite: SpriteBundle {
                    texture: textures.card_blue.clone(),
                    ..default()
                },
                ordinal: Ordinal(i),
            })
            .id();
        cmd.entity(deck_id).push_children(&[card_id]);
    }
}
//switch from children to giving each card an in hand component with a position
//spawned card will get the next index possible in hand
//be more abstract define deck as a zone, hand is a zone that is spread out
fn position_cards(
    q_deck: Query<(&Transform, &Deck, &Children)>,
    mut q_cards: Query<(&Card, &mut Transform), Without<Deck>>,
) {
    if q_deck.is_empty() {
        return;
    }

    let (deck_t, deck, children) = q_deck.single();
    let mut i = 1.;

    for &child in children.iter() {
        if let Ok((card, mut transform)) = q_cards.get_mut(child) {
            transform.translation.y = deck_t.translation.y + i * 0.5;
            transform.translation.z = i;
        }
        i += 1.;
    }
}

fn draw_card(
    mut cmd: Commands,
    mut query: Query<(
        &ActionState<DeckAction>,
        &Transform,
        &mut Deck,
        &mut Children,
    )>,
    mut q_cards: Query<&Card>,
    mut hand: Query<(Entity, &mut Hand)>,
    mut writer: EventWriter<UpdatePosition>,
) {
    let (action_state, transform, mut deck, children) = query.single_mut();

    if action_state.just_pressed(DeckAction::Draw) {
        let (entity, mut hand) = hand.single_mut();
        if let Some(drawn) = children.last() {
            cmd.entity(*drawn).remove_parent();
            cmd.entity(*drawn).insert(Pickable::default());
            cmd.entity(*drawn).insert(Transform::clone(transform));
            cmd.entity(*drawn).insert(Ordinal(hand.size));
            deck.size -= 1;
            cmd.entity(entity).push_children(&[*drawn]);
            hand.size += 1;
            writer.send(UpdatePosition {});
        };

        println!("Created");
    }
}
