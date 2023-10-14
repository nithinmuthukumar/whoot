use super::{axial::Axial, hex::BuildType};

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
