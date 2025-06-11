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
    windows: Query<(Entity, &Window)>,
    monitors: Query<&bevy::window::Monitor>,
) {
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
            let screen = monitor_offset + cursor_pos;
            let world = Vec2::new(screen.x, -screen.y);
            *cursor_position = CursorPosition::Some { screen, world };
            return;
        }
    }
    *cursor_position = CursorPosition::None;
}

pub struct WorldCursorPlugin;

impl Plugin for WorldCursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPosition>()
            .register_type::<CursorPosition>()
            .add_systems(PreUpdate, update_cursor_positions);
    }
}
