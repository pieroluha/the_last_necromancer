use crate::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use std::time::Duration;

#[derive(AssetCollection)]
pub struct ImageAssets {
    #[asset(path = "icon.png")]
    pub icon: Handle<Image>,

    #[asset(path = "images/player_platform400x400.png")]
    pub platform: Handle<Image>,

    #[asset(texture_atlas(tile_size_x = 64.0, tile_size_y = 64.0, columns = 8, rows = 8,))]
    #[asset(path = "images/fireball.png")]
    pub fireball: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 4, rows = 1,))]
    #[asset(path = "images/player.png")]
    pub player: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 16.0, tile_size_y = 28.0, columns = 4, rows = 4,))]
    #[asset(path = "images/enemies.png")]
    pub enemies: Handle<TextureAtlas>,
}

pub struct AnimationHandles {
    pub fireball: Handle<SpriteSheetAnimation>,
    pub idle_player: Handle<SpriteSheetAnimation>,
    pub idle_enemy: Handle<SpriteSheetAnimation>,
}

fn setup_animation_handles(
    mut commands: Commands,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
) {
    let fireball = animations.add(SpriteSheetAnimation::from_range(
        0..=60,
        Duration::from_millis(100),
    ));

    let idle_player = animations.add(SpriteSheetAnimation::from_range(
        0..=3,
        Duration::from_millis(100),
    ));

    let idle_enemy = animations.add(SpriteSheetAnimation::from_range(
        0..=3,
        Duration::from_millis(100),
    ));

    commands.insert_resource(AnimationHandles {
        fireball,
        idle_player,
        idle_enemy,
    });
}

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AssetLoad).with_system(setup_animation_handles));
        AssetLoader::new(AssetLoad)
            .continue_to_state(Playing)
            .with_collection::<ImageAssets>()
            .build(app);
    }
}
