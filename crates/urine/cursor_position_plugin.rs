// Renamed from world_cursor_plugin.rs
use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
#[derive(Default)]
pub enum CursorPosition {
    #[default]
    None,
    Some {
        screen: Vec2,
        world: Vec2,
    },
}

fn update_cursor_positions(
    mut cursor_position: ResMut<CursorPosition>,
    windows: Query<&Window>,
) {
    // Find the window under the cursor
    for window in windows.iter() {
        if let Some(cursor_pos) = window.cursor_position() {
            // The window's position is the top-left in virtual screen coordinates
            let window_pos = match window.position {
                bevy::window::WindowPosition::At(pos) => pos.as_vec2(),
                _ => Vec2::ZERO,
            };
            let screen = window_pos + cursor_pos;
            let world = Vec2::new(screen.x, -screen.y);
            *cursor_position = CursorPosition::Some { screen, world };
            return;
        }
    }
    *cursor_position = CursorPosition::None;
}

pub struct CursorPositionPlugin;

impl Plugin for CursorPositionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPosition>()
            .register_type::<CursorPosition>()
            .add_systems(PreUpdate, update_cursor_positions);
    }
}
