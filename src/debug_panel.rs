use crate::prelude::*;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy_inspector_egui::{Inspectable, InspectorPlugin, WorldInspectorPlugin};

#[derive(Inspectable, Default)]
struct DebugPanel {
    fps: f64,
    frame_time: f64,
}

fn debug_fps_information(
    mut debug_panel: ResMut<DebugPanel>,
    mut loaded: Local<bool>,
    diagnostics: Res<Diagnostics>,
) {
    if *loaded {
        let fps = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS).unwrap();
        let frame_time = diagnostics
            .get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
            .unwrap();
        debug_panel.fps = fps.average().unwrap();
        debug_panel.frame_time = frame_time.value().unwrap();
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

pub struct DebugPanelPlugin;
impl Plugin for DebugPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_plugin(InspectorPlugin::<DebugPanel>::new())
            .add_system_set(SystemSet::on_update(Playing).with_system(debug_fps_information));
    }
}
