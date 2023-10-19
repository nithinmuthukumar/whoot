use std::{cmp::Ordering, ops::Add};

use bevy::prelude::{Vec2, Vec3};
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Axial {
    q: i32,
    r: i32,
}

impl Axial {
    pub fn new(q: i32, r: i32) -> Self {
        Axial { q, r }
    }

    pub fn get_cartesian(&self) -> Vec3 {
        let scale_x = 60.;
        let scale_y = 60.;
        let l = Vec2::new(
            f32::cos(std::f32::consts::PI / 6.0),
            f32::sin(std::f32::consts::PI / 6.0),
        );

        // Line 1
        let a2: Vec2 = l + Vec2::new(-l.y, l.x) * self.q as f32;

        let m = f32::tan(std::f32::consts::PI / 6.0);
        let y = m * (self.r as f32 - a2.x) + a2.y;

        // scale is the size of the hex from the center to the edge
        Vec3::new(self.r as f32 * scale_x, y * scale_y, 0.)
    }

    pub fn get_axial(world_pos: Vec3) -> Self {
        let result = Vec2::new(world_pos.x / 0.5, world_pos.z / 0.5);
        let m = f32::tan(std::f32::consts::PI / 6.0);
        let l = Vec2::new(
            f32::cos(std::f32::consts::PI / 6.0),
            f32::sin(std::f32::consts::PI / 6.0),
        );
        let cx = (result.y - l.y - m * result.x + m * l.x) / (-l.y * m - l.x * m);
        Axial::new(cx as i32, result.x as i32)
    }
}
impl Add<Axial> for Axial {
    type Output = Axial;

    fn add(self, other: Axial) -> Axial {
        Axial::new(self.q + other.q, self.r + other.r)
    }
}
impl PartialOrd for Axial {
    fn partial_cmp(&self, other: &Axial) -> Option<Ordering> {
        if self.q < other.q || (self.q == other.q && self.r < other.r) {
            Some(Ordering::Less)
        } else if self == other {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl Ord for Axial {
    fn cmp(&self, other: &Axial) -> Ordering {
        self.partial_cmp(other).unwrap() // Unwrap the Option to get the Ordering
    }
}
