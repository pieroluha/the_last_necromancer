use crate::prelude::*;

#[derive(Component)]
pub struct MainCamera;

fn add_2d_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera)
        .insert(Name::new("MainCamera"));
}

//fn add_3d_camera(mut commands: Commands) {
//    commands
//        .spawn_bundle(PerspectiveCameraBundle::default())
//        .insert(MainCamera)
//        .insert(Name::new("MainCamera"));
//}

fn add_ui_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

pub struct MyCameraPlugin;
impl Plugin for MyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AssetLoad).with_system(add_2d_camera))
            .add_system_set(SystemSet::on_enter(AssetLoad).with_system(add_ui_camera));
    }
}
