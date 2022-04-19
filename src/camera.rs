use crate::prelude::*;
use bevy::render::camera::ScalingMode;

#[derive(Component)]
pub struct MainCamera;

fn add_2d_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera)
        .insert(Name::new("MainCamera"));
}

fn add_ui_camera(mut commands: Commands) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(Name::new("UiCamera"));
}

fn edit_camera_scaling(mut query_camera: Query<&mut OrthographicProjection, With<MainCamera>>) {
    let mut camera = query_camera.single_mut();
    camera.scaling_mode = ScalingMode::FixedVertical;
    camera.scale = 200.0;
}

pub struct MyCameraPlugin;
impl Plugin for MyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AssetLoad).with_system(add_2d_camera))
            .add_system_set(SystemSet::on_enter(AssetLoad).with_system(add_ui_camera))
            .add_system_set(SystemSet::on_enter(Playing).with_system(edit_camera_scaling));
    }
}
