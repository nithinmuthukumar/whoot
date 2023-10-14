use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use lazy_static::lazy_static;

use super::{axial::Axial, edge::Edge, hex::*, vertex::Vertex};

lazy_static! {
    static ref OFFSETS: [Axial; 6] = [
        Axial::new(1, 0),
        Axial::new(0, 1),
        Axial::new(-1, 1),
        Axial::new(-1, 0),
        Axial::new(0, -1),
        Axial::new(1, -1),
    ];
}
#[derive(Resource)]
pub struct Board {
    pub hexes: HashMap<Axial, Hex>,
    pub edges: HashMap<HashSet<Axial>, Edge>,
    pub vertices: HashMap<Axial, Vertex>,
    pub robber: Axial,
}
impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            hexes: HashMap::new(),
            edges: HashMap::new(),
            vertices: HashMap::new(),
            robber: Axial::new(0, 0),
        };
        board.hexes.insert(
            Axial::new(4, -2),
            Hex {
                pos: Axial::new(4, -2),
                number: 10,
                resource_type: super::hex::Resource::Ore,
            },
        );
        board.hexes.insert(
            Axial::new(3, 0),
            Hex {
                pos: Axial::new(3, 0),
                number: 2,
                resource_type: super::hex::Resource::Wood,
            },
        );
        board.hexes.insert(
            Axial::new(2, 2),
            Hex {
                pos: Axial::new(2, 2),
                number: 2,
                resource_type: super::hex::Resource::Wood,
            },
        );
        board.hexes.insert(
            Axial::new(3, -3),
            Hex {
                pos: Axial::new(3, -3),
                number: 2,
                resource_type: super::hex::Resource::Wood,
            },
        );
        board.hexes.insert(
            Axial::new(0, 0),
            Hex {
                pos: Axial::new(0, 0),
                number: 2,
                resource_type: super::hex::Resource::Wood,
            },
        );

        board
    }
}
