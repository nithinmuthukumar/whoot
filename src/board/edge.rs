use bevy::utils::HashSet;

use super::axial::Axial;

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
