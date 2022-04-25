use bevy::prelude::*;

#[cfg(feature = "dev")]
mod debug_panel;

mod arena;
mod asset_loader;
mod camera;
mod collision;
mod death;
mod enemy;
mod helper;
mod input;
mod minion;
mod pathfinding;
mod player;
mod projectiles;
mod selection_box;
mod ui;
mod prelude {
    pub use crate::arena::*;
    pub use crate::asset_loader::*;
    pub use crate::camera::*;
    pub use crate::collision::*;
    pub use crate::death::*;
    pub use crate::enemy::*;
    pub use crate::helper::*;
    pub use crate::input::*;
    pub use crate::minion::*;
    pub use crate::pathfinding::*;
    pub use crate::player::*;
    pub use crate::projectiles::*;
    pub use crate::selection_box::*;
    pub use crate::ui::*;
    pub use benimator::*;
    pub use bevy::prelude::*;

    #[cfg(feature = "dev")]
    pub use crate::debug_panel::*;

    #[derive(Clone, Eq, PartialEq, Debug, Hash)]
    pub enum GameState {
        AssetLoad,
        StartMenu,
        Playing,
        GameOver,
        Pause,
        Exit,
    }
    pub use Action::*;
    pub use GameState::*;

    #[derive(Default)]
    pub struct Wambo(pub u8);

    #[derive(Component, DerefMut, Deref)]
    pub struct Life(pub u8);
    #[derive(Component, DerefMut, Deref)]
    pub struct Mana(pub f32);
    #[derive(Component, DerefMut, Deref)]
    pub struct Speed(pub f32);

    pub const DEFAULT_WIN_WIDTH: f32 = 800.0;
    pub const DEFAULT_WIN_HEIGHT: f32 = 800.0;
    pub const BACKGROUND: Color = Color::rgb(34.0 / 255.0, 34.0 / 255.0, 34.0 / 255.0);
}

use prelude::*;

fn main() {
    let mut app = App::new();
    app.add_state(AssetLoad);

    app.insert_resource(ClearColor(BACKGROUND))
        .insert_resource(WindowDescriptor {
            title: "The Last Necromancer".to_string(),
            width: DEFAULT_WIN_WIDTH,
            height: DEFAULT_WIN_HEIGHT,
            resizable: true,
            ..default()
        });

    app.init_resource::<Wambo>();

    app.add_plugins(DefaultPlugins)
        .add_plugin(AssetLoaderPlugin)
        .add_plugin(AnimationPlugin::default())
        .add_plugin(MyCameraPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(ArenaPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(MinionPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(PathfindingPlugin)
        .add_plugin(SelectionBoxPlugin)
        .add_plugin(DeathPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(ProjectilesPlugin);

    #[cfg(feature = "dev")]
    app.add_plugin(DebugPanelPlugin);

    app.run()
}
