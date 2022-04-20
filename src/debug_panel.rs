use crate::prelude::*;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy_inspector_egui::{Inspectable, InspectorPlugin, WorldInspectorPlugin};

#[derive(Inspectable, Default)]
struct DebugPanel {
    fps: f64,
    frame_time: f64,
    cursor_position: Vec2,
}

fn debug_fps_information(
    cursor_position: Res<CursorPosition>,
    diagnostics: Res<Diagnostics>,
    mut debug_panel: ResMut<DebugPanel>,
    mut loaded: Local<bool>,
) {
    if *loaded {
        let fps = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS).unwrap();
        let frame_time = diagnostics
            .get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
            .unwrap();
        debug_panel.fps = fps.average().unwrap();
        debug_panel.frame_time = frame_time.value().unwrap();
        debug_panel.cursor_position = cursor_position.pos;
    } else {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(frame_time) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME) {
                if let Some(_) = fps.value() {
                    if let Some(_) = frame_time.value() {
                        *loaded = true;
                    }
                }
            }
        }
    }
}

#[derive(Default)]
struct CursorPosition {
    pos: Vec2,
}

impl CursorPosition {
    fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }
}

fn set_cursor_pos(
    windows: Res<Windows>,
    query_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    let (camera, camera_transform) = query_camera.single();

    let window = windows.get_primary().unwrap();

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = window.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();
        // sets the cursor positions into the resource
        cursor_position.set_pos(world_pos);
    }
}

pub struct DebugPanelPlugin;
impl Plugin for DebugPanelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPosition>()
            .add_plugin(WorldInspectorPlugin::new())
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_plugin(InspectorPlugin::<DebugPanel>::new())
            .add_system_set(SystemSet::on_update(Playing).with_system(set_cursor_pos))
            .add_system_set(SystemSet::on_update(Playing).with_system(debug_fps_information));
    }
}
