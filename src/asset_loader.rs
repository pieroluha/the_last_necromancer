use crate::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use std::time::Duration;

#[derive(AssetCollection)]
pub struct ImageAssets {
    #[asset(path = "icon.png")]
    pub icon: Handle<Image>,
    #[asset(path = "images/player_platform400x400.png")]
    pub platform: Handle<Image>,
    #[asset(texture_atlas(
        tile_size_x = 64.0,
        tile_size_y = 64.0,
        columns = 8,
        rows = 8,
        //padding_y = 0.3,
        //padding_x = 0.3
    ))]
    #[asset(path = "images/fireball.png")]
    pub fireball: Handle<TextureAtlas>,
    #[asset(texture_atlas(
        tile_size_x = 32.0,
        tile_size_y = 32.0,
        columns = 1,
        rows = 4,
        //padding_y = 0.3,
        //padding_x = 0.3
    ))]
    #[asset(path = "images/player.png")]
    pub player: Handle<TextureAtlas>,
}

pub struct AnimationHandles {
    pub animate_fireball: Handle<SpriteSheetAnimation>,
    pub animate_player: Handle<SpriteSheetAnimation>,
}

fn setup_animation_handles(
    mut commands: Commands,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
) {
    let animate_fireball = animations.add(SpriteSheetAnimation::from_range(
        0..=60,
        Duration::from_millis(100),
    ));

    let animate_player = animations.add(SpriteSheetAnimation::from_range(
        0..=3,
        Duration::from_millis(100),
    ));

    commands.insert_resource(AnimationHandles {
        animate_fireball,
        animate_player,
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
