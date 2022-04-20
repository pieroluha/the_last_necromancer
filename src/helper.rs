use crate::prelude::*;

pub fn look_at_player(pos: &Vec3) -> Vec3 {
    let x = ARENA_OFFSET - pos.x;
    let y = ARENA_OFFSET - pos.y;
    Vec3::new(x, y, 0.0).normalize()
}
