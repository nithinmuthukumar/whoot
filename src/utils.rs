use bevy::prelude::{Transform, Vec2, Vec3};

pub fn calculate_rotated_bounds(
    transform: &Transform,
    half_width: f32,
    half_height: f32,
) -> [Vec2; 4] {
    let transform_matrix = transform.compute_matrix();
    let rotated_corner_1 =
        transform_matrix.transform_point3(Vec3::new(-half_width, -half_height, 0.0));
    let rotated_corner_2 =
        transform_matrix.transform_point3(Vec3::new(half_width, -half_height, 0.0));
    let rotated_corner_3 =
        transform_matrix.transform_point3(Vec3::new(half_width, half_height, 0.0));
    let rotated_corner_4 =
        transform_matrix.transform_point3(Vec3::new(-half_width, half_height, 0.0));

    [
        Vec2::new(rotated_corner_1.x, rotated_corner_1.y),
        Vec2::new(rotated_corner_2.x, rotated_corner_2.y),
        Vec2::new(rotated_corner_3.x, rotated_corner_3.y),
        Vec2::new(rotated_corner_4.x, rotated_corner_4.y),
    ]
}

pub fn point_in_polygon(point: Vec2, polygon: &[Vec2]) -> bool {
    let n = polygon.len();
    let mut inside = false;

    for i in 0..n {
        let j = (i + 1) % n;

        let intersect = ((polygon[i].y > point.y) != (polygon[j].y > point.y))
            && (point.x
                < (polygon[j].x - polygon[i].x) * (point.y - polygon[i].y)
                    / (polygon[j].y - polygon[i].y)
                    + polygon[i].x);

        if intersect {
            inside = !inside;
        }
    }

    inside
}
