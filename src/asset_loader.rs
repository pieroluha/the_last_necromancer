use crate::enemy::EnemyType;
use crate::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use std::time::Duration;

pub const HIT_SIZE: Vec2 = const_vec2!([16.0, 16.0]);

#[derive(AssetCollection)]
pub struct FontHandles {
    #[asset(path = "fonts/DungeonFont.ttf")]
    pub dungeon_font: Handle<Font>,
}

// Should have broken this into chunks
#[derive(AssetCollection)]
pub struct ImageHandles {
    #[asset(path = "icon.png")]
    pub icon: Handle<Image>,

    #[asset(path = "images/arena.png")]
    pub arena: Handle<Image>,

    #[asset(path = "images/won.png")]
    pub win: Handle<Image>,

    #[asset(path = "images/died.png")]
    pub lose: Handle<Image>,

    #[asset(path = "images/arena_shadow.png")]
    pub arena_shadow: Handle<Image>,

    #[asset(path = "images/heart0.png")]
    pub full_heart: Handle<Image>,

    #[asset(path = "images/mana0.png")]
    pub full_mana: Handle<Image>,

    #[asset(path = "images/mage_icon.png")]
    pub mage_icon: Handle<Image>,

    #[asset(path = "images/archer_icon.png")]
    pub archer_icon: Handle<Image>,

    #[asset(path = "images/spell_icon.png")]
    pub spell_icon: Handle<Image>,

    #[asset(texture_atlas(tile_size_x = 64.0, tile_size_y = 64.0, columns = 8, rows = 8,))]
    #[asset(path = "images/fireball.png")]
    pub fireball: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 63.0, tile_size_y = 32.0, columns = 4, rows = 1,))]
    #[asset(path = "images/pulse.png")]
    pub pulse: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 16.0, tile_size_y = 16.0, columns = 4, rows = 1,))]
    #[asset(path = "images/bolt.png")]
    pub bolt: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 63.0, tile_size_y = 32.0, columns = 5, rows = 1,))]
    #[asset(path = "images/spark.png")]
    pub spark: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 95.0, tile_size_y = 32.0, columns = 4, rows = 1,))]
    #[asset(path = "images/waveform.png")]
    pub waveform: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 5, rows = 1,))]
    #[asset(path = "images/hits_01.png")]
    pub hits_01: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 7, rows = 1,))]
    #[asset(path = "images/hits_02.png")]
    pub hits_02: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 5, rows = 1,))]
    #[asset(path = "images/hits_03.png")]
    pub hits_03: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 7, rows = 1,))]
    #[asset(path = "images/hits_04.png")]
    pub hits_04: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 16.0, tile_size_y = 16.0, columns = 6, rows = 1,))]
    #[asset(path = "images/skull_buster.png")]
    pub skull_buster: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 64.0, tile_size_y = 64.0, columns = 5, rows = 1,))]
    #[asset(path = "images/dark_edge.png")]
    pub dark_edge: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 2, rows = 2,))]
    #[asset(path = "images/stupid.png")]
    pub stupid: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 13.0, tile_size_y = 16.0, columns = 10, rows = 1,))]
    #[asset(path = "images/arrow.png")]
    pub arrow: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 8, rows = 4,))]
    #[asset(path = "images/selected.png")]
    pub selected: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 4, rows = 1,))]
    #[asset(path = "images/player.png")]
    pub player: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 128.0, tile_size_y = 128.0, columns = 4, rows = 1,))]
    #[asset(path = "images/shield.png")]
    pub shield: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 16.0, tile_size_y = 28.0, columns = 4, rows = 4,))]
    #[asset(path = "images/enemies.png")]
    pub enemies: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 16.0, tile_size_y = 24.0, columns = 4, rows = 2,))]
    #[asset(path = "images/chortle.png")]
    pub demon: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 16.0, tile_size_y = 16.0, columns = 4, rows = 2,))]
    #[asset(path = "images/skelly.png")]
    pub skeleton: Handle<TextureAtlas>,
}

impl ImageHandles {
    pub fn get_projectile(
        &self,
        animation_handles: &AnimationHandles,
    ) -> (Handle<TextureAtlas>, Vec2, Handle<SpriteSheetAnimation>) {
        let i = fastrand::u32(0..4);
        match i {
            0 => (
                self.pulse.clone(),
                (Vec2::new(63.0, 32.0) * 0.5),
                animation_handles.idle_player.clone(),
            ),
            1 => (
                self.bolt.clone(),
                Vec2::new(16.0, 16.0),
                animation_handles.idle_player.clone(),
            ),
            2 => (
                self.spark.clone(),
                (Vec2::new(63.0, 32.0) * 0.5),
                animation_handles.dark_edge.clone(),
            ),
            3 => (
                self.waveform.clone(),
                (Vec2::new(95.0, 32.0) * 0.5),
                animation_handles.idle_player.clone(),
            ),
            _ => (
                self.pulse.clone(),
                (Vec2::new(63.0, 32.0) * 0.5),
                animation_handles.idle_player.clone(),
            ),
        }
    }

    pub fn get_hit(
        &self,
        animation_handles: &AnimationHandles,
    ) -> (Handle<TextureAtlas>, Handle<SpriteSheetAnimation>) {
        let i = fastrand::u32(0..4);
        match i {
            0 => (self.hits_01.clone(), animation_handles.five.clone()),
            1 => (self.hits_02.clone(), animation_handles.seven.clone()),
            2 => (self.hits_03.clone(), animation_handles.five.clone()),
            3 => (self.hits_04.clone(), animation_handles.seven.clone()),
            _ => (self.hits_01.clone(), animation_handles.five.clone()),
        }
    }
}

pub struct AnimationHandles {
    pub fireball: Handle<SpriteSheetAnimation>,
    pub five: Handle<SpriteSheetAnimation>,
    pub seven: Handle<SpriteSheetAnimation>,
    pub arrow_fly: Handle<SpriteSheetAnimation>,
    pub arrow_break: Handle<SpriteSheetAnimation>,
    pub idle_player: Handle<SpriteSheetAnimation>,
    pub idle_mage_f: Handle<SpriteSheetAnimation>,
    pub idle_mage_m: Handle<SpriteSheetAnimation>,
    pub idle_elf_f: Handle<SpriteSheetAnimation>,
    pub idle_elf_m: Handle<SpriteSheetAnimation>,
    pub demon_idle: Handle<SpriteSheetAnimation>,
    pub demon_run: Handle<SpriteSheetAnimation>,
    pub skeleton_idle: Handle<SpriteSheetAnimation>,
    pub skeleton_run: Handle<SpriteSheetAnimation>,
    pub dark_edge: Handle<SpriteSheetAnimation>,
    pub skull_buster: Handle<SpriteSheetAnimation>,
}

impl AnimationHandles {
    pub fn enemy_sprite(&self, enemy_type: EnemyType) -> &Handle<SpriteSheetAnimation> {
        match enemy_type {
            EnemyType::Mage(is_alt) => {
                if !is_alt {
                    &self.idle_mage_f
                } else {
                    &self.idle_mage_m
                }
            }
            EnemyType::Archer(is_alt) => {
                if !is_alt {
                    &self.idle_elf_f
                } else {
                    &self.idle_elf_m
                }
            }
        }
    }
}

fn setup_animation_handles(
    mut commands: Commands,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
) {
    let fireball = animations.add(SpriteSheetAnimation::from_range(
        0..=60,
        Duration::from_millis(100),
    ));

    let five =
        animations.add(SpriteSheetAnimation::from_range(0..=4, Duration::from_millis(100)).once());

    let seven =
        animations.add(SpriteSheetAnimation::from_range(0..=6, Duration::from_millis(100)).once());

    let dark_edge = animations.add(SpriteSheetAnimation::from_range(
        0..=4,
        Duration::from_millis(100),
    ));

    let skull_buster = animations.add(SpriteSheetAnimation::from_range(
        0..=5,
        Duration::from_millis(100),
    ));

    let idle_player = animations.add(SpriteSheetAnimation::from_range(
        0..=3,
        Duration::from_millis(100),
    ));

    let idle_mage_f = animations.add(SpriteSheetAnimation::from_range(
        0..=3,
        Duration::from_millis(100),
    ));

    let idle_mage_m = animations.add(SpriteSheetAnimation::from_range(
        4..=7,
        Duration::from_millis(100),
    ));

    let idle_elf_f = animations.add(SpriteSheetAnimation::from_range(
        8..=11,
        Duration::from_millis(100),
    ));

    let idle_elf_m = animations.add(SpriteSheetAnimation::from_range(
        12..=15,
        Duration::from_millis(100),
    ));

    let arrow_fly = animations.add(SpriteSheetAnimation::from_range(
        0..=5,
        Duration::from_millis(100),
    ));

    let arrow_break = animations.add(SpriteSheetAnimation::from_range(
        6..=9,
        Duration::from_millis(100),
    ));

    let demon_idle = animations.add(SpriteSheetAnimation::from_range(
        0..=3,
        Duration::from_millis(100),
    ));

    let demon_run = animations.add(SpriteSheetAnimation::from_range(
        4..=7,
        Duration::from_millis(200),
    ));

    let skeleton_idle = animations.add(SpriteSheetAnimation::from_range(
        0..=3,
        Duration::from_millis(100),
    ));

    let skeleton_run = animations.add(SpriteSheetAnimation::from_range(
        4..=7,
        Duration::from_millis(100),
    ));

    commands.insert_resource(AnimationHandles {
        fireball,
        five,
        seven,
        arrow_fly,
        arrow_break,
        idle_player,
        idle_mage_f,
        idle_mage_m,
        idle_elf_f,
        idle_elf_m,
        demon_idle,
        demon_run,
        skeleton_idle,
        skeleton_run,
        dark_edge,
        skull_buster,
    });
}

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AssetLoad).with_system(setup_animation_handles));
        AssetLoader::new(AssetLoad)
            .continue_to_state(Playing)
            .with_collection::<ImageHandles>()
            .with_collection::<FontHandles>()
            .build(app);
    }
}
