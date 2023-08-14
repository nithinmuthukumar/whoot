use std::{f32::INFINITY, time::Duration};

use bevy::{input::mouse::MouseButtonInput, math::Vec2Swizzles, prelude::*, window::PrimaryWindow};
use bevy_tweening::{lens::TransformRotationLens, *};
use leafwing_input_manager::{
    action_state,
    prelude::{ActionState, InputManagerPlugin, InputMap},
    Actionlike, InputManagerBundle,
};

use crate::{
    camera::{lerp, MainCamera},
    card::{Card, Ordinal, Pickable},
    deck::DeckAction,
    GameState,
};

#[derive(Component)]
pub struct Hand {
    pub size: usize,
}
#[derive(Event)]
pub struct UpdatePosition {}

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
                (position_cards, select_card, drag_card, pickable_lerp)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_event::<UpdatePosition>();
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
fn position_cards(
    mut cmd: Commands,
    q_hand: Query<(&Hand, &Children)>,
    mut q_cards: Query<(Entity, &Card, &mut Transform, &Ordinal, &Pickable)>,
    mut event: EventReader<UpdatePosition>,
) {
    if event.is_empty() || q_hand.is_empty() {
        return;
    }
    event.clear();

    let (hand, children) = q_hand.single();
    let arc_length = 180.0;
    let rotation_factor = 30.; // Adjust the rotation factor as desired

    let width = (hand.size * 80).clamp(0, 600);

    for &child in children.iter() {
        if let Ok((entity, card, mut transform, ord, draggable)) = q_cards.get_mut(child) {
            if draggable.selected {
                return;
            }
            let angle = (ord.0 as f32 / (hand.size as f32)) * arc_length;
            let x = ord.0 as f32 / hand.size as f32 * width as f32;
            let y = angle.to_radians().sin() * 40.0; // Calculate y position along the arc
            let rot = ord.0 as f32 / hand.size as f32 * rotation_factor - rotation_factor / 2.;
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(250),
                TransformLens {
                    start: transform.clone(),
                    end: Transform {
                        translation: Vec3::new(x - 300., y, ord.0 as f32),
                        rotation: Quat::from_rotation_z(-rot.to_radians()),
                        ..default()
                    },
                },
            );
            cmd.entity(entity).insert(Animator::new(tween));

            // transform.translation.x = x - 300.;
            // transform.translation.y = y;
            // transform.translation.z = ord.0 as f32;
        }
    }
}
fn pickable_lerp(mut q_cards: Query<(&Card, &mut Transform, &mut Pickable)>) {
    for (card, mut transform, pickable) in q_cards.iter_mut() {
        if pickable.selected {
            if let Some(target) = pickable.target {
                transform.translation.x = transform.translation.x.lerp(&target.x, &0.2);
                transform.translation.y = transform.translation.y.lerp(&target.y, &0.2);
            }
        }
    }
}
fn drag_card(
    mut q_cards: Query<(&Card, &mut Transform, &mut Pickable)>,
    mut q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut q_window: Query<&Window, With<PrimaryWindow>>,
) {
    if let Some(pos) = q_window.single().cursor_position() {
        let (camera, camera_transform) = q_camera.single();
        if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, pos) {
            for (card, mut transform, mut draggable) in q_cards.iter_mut() {
                if !draggable.selected {
                    continue;
                }
                draggable.target = Some(world_pos);
            }
        }
    }
}
fn select_card(
    mut cmd: Commands,
    mut writer: EventWriter<UpdatePosition>,
    mut query: Query<(&ActionState<HandAction>, &mut Hand, &mut Children)>,
    mut q_window: Query<&Window, With<PrimaryWindow>>,
    mut q_cards: Query<(Entity, &Card, &mut Transform, &Ordinal, &mut Pickable)>,
    mut q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if query.is_empty() {
        return;
    }

    let (action_state, hand, children) = query.single_mut();
    if action_state.just_pressed(HandAction::Select) {
        if let Some(pos) = q_window.single().cursor_position() {
            let (camera, camera_transform) = q_camera.single();
            if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, pos) {
                let mut selected = None;
                for &child in children.iter() {
                    if let Ok((entity, card, transform, ord, mut draggable)) =
                        q_cards.get_mut(child)
                    {
                        if let Some(s) = selected {
                            if s > ord.0 {
                                continue;
                            }
                        }
                        //card is 140,190
                        let rotation = transform.rotation.to_axis_angle().1;
                        let half_width = 70.;
                        let half_height = 95.;

                        let transform_matrix = transform.compute_matrix();
                        let rotated_corner_1 = transform_matrix.transform_point3(Vec3::new(
                            -half_width,
                            -half_height,
                            0.0,
                        ));
                        let rotated_corner_2 = transform_matrix.transform_point3(Vec3::new(
                            half_width,
                            -half_height,
                            0.0,
                        ));
                        let rotated_corner_3 = transform_matrix.transform_point3(Vec3::new(
                            half_width,
                            half_height,
                            0.0,
                        ));
                        let rotated_corner_4 = transform_matrix.transform_point3(Vec3::new(
                            -half_width,
                            half_height,
                            0.0,
                        ));
                        let rotated_bounds = [
                            Vec2::new(rotated_corner_1.x, rotated_corner_1.y),
                            Vec2::new(rotated_corner_2.x, rotated_corner_2.y),
                            Vec2::new(rotated_corner_3.x, rotated_corner_3.y),
                            Vec2::new(rotated_corner_4.x, rotated_corner_4.y),
                        ];
                        //TODO LOOK AT Z INDEX TO MAKE FINAL CHOICE

                        if point_in_polygon(world_pos, &rotated_bounds) {
                            selected = Some(ord.0);
                        }
                    }
                }
                if let Some(s) = selected {
                    for (entity, card, transform, ord, mut draggable) in q_cards.iter_mut() {
                        if s == ord.0 {
                            draggable.selected = true;
                            let tween = Tween::new(
                                EaseFunction::QuadraticInOut,
                                Duration::from_millis(250),
                                TransformRotationLens {
                                    start: transform.rotation,
                                    end: Quat::IDENTITY,
                                },
                            );
                            cmd.entity(entity).insert(Animator::new(tween));
                        }
                    }
                }
            }
        }

        println!("selected");
    }
    if action_state.just_released(HandAction::Select) {
        for (entity, card, transform, ord, mut draggable) in q_cards.iter_mut() {
            draggable.selected = false;
        }
        writer.send(UpdatePosition {});
        println!("unselected");
    }
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
