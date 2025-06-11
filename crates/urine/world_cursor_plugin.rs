use bevy::prelude::*;
// use bevy::render::camera::RenderTarget;
// use bevy::window::WindowRef;

#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource)]
pub struct WorldCursorPosition(pub Option<Vec2>);

#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource)]
pub struct ScreenCursorPosition(pub Option<Vec2>);

fn update_cursor_positions(
    mut world_cursor_position: ResMut<WorldCursorPosition>,
    mut screen_cursor_position: ResMut<ScreenCursorPosition>,
    windows: Query<(Entity, &Window)>,
    monitors: Query<&bevy::window::Monitor>,
) {
    let mut found = false;
    for (_, window) in windows.iter() {
        if let Some(cursor_pos) = window.cursor_position() {
            let monitor = monitors
                .iter()
                .find(|m| WindowPosition::At(m.physical_position) == window.position);
            let monitor_offset = if let Some(monitor) = monitor {
                monitor.physical_position.as_vec2()
            } else {
                Vec2::ZERO
            };
            // Screen position: cursor relative to desktop (monitor offset + cursor_pos, Y not flipped)
            let screen_pos = monitor_offset + cursor_pos;
            world_cursor_position.0 = Some(Vec2::new(screen_pos.x, -screen_pos.y));
            screen_cursor_position.0 = Some(screen_pos);
            found = true;
            break;
        }
    }
    if !found {
        world_cursor_position.0 = None;
        screen_cursor_position.0 = None;
    }
}

pub struct WorldCursorPlugin;

impl Plugin for WorldCursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldCursorPosition>()
            .init_resource::<ScreenCursorPosition>()
            .register_type::<WorldCursorPosition>()
            .register_type::<ScreenCursorPosition>()
            .add_systems(PreUpdate, update_cursor_positions);
    }
}
