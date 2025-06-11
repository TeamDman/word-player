use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct WorldCursorPosition(pub Option<Vec2>);

fn update_world_cursor_position(
    mut world_cursor_position: ResMut<WorldCursorPosition>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = match cameras.iter().next() {
        Some(pair) => pair,
        None => {
            world_cursor_position.0 = None;
            return;
        }
    };

    let mut cursor_in_a_window = false;
    for window in windows.iter() {
        if let Some(pos) = window.cursor_position() {
            let window_height = window.resolution.height();
            let flipped_pos = Vec2::new(pos.x, window_height - pos.y);
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, flipped_pos) {
                world_cursor_position.0 = Some(world_pos);
                cursor_in_a_window = true;
                break;
            }
        }
    }
    if !cursor_in_a_window {
        world_cursor_position.0 = None;
    }
}

pub struct WorldCursorPlugin;

impl Plugin for WorldCursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldCursorPosition>()
            .add_systems(PreUpdate, update_world_cursor_position);
    }
}
