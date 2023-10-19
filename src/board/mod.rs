mod axial;
pub mod board_data;
pub mod edge;
pub mod hex;
pub mod pickable_pos;
pub mod vertex;
use std::time::Duration;

use bevy::{prelude::*, render::view::RenderLayers, window::PrimaryWindow};
use bevy_tweening::{
    lens::TransformScaleLens, Animator, EaseFunction, RepeatCount, RepeatStrategy, Tween,
};
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    Actionlike, InputManagerBundle,
};

use self::{board_data::BoardData, pickable_pos::PickablePos};
use crate::{
    camera::BoardCamera,
    deck::DeckAction,
    hand::HandAction,
    loading::TextureAssets,
    utils::{calculate_rotated_bounds, point_in_polygon},
    AppState,
};
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Setup,
    Roll,
    Turn,
}
#[derive(Component)]
pub struct Board;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Playing),
            (spawn_board, spawn_settlement_coord_pickers),
        )
        .add_systems(Update, select_picker.run_if(in_state(AppState::Playing)))
        .insert_resource(BoardData::new());
        // .add_systems(
        //     Update,
        //     (position_cards, draw_card).run_if(in_state(GameState::Playing)),
        // )
        // .add_plugins(InputManagerPlugin::<DeckAction>::default());
    }
}
pub fn spawn_board(board: Res<BoardData>, mut cmd: Commands, textures: Res<TextureAssets>) {
    for pos in board.hexes.keys() {
        let t = match board.hexes[pos].resource_type {
            hex::Resource::None => textures.desert_tile.clone(),
            hex::Resource::Ore => textures.ore_tile.clone(),
            hex::Resource::Wheat => textures.wheat_tile.clone(),
            hex::Resource::Sheep => textures.sheep_tile.clone(),
            hex::Resource::Brick => textures.brick_tile.clone(),
            hex::Resource::Wood => textures.wood_tile.clone(),
        };
        cmd.spawn((
            SpriteBundle {
                texture: t,
                transform: Transform {
                    translation: pos.get_cartesian(),
                    ..default()
                },
                ..default()
            },
            RenderLayers::layer(1),
        ));
    }
}
pub fn spawn_settlement_coord_pickers(
    board: Res<BoardData>,
    mut cmd: Commands,
    textures: Res<TextureAssets>,
) {
    for c in board.get_valid_settlement_coords() {
        let entity = cmd
            .spawn((
                SpriteBundle {
                    texture: textures.pickable_pos.clone(),
                    transform: Transform {
                        translation: c.pos.get_cartesian() + Vec3::Z,
                        ..default()
                    },
                    ..default()
                },
                PickablePos::new(c.pos),
                RenderLayers::layer(1),
            ))
            .id();
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_millis(2000),
            TransformScaleLens {
                start: Vec3::new(0.75, 0.75, 1.),
                end: Vec3::new(1.25, 1.25, 1.),
            },
        )
        .with_repeat_count(RepeatCount::Infinite)
        .with_repeat_strategy(RepeatStrategy::MirroredRepeat);
        cmd.entity(entity).insert(Animator::new(tween));
    }
}

pub fn select_picker(
    mut cmd: Commands,
    mut q: Query<&ActionState<HandAction>>,
    mut pickers: Query<(Entity, &PickablePos, &Transform)>,
    mut q_window: Query<&Window, With<PrimaryWindow>>,
    mut q_camera: Query<(&Camera, &GlobalTransform), With<BoardCamera>>,
) {
    let a = q.single();

    if a.just_pressed(HandAction::Select) {
        if let Some(pos) = q_window.single().cursor_position() {
            let (camera, camera_transform) = q_camera.single();
            if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, pos) {
                for (entity, picker, transform) in pickers.iter() {
                    let half_width = 18.;
                    let half_height = 18.;
                    let rotated_bounds =
                        calculate_rotated_bounds(transform, half_width, half_height);

                    if point_in_polygon(world_pos, &rotated_bounds) {
                        cmd.entity(entity).remove::<Animator<Transform>>();
                        // max_ord = Some(ord.0);
                        // hovered_entity = Some(entity);
                    }
                }
            }
        }
    }
}
// fn create_ui(mut cmd: Commands){
//     commands.spawn(NodeBundle{
//         style:Style{
//             size:Siz
//         }
//     })
// }
