use bevy::prelude::*;
use leafwing_input_manager::{prelude::InputManagerPlugin, Actionlike};

use crate::{loading::TextureAssets, AppState};

#[derive(Component)]
pub struct Card {
    pub front: Entity,
    pub back: Entity,
    pub face_up: bool,
}
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum CardAction {
    Flip,
}
#[derive(Event)]
pub struct FlipCard {
    pub card: Entity,
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
#[derive(Component)]
pub struct Ordinal(pub usize);

#[derive(Component)]
pub struct Flipping {
    half: bool,
    rotation_speed: f32,
    current_rotation: f32,
}

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, flip_card.run_if(in_state(AppState::Playing)))
            .add_plugins(InputManagerPlugin::<CardAction>::default())
            .add_event::<FlipCard>();
    }
}
//TODO rotate in axis of rotation so the card flips not in y unless straight
pub fn flip_card(
    mut q_cards: Query<(Entity, &mut Card), Without<Flipping>>,
    mut q_flipping: Query<(Entity, &mut Card, &mut Flipping, &mut Transform)>,
    mut q_faces: Query<(&CardFace, &mut Visibility)>,
    mut flip_event: EventReader<FlipCard>,
    mut cmd: Commands,
    time: Res<Time>,
) {
    for e in flip_event.iter() {
        if let Ok((entity, mut card)) = q_cards.get_mut(e.card) {
            card.face_up = !card.face_up;
            cmd.entity(entity).insert(Flipping {
                half: false,
                rotation_speed: 400.0,
                current_rotation: 0.0,
            });
        }
    }
    for (entity, mut card, mut flipping, mut transform) in q_flipping.iter_mut() {
        let rotation_angle = flipping.rotation_speed * time.delta_seconds();
        flipping.current_rotation += rotation_angle;
        if flipping.current_rotation > 90. && !flipping.half {
            flipping.half = true;

            if let Ok((front, mut f_vis)) = q_faces.get_mut(card.front) {
                match card.face_up {
                    true => *f_vis = Visibility::Visible,
                    false => *f_vis = Visibility::Hidden,
                }
            }
            if let Ok((back, mut b_vis)) = q_faces.get_mut(card.back) {
                match card.face_up {
                    true => *b_vis = Visibility::Hidden,
                    false => *b_vis = Visibility::Visible,
                }
            }
        }
        if flipping.current_rotation >= 180.0 {
            flipping.current_rotation = 0.;

            cmd.entity(entity).remove::<Flipping>();
        }
        let rotation_quaternion = Quat::from_rotation_y(rotation_angle.to_radians());
        transform.rotate(rotation_quaternion);
    }
}
