mod axial;
pub mod board;
pub mod edge;
pub mod hex;
pub mod vertex;
use bevy::prelude::*;

use self::board::Board;
use crate::{loading::TextureAssets, GameState};

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_board)
            .insert_resource(Board::new());
        // .add_systems(
        //     Update,
        //     (position_cards, draw_card).run_if(in_state(GameState::Playing)),
        // )
        // .add_plugins(InputManagerPlugin::<DeckAction>::default());
    }
}
pub fn spawn_board(board: Res<Board>, mut cmd: Commands, textures: Res<TextureAssets>) {
    for pos in board.hexes.keys() {
        cmd.spawn(SpriteBundle {
            texture: textures.ore_tile.clone(),
            transform: Transform {
                translation: pos.get_cartesian(),
                ..default()
            },
            ..default()
        });
    }
}
