use bevy::utils::HashSet;

use super::axial::Axial;

#[derive(Debug)]
pub struct Edge {
    pub path_coords: EdgeCoords,
    pub path_type: EdgeType,
}

#[derive(Debug)]
pub enum EdgeType {
    Road,
    None,
}

impl Edge {
    pub fn new(path_coords: EdgeCoords, path_type: EdgeType) -> Self {
        Edge {
            path_coords,
            path_type,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct EdgeCoords {
    a: Axial,
    b: Axial,
}
impl EdgeCoords {
    pub fn new(a: Axial, b: Axial) -> EdgeCoords {
        // Ensure that UnorderedPairCoords is always ordered lexicographically
        if a < b {
            EdgeCoords { a, b }
        } else {
            EdgeCoords { b, a }
        }
    }
    pub fn contains(&self, c: Axial) -> bool {
        self.a == c || self.b == c
    }
}
