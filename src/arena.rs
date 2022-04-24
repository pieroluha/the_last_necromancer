use crate::prelude::*;

pub const ARENA_WORLD_SIZE: f32 = 400.0;
pub const ARENA_SIZE: f32 = ARENA_WORLD_SIZE / CELL_SIZE;
pub const ARENA_GRID_SIZE: usize = (ARENA_SIZE * ARENA_SIZE) as usize;
pub const ARENA_OFFSET: f32 = ARENA_WORLD_SIZE / 2.0;

fn setup_arena(image_handles: Res<ImageHandles>, mut commands: Commands) {
    let transform = Transform::from_xyz(ARENA_OFFSET, ARENA_OFFSET, 0.0);
    commands
        .spawn_bundle(SpriteBundle {
            texture: image_handles.platform.clone(),
            transform,
            ..default()
        })
        .insert(Name::new("Arena"));
}

pub struct ArenaPlugin;
impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(Playing).with_system(setup_arena));
    }
}
