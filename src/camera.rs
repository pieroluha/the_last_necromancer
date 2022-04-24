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

fn edit_camera_scaling(
    mut query_camera: Query<(&mut OrthographicProjection, &mut Transform), With<MainCamera>>,
) {
    let (mut camera, mut transform) = query_camera.single_mut();
    camera.scaling_mode = ScalingMode::FixedVertical;
    camera.scale = 200.0;
    transform.translation.x = ARENA_OFFSET;
    transform.translation.y = ARENA_OFFSET;
}

#[derive(Default)]
pub struct CursorPosition {
    pub pos: Vec2,
}

impl CursorPosition {
    fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    //pub fn offset_pos(&self) -> Vec2 {
    //    let multiple = CELL_SIZE as i32;
    //    let x = self.pos.x as i32;
    //    let y = self.pos.y as i32;
    //    let x = ((x + multiple - 1) & -multiple) as f32;
    //    let y = ((y + multiple - 1) & -multiple) as f32;

    //    Vec2::new(x, y)
    //}

    pub fn offset_pos_integer(&self) -> IVec2 {
        let multiple = CELL_SIZE as i32;
        let x = self.pos.x as i32;
        let y = self.pos.y as i32;
        let x = (x + multiple - 1) & -multiple;
        let y = (y + multiple - 1) & -multiple;

        IVec2::new(x, y)
    }
}

// Code is from the Unoffical Bevy Cheatbook
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

pub struct MyCameraPlugin;
impl Plugin for MyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPosition>()
            .add_system_set(SystemSet::on_enter(AssetLoad).with_system(add_2d_camera))
            .add_system_set(SystemSet::on_enter(AssetLoad).with_system(add_ui_camera))
            .add_system_set(SystemSet::on_update(Playing).with_system(set_cursor_pos))
            .add_system_set(SystemSet::on_enter(Playing).with_system(edit_camera_scaling));
    }
}
