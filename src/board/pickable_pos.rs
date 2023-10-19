use bevy::prelude::Component;

use super::axial::Axial;

#[derive(Component)]
pub struct PickablePos {
    a: Axial,
    selected: bool,
}
impl PickablePos {
    pub fn new(a: Axial) -> Self {
        Self { a, selected: false }
    }
    // add code here
}
