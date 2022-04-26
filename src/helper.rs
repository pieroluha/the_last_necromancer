use crate::prelude::*;

pub fn look_at_player(pos: &Vec3) -> Vec3 {
    (PLAYER_POS.truncate() - pos.truncate())
        .extend(0.0)
        .normalize()
}

pub fn look_at(pos: &Vec3, target: &Vec3) -> Vec3 {
    let x = target.x - pos.x;
    let y = target.y - pos.y;
    Vec3::new(x, y, 0.0).normalize()
}

// 90f32 offset for the arrows
pub fn heading(transform: &Transform, offset: f32) -> Transform {
    let mut transform = transform.clone();
    let vel = look_at_player(&transform.translation);
    let angle = vel.y.atan2(vel.x);
    let angle = angle - offset.to_radians();
    transform.rotation = Quat::from_rotation_z(angle);
    transform
}

use std::f32::consts::PI;
const MORE_DIRECTIONS: [(f32, f32); 8] = [
    (0.0, 16.0),
    (0.0, -16.0),
    (16.0, 0.0),
    (-16.0, 0.0),
    (16.0, 16.0),
    (-16.0, 16.0),
    (16.0, -16.0),
    (-16.0, -16.0),
];

pub fn spin_me_right_round() -> Vec<Vec3> {
    let mut projectiles = Vec::new();

    for pos in MORE_DIRECTIONS.iter() {
        let transform = PLAYER_POS + Vec3::new(pos.0, pos.1, 0.0);
        projectiles.push(transform);
    }

    projectiles
}

pub fn rotate2d(point: Vec3, angle: f32) -> Vec3 {
    let x = point.x * angle.cos() - point.y * angle.sin();
    let y = point.x * angle.sin() + point.y * angle.cos();
    Vec3::new(x, y, 2.0)
}

//pub fn get_world_pos(transform: &Transform) -> Vec2 {
//    let pos = transform.translation.truncate();
//    let multiple = CELL_SIZE as i32;
//    let x = pos.x as i32;
//    let y = pos.y as i32;
//    let x = ((x + multiple - 1) & -multiple) as f32;
//    let y = ((y + multiple - 1) & -multiple) as f32;
//
//    Vec2::new(x, y)
//}

pub fn world_pos_to_grid(transform: &Transform) -> IVec2 {
    let pos = transform.translation.truncate();
    let cell_size = CELL_SIZE as i32;
    let multiple = cell_size;
    let x = pos.x as i32;
    let y = pos.y as i32;
    let x = (x + multiple - 1) & -multiple;
    let y = (y + multiple - 1) & -multiple;

    IVec2::new(x >> 4, y >> 4)
}

pub fn vec_pos_to_grid(pos: &IVec2) -> IVec2 {
    IVec2::new(pos.x >> 4, pos.y >> 4)
}
