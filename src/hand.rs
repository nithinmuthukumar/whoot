use bevy::{input::mouse::MouseButtonInput, prelude::*};
use leafwing_input_manager::{
    prelude::{ActionState, InputManagerPlugin, InputMap},
    Actionlike, InputManagerBundle,
};

use crate::{card::Card, deck::DeckAction, GameState};

#[derive(Component)]
pub struct Hand {
    pub size: u32,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum HandAction {
    Select,
}

pub struct HandPlugin;

impl Plugin for HandPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<HandAction>::default())
            .add_system(spawn_hand.in_schedule(OnEnter(GameState::Playing)))
            .add_system(position_cards.in_set(OnUpdate(GameState::Playing)));
    }
}

//spawn deck when deck plugin is made
fn spawn_hand(mut commands: Commands) {
    commands
        .spawn((
            InputManagerBundle::<HandAction> {
                action_state: ActionState::default(),
                input_map: InputMap::new([(MouseButton::Left, HandAction::Select)]),
            },
            SpatialBundle::default(),
        ))
        .insert(Hand { size: 0 });
}
//switch from children to giving each card an in hand component with a position
//spawned card will get the next index possible in hand
//be more abstract define deck as a zone, hand is a zone that is spread out
fn position_cards(q_hand: Query<(&Hand, &Children)>, mut q_cards: Query<(&Card, &mut Transform)>) {
    if q_hand.is_empty() {
        return;
    }
    let (hand, children) = q_hand.single();
    let mut ind = 0;
    let radius = 20.;
    let angle_increment = 180. / (hand.size - 1) as f32;

    for &child in children.iter() {
        if let Ok((card, mut transform)) = q_cards.get_mut(child) {
            let angle = (angle_increment * ind as f32).to_radians();
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            transform.translation.x = x * 20.;
            transform.translation.y = y * 20.;
            ind += 1;
        }
    }
}
