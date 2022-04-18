use crate::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Life(pub u8);
#[derive(Component, Deref, DerefMut)]
pub struct Mana(pub u32);
#[derive(Component, Deref, DerefMut)]
pub struct Speed(pub f32);
#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);
impl Velocity {
    pub fn look_at(&mut self, point: Vec2) {
        self.x = point.x - self.x;
        self.y = point.y - self.y;
        self.0 = self.normalize();
    }
}
