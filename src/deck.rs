use std::f32::consts::PI;

use bevy::prelude::*;
use leafwing_input_manager::{
    prelude::{ActionState, InputManagerPlugin, InputMap},
    Actionlike, InputManagerBundle,
};

use crate::{
    card::{Card, CardBundle, CardFace, FlipCard, Ordinal},
    hand::Hand,
    loading::TextureAssets,
    AppState,
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
        app.add_systems(OnEnter(AppState::Playing), spawn_deck)
            .add_systems(
                Update,
                (position_cards, draw_card).run_if(in_state(AppState::Playing)),
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

    //TODO make this a make_card func
    for i in 0..60 {
        let front = cmd
            .spawn((
                SpriteBundle {
                    texture: textures.card_king.clone(),
                    visibility: Visibility::Hidden,
                    transform: Transform {
                        rotation: Quat::from_euler(EulerRot::XYZ, 0., PI, 0.),
                        ..default()
                    },

                    ..default()
                },
                CardFace { is_front: true },
            ))
            .id();
        let back = cmd
            .spawn((
                SpriteBundle {
                    texture: textures.card_blue.clone(),
                    ..default()
                },
                CardFace { is_front: false },
            ))
            .id();

        let card_id = cmd
            .spawn(CardBundle {
                card: Card {
                    back,
                    front,
                    face_up: false,
                },
                sprite: SpriteBundle { ..default() },
                ordinal: Ordinal(i),
            })
            .id();

        cmd.entity(card_id).push_children(&[front, back]);
        cmd.entity(deck_id).push_children(&[card_id]);
    }
}
fn position_cards(
    q_deck: Query<(&Transform, &Deck, &Children)>,
    mut q_cards: Query<(&Ordinal, &Card, &mut Transform), Without<Deck>>,
) {
    let (deck_t, deck, children) = q_deck.single();

    for &child in children.iter() {
        if let Ok((ord, card, mut transform)) = q_cards.get_mut(child) {
            transform.translation.y = ord.0 as f32 * 0.5;
            transform.translation.z = ord.0 as f32;
        }
    }
}

pub fn draw_card(
    mut cmd: Commands,
    mut query: Query<
        (
            &ActionState<DeckAction>,
            &Transform,
            &mut Deck,
            &mut Children,
        ),
        Without<Card>,
    >,
    mut q_cards: Query<(&Card, &mut Ordinal, &mut Transform)>,
    mut hand: Query<(Entity, &mut Hand)>,
    mut flip_writer: EventWriter<FlipCard>,
) {
    let (action_state, deck_transform, mut deck, children) = query.single_mut();

    if action_state.just_pressed(DeckAction::Draw) {
        let (entity, mut hand) = hand.single_mut();
        for &child in children.iter() {
            if let Ok((card, mut ordinal, mut card_transform)) = q_cards.get_mut(child) {
                if ordinal.0 != deck.size - 1 {
                    continue;
                }
                cmd.entity(child).remove_parent();

                ordinal.0 = hand.size;
                card_transform.translation.x += deck_transform.translation.x;
                card_transform.translation.y += deck_transform.translation.y;

                deck.size -= 1;
                cmd.entity(entity).push_children(&[child]);
                hand.size += 1;
                flip_writer.send(FlipCard { card: child });

                return;
            }
        }
    }
}
