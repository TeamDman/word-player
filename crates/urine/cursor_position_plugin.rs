use bevy::prelude::*;
use bevy_math_utils::prelude::NegativeYVec2;

#[derive(Resource, Debug, Clone, Reflect, Default)]
#[reflect(Resource)]
pub struct CursorPosition {
    pub world_position: Option<Vec2>,
}

fn update_cursor_positions(mut cursor_position: ResMut<CursorPosition>, windows: Query<&Window>) {
    for window in windows.iter() {
        if let Some(cursor_pos) = window.cursor_position() {
            let window_pos = match window.position {
                bevy::window::WindowPosition::At(pos) => pos.as_vec2(),
                _ => {
                    unreachable!("Window position must be set to 'At' for cursor position tracking")
                }
            };
            let host = window_pos + cursor_pos;
            let world = host.neg_y();
            cursor_position.world_position = Some(world);
            return;
        }
    }
    cursor_position.world_position = None;
}

pub struct CursorPositionPlugin;

impl Plugin for CursorPositionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPosition>()
            .register_type::<CursorPosition>()
            .add_systems(PreUpdate, update_cursor_positions);
    }
}
