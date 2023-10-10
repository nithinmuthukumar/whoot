use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

use super::axial::Axial;

static OFFSETS: [Axial; 6] = [
    Axial::new(1.0, 0.0),
    Axial::new(0.0, 1.0),
    Axial::new(-1.0, 1.0),
    Axial::new(-1.0, 0.0),
    Axial::new(0.0, -1.0),
    Axial::new(1.0, -1.0),
];
#[derive(Component)]
pub struct Board {
    pub hexes: HashMap<Axial, Hex>,
    pub edges: HashMap<HashSet<Axial>, Edge>,
    pub vertices: HashMap<Axial, Vertex>,
    pub robber: Axial,
}
#[derive(Debug)]
pub struct Edge {
    pub path_coords: HashSet<Axial>,
    pub path_type: EdgeType,
}

#[derive(Debug)]
pub enum EdgeType {
    Road,
    None,
}

impl Edge {
    pub fn new(path_coords: HashSet<Axial>, path_type: EdgeType) -> Self {
        Edge {
            path_coords,
            path_type,
        }
    }
}
pub struct Vertex {
    pos: Axial,
    build_type: BuildType,
    owner: i32,
}
impl Vertex {
    pub fn new(pos: Axial, build_type: BuildType, owner: i32) -> Self {
        Vertex {
            pos,
            build_type,
            owner,
        }
    }
}
pub struct Hex {
    pub resource_type: Resource,
    pub number: i32,
    pub pos: Axial,
}

#[derive(Debug)]
pub enum BuildType {
    City,
    Settlement,
    None,
}
pub enum Resource {
    None,
    Ore,
    Wheat,
    Sheep,
    Brick,
    Wood,
}
