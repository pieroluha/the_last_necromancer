use crate::prelude::*;

pub fn look_at_player(pos: &Vec3) -> Vec3 {
    let x = ARENA_OFFSET - pos.x;
    let y = ARENA_OFFSET - pos.y;
    Vec3::new(x, y, 0.0).normalize()
}

pub fn look_at(pos: &Vec3, target: &Vec3) -> Vec3 {
    let x = target.x - pos.x;
    let y = target.y - pos.y;
    Vec3::new(x, y, 0.0).normalize()
}

pub fn get_world_pos(transform: &Transform) -> Vec2 {
    let pos = transform.translation.truncate();
    let multiple = CELL_SIZE as i32;
    let x = pos.x as i32;
    let y = pos.y as i32;
    let x = ((x + multiple - 1) & -multiple) as f32;
    let y = ((y + multiple - 1) & -multiple) as f32;

    Vec2::new(x, y)
}

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
