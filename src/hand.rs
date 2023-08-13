use bevy::{input::mouse::MouseButtonInput, math::Vec2Swizzles, prelude::*, window::PrimaryWindow};
use leafwing_input_manager::{
    action_state,
    prelude::{ActionState, InputManagerPlugin, InputMap},
    Actionlike, InputManagerBundle,
};

use crate::{
    camera::MainCamera,
    card::{Card, Draggable, Ordinal},
    deck::DeckAction,
    GameState,
};

#[derive(Component)]
pub struct Hand {
    pub size: usize,
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
            .add_system(position_cards.in_set(OnUpdate(GameState::Playing)))
            .add_system(select_card.in_set(OnUpdate(GameState::Playing)));
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
    q_hand: Query<(&Hand, &Children)>,
    mut q_cards: Query<(&Card, &mut Transform, &Ordinal)>,
) {
    if q_hand.is_empty() {
        return;
    }
    let (hand, children) = q_hand.single();
    let arc_length = 180.0;
    let rotation_factor = 30.; // Adjust the rotation factor as desired

    let width = (hand.size * 80).clamp(0, 600);

    for &child in children.iter() {
        if let Ok((card, mut transform, ord)) = q_cards.get_mut(child) {
            let angle = (ord.0 as f32 / (hand.size as f32)) * arc_length;
            let x = ord.0 as f32 / hand.size as f32 * width as f32;
            let y = angle.to_radians().sin() * 40.0; // Calculate y position along the arc
            let rot = ord.0 as f32 / hand.size as f32 * rotation_factor - rotation_factor / 2.;

            transform.translation.x = x - 300.;
            transform.translation.y = y;
            transform.translation.z = ord.0 as f32;
            transform.rotation = Quat::from_rotation_z(-rot.to_radians());
        }
    }
}
fn select_card(
    mut query: Query<(&ActionState<HandAction>, &mut Hand, &mut Children)>,
    mut q_window: Query<&Window, With<PrimaryWindow>>,
    mut q_cards: Query<(&Card, &mut Transform, &Ordinal, &Draggable)>,
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
                for &child in children.iter() {
                    if let Ok((card, transform, ord, draggable)) = q_cards.get(child) {
                        println!("{:?}", ord.0);

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
                            println!("Cursor is hovering over the sprite.");
                        }
                    }
                }
            }
        }

        println!("selected");
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
