use std::{f32::INFINITY, time::Duration};

use bevy::{input::mouse::MouseButtonInput, math::Vec2Swizzles, prelude::*, window::PrimaryWindow};
use bevy_tweening::{
    lens::{TransformRotationLens, TransformScaleLens},
    *,
};
use leafwing_input_manager::{
    action_state,
    prelude::{ActionState, InputManagerPlugin, InputMap},
    Actionlike, InputManagerBundle,
};

use crate::{
    camera::{lerp, MainCamera},
    card::{Card, Ordinal},
    deck::{draw_card, DeckAction},
    GameState,
};

#[derive(Component)]
pub struct Hand {
    pub size: usize,
    pub selected: Option<Entity>,
    pub hovered: Option<Entity>,
}
#[derive(Event)]
pub struct UpdateHand {}
impl Default for UpdateHand {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum HandAction {
    Select,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TransformLens {
    /// Start value.
    pub start: Transform,
    /// End value.
    pub end: Transform,
}

impl Lens<Transform> for TransformLens {
    fn lerp(&mut self, target: &mut Transform, ratio: f32) {
        //rotation
        target.rotation = self.start.rotation.slerp(self.end.rotation, ratio);
        //position
        let value =
            self.start.translation + (self.end.translation - self.start.translation) * ratio;
        target.scale = self.start.scale.lerp(self.end.scale, ratio);

        target.translation = value;
    }
}

pub struct HandPlugin;

impl Plugin for HandPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<HandAction>::default())
            .add_systems(OnEnter(GameState::Playing), spawn_hand)
            .add_systems(Update, component_animator_system::<Transform>)
            .add_systems(
                Update,
                (position_cards.before(draw_card), select_card, pickable_lerp)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_event::<UpdateHand>();
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
        .insert(Hand {
            size: 0,
            selected: None,
            hovered: None,
        });
}
//whenever hand is updated position cards in hand that are not selected by ord using a tween
fn position_cards(
    mut cmd: Commands,
    q_hand: Query<(&Hand, &Children)>,
    mut q_cards: Query<(Entity, &Card, &mut Transform, &Ordinal)>,
) {
    if q_hand.is_empty() {
        return;
    }

    let (hand, children) = q_hand.single();
    let arc_length = 180.0;
    let rotation_factor = 30.; // Adjust the rotation factor as desired

    let width = (hand.size * 80).clamp(0, 600);

    for &child in children.iter() {
        if let Ok((entity, card, mut transform, ord)) = q_cards.get_mut(child) {
            if hand.selected == Some(entity) {
                return;
            }
            let angle = (ord.0 as f32 / (hand.size as f32)) * arc_length;
            let x = ord.0 as f32 / hand.size as f32 * width as f32 - 300.;
            let y = angle.to_radians().sin() * 40.0; // Calculate y position along the arc
            let rot = ord.0 as f32 / hand.size as f32 * rotation_factor - rotation_factor / 2.;
            transform.translation.x = transform.translation.x.lerp(&x, &0.2);
            transform.translation.y = transform.translation.y.lerp(&y, &0.2);
            transform.translation.z = ord.0 as f32;
            transform.rotation = transform
                .rotation
                .lerp(Quat::from_rotation_z(-rot.to_radians()), 0.2);
        }
    }
}
//whenever a card is selected move it toward the target
fn pickable_lerp(
    mut q_hand: Query<&Hand>,
    mut q_cards: Query<(Entity, &Card, &mut Transform)>,
    mut q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut q_window: Query<&Window, With<PrimaryWindow>>,
) {
    if let Some(selected) = q_hand.single().selected {
        if let Some(pos) = q_window.single().cursor_position() {
            let (camera, camera_transform) = q_camera.single();
            if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, pos) {
                if let Ok((entity, card, mut transform)) = q_cards.get_mut(selected) {
                    transform.translation.x = transform.translation.x.lerp(&world_pos.x, &0.2);
                    transform.translation.y = transform.translation.y.lerp(&world_pos.y, &0.2);
                }
            }
        }
    }
}

fn select_card(
    mut cmd: Commands,
    mut writer: EventWriter<UpdateHand>,
    mut query: Query<(&ActionState<HandAction>, &mut Hand, &mut Children)>,
    mut q_window: Query<&Window, With<PrimaryWindow>>,
    mut q_cards: Query<(Entity, &Card, &Transform, &Ordinal)>,
    mut q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if query.is_empty() {
        return;
    }

    let (action_state, mut hand, children) = query.single_mut();
    let mut max_ord = None;
    let mut hovered_entity = None;

    if hand.selected.is_none() {
        if let Some(pos) = q_window.single().cursor_position() {
            let (camera, camera_transform) = q_camera.single();
            if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, pos) {
                for &child in children.iter() {
                    //get the topmost hovered card
                    if let Ok((entity, card, transform, ord)) = q_cards.get_mut(child) {
                        if let Some(s) = max_ord {
                            if s > ord.0 {
                                continue;
                            }
                        }
                        //card is 140,190
                        let rotation = transform.rotation.to_axis_angle().1;
                        let half_width = 70.;
                        let half_height = 95.;
                        let rotated_bounds =
                            calculate_rotated_bounds(transform, half_width, half_height);

                        if point_in_polygon(world_pos, &rotated_bounds) {
                            max_ord = Some(ord.0);
                            hovered_entity = Some(entity);
                        }
                    }
                }
            }
        }
        if hovered_entity != hand.hovered {
            if let Some(h) = hand.hovered {
                if let Ok((entity, card, transform, ord)) = q_cards.get(h) {
                    let tween = Tween::new(
                        EaseFunction::QuadraticInOut,
                        Duration::from_millis(100),
                        TransformScaleLens {
                            start: transform.scale,
                            end: Vec3::new(1., 1., 1.),
                        },
                    );

                    cmd.entity(hand.hovered.unwrap())
                        .insert(Animator::new(tween));
                }
            }

            hand.hovered = hovered_entity;
            if let Some(h) = hand.hovered {
                if let Ok((entity, card, transform, ord)) = q_cards.get(h) {
                    let tween = Tween::new(
                        EaseFunction::QuadraticInOut,
                        Duration::from_millis(100),
                        TransformScaleLens {
                            start: transform.scale,
                            end: Vec3::new(1.1, 1.1, 1.),
                        },
                    );
                    cmd.entity(entity).insert(Animator::new(tween));
                }
            }
        }
        if action_state.just_pressed(HandAction::Select) && hand.hovered.is_some() {
            hand.selected = hand.hovered;

            if let Ok((entity, card, transform, ord)) = q_cards.get(hand.selected.unwrap()) {
                //straigten the card
                let tween = Tween::new(
                    EaseFunction::QuadraticInOut,
                    Duration::from_millis(250),
                    TransformRotationLens {
                        start: transform.rotation,
                        end: Quat::IDENTITY,
                    },
                );
                println!("straighten");
                cmd.entity(entity).insert(Animator::new(tween));
            }
        }
    }

    let select_released = action_state.just_released(HandAction::Select);
    if select_released {
        hand.selected = None;
    }
}
fn calculate_rotated_bounds(transform: &Transform, half_width: f32, half_height: f32) -> [Vec2; 4] {
    let transform_matrix = transform.compute_matrix();
    let rotated_corner_1 =
        transform_matrix.transform_point3(Vec3::new(-half_width, -half_height, 0.0));
    let rotated_corner_2 =
        transform_matrix.transform_point3(Vec3::new(half_width, -half_height, 0.0));
    let rotated_corner_3 =
        transform_matrix.transform_point3(Vec3::new(half_width, half_height, 0.0));
    let rotated_corner_4 =
        transform_matrix.transform_point3(Vec3::new(-half_width, half_height, 0.0));

    [
        Vec2::new(rotated_corner_1.x, rotated_corner_1.y),
        Vec2::new(rotated_corner_2.x, rotated_corner_2.y),
        Vec2::new(rotated_corner_3.x, rotated_corner_3.y),
        Vec2::new(rotated_corner_4.x, rotated_corner_4.y),
    ]
}

fn point_in_polygon(point: Vec2, polygon: &[Vec2]) -> bool {
    let n = polygon.len();
    let mut inside = false;

    for i in 0..n {
        let j = (i + 1) % n;

        let intersect = ((polygon[i].y > point.y) != (polygon[j].y > point.y))
            && (point.x
                < (polygon[j].x - polygon[i].x) * (point.y - polygon[i].y)
                    / (polygon[j].y - polygon[i].y)
                    + polygon[i].x);

        if intersect {
            inside = !inside;
        }
    }

    inside
}
