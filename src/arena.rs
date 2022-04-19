use crate::prelude::*;

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
