use bevy::prelude::*;

mod camera;
mod debug_panel;
mod input;
mod prelude {
    pub use crate::camera::*;
    pub use crate::input::*;
    pub use bevy::prelude::*;

    #[derive(Clone, Eq, PartialEq, Debug, Hash)]
    pub enum GameState {
        AssetLoad,
        MainMenu,
        Playing,
        Pause,
        Exit,
    }
    pub use Action::*;
    pub use GameState::*;

    pub const DEFAULT_WIN_WIDTH: f32 = 800.0;
    pub const DEFAULT_WIN_HEIGHT: f32 = 600.0;
    pub const SLATE: Color = Color::rgb(38.0 / 255.0, 40.0 / 255.0, 42.0 / 255.0);

    #[cfg(feature = "dev")]
    pub use crate::debug_panel::*;
}

use prelude::*;

fn main() {
    let mut app = App::new();
    app.add_state(AssetLoad);

    app.insert_resource(ClearColor(SLATE))
        .insert_resource(WindowDescriptor {
            title: "placeholder_title".to_string(),
            width: DEFAULT_WIN_WIDTH,
            height: DEFAULT_WIN_HEIGHT,
            ..default()
        });

    app.add_plugins(DefaultPlugins)
        .add_plugin(MyCameraPlugin)
        .add_plugin(InputPlugin);

    #[cfg(feature = "dev")]
    app.add_plugin(DebugPanelPlugin);

    app.run()
}
