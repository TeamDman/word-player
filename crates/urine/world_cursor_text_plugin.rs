use crate::cursor_position_plugin::CursorPosition;
use bevy::prelude::*;
use bevy::window::Monitor;

#[derive(Component)]
struct WorldCursorText;

fn setup_world_cursor_text_display(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    monitors: Query<&Monitor>,
) {
    let mut monitors_vec: Vec<&Monitor> = monitors.iter().collect();
    monitors_vec.sort_by(|a, b| a.physical_position.x.cmp(&b.physical_position.x));
    let mid_idx = monitors_vec.len() / 2;

    if let Some(monitor) = monitors_vec.get(mid_idx) {
        let position = monitor.physical_position.as_vec2();
        let size = monitor.physical_size().as_vec2();

        let text_x = position.x + size.x - 100.0;
        let text_y = -(position.y + 30.0);

        commands.spawn((
            Text2d::new("X: ---, Y: ---"),
            TextFont {
                font: asset_server.load("fonts/FixederSys2x.ttf"),
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::WHITE),
            TextLayout::new_with_justify(JustifyText::Right),
            Transform::from_xyz(text_x, text_y, 200.0),
            Name::new("World Cursor Text"),
            WorldCursorText,
        ));
    }
}

fn update_world_cursor_text_display(
    cursor_position: Res<CursorPosition>,
    mut query: Query<&mut Text2d, With<WorldCursorText>>,
) {
    if let CursorPosition::Some { world, .. } = &*cursor_position {
        for mut text in query.iter_mut() {
            text.0 = format!("X: {:.0}, Y: {:.0}", world.x, world.y);
        }
    } else {
        for mut text in query.iter_mut() {
            text.0 = "X: ---, Y: ---".to_string();
        }
    }
}

pub struct WorldCursorTextPlugin;

impl Plugin for WorldCursorTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world_cursor_text_display)
            .add_systems(Update, update_world_cursor_text_display);
    }
}
