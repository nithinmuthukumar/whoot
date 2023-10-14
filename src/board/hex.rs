use super::axial::Axial;

#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub enum Resource {
    None,
    Ore,
    Wheat,
    Sheep,
    Brick,
    Wood,
}
