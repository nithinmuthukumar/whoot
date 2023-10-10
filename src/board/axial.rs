#[derive(Debug, Clone, Copy)]
pub struct Axial {
    c: Vec2,
}

impl Axial {
    pub fn new(q: f32, r: f32) -> Self {
        Axial { c: Vec2::new(q, r) }
    }

    pub fn get_cartesian(&self) -> Vec3 {
        let l = Vec2::new(
            f32::cos(std::f32::consts::PI / 6.0),
            f32::sin(std::f32::consts::PI / 6.0),
        );

        // Line 1
        let a2 = l + l.yx() * self.c.x;

        let m = f32::tan(std::f32::consts::PI / 6.0);
        let z = m * (self.c.y - a2.x) + a2.y;

        // 0.5 is the size of the hex from the center to the edge
        Vec3::new(self.c.y * 0.5, z * 0.5, 0.0)
    }

    pub fn get_world_position(world_pos: Vec3) -> Self {
        let mut result = Axial::new(world_pos.x / 0.5, world_pos.y / 0.5);
        let m = f32::tan(std::f32::consts::PI / 6.0);
        let l = Vec2::new(
            f32::cos(std::f32::consts::PI / 6.0),
            f32::sin(std::f32::consts::PI / 6.0),
        );
        let cx = (result.c.y - l.y - m * result.c.x + m * l.x) / (l.yx().y - m * l.yx().x);
        result.c.x = cx;
        result
    }
}
