use crate::prelude::*;

#[derive(Component)]
pub struct Player;

fn spawn_player(image_assets: Res<ImageAssets>, animation_handles: Res<AnimationHandles>, mut commands: Commands) {

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: image_assets.player.clone(),
            transform: Transform::from_xyz(ARENA_OFFSET, ARENA_OFFSET, 2.0),
            ..default()
        })
        .insert(Player)
        .insert(animation_handles.idle_player.clone())
        .insert(Play);
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(Playing).with_system(spawn_player));
    }
}
