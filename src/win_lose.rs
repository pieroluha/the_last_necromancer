use crate::prelude::*;

fn display_win_message(
    mut commands: Commands,
    windows: Res<Windows>,
    image_handles: Res<ImageHandles>,
) {
    let window = windows.get_primary().unwrap();
    let width = window.height();
    let height = window.height();
    commands.spawn_bundle(SpriteBundle {
        texture: image_handles.win.clone(),
        sprite: Sprite {
            custom_size: Some(Vec2::new(width, height)),
            ..default()
        },
        ..default()
    });
}

fn display_lose_message(
    mut commands: Commands,
    windows: Res<Windows>,
    image_handles: Res<ImageHandles>,
) {
    let window = windows.get_primary().unwrap();
    let width = window.height();
    let height = window.height();
    commands.spawn_bundle(SpriteBundle {
        texture: image_handles.lose.clone(),
        sprite: Sprite {
            custom_size: Some(Vec2::new(width, height)),
            ..default()
        },
        ..default()
    });
}

pub struct WinLosePlugin;
impl Plugin for WinLosePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(Winner).with_system(display_win_message))
            .add_system_set(SystemSet::on_enter(GameOver).with_system(display_lose_message));
    }
}
