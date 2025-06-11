use bevy::prelude::*;
use bevy::window::Monitor;

use crate::Args;

fn prompt_draw_system(
    args: Res<Args>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    monitors: Query<&Monitor>,
) {
    info!("Prompt: {:?}", args.prompt);
    if let Some(prompt) = &args.prompt {
        // Find the middlemost monitor by sorting by x position
        let mut monitors_vec: Vec<&Monitor> = monitors.iter().collect();
        monitors_vec.sort_by(|a, b| a.physical_position.x.cmp(&b.physical_position.x));
        let mid_idx = monitors_vec.len() / 2;
        if let Some(monitor) = monitors_vec.get(mid_idx) {
            let position = monitor.physical_position.as_vec2();
            let size = monitor.physical_size().as_vec2();
            // Top left in world space, inverting y
            let text_x = position.x + size.x/2.0;
            let text_y = -(position.y + 100.0);
            commands.spawn((
                Text2d::new(prompt),
                TextFont {
                    font: asset_server.load("fonts/FixederSys2x.ttf"),
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(JustifyText::Left),
                Transform::from_xyz(text_x, text_y, 1.0),
            ));
        }
    }
}

pub struct PromptDrawPlugin;

impl Plugin for PromptDrawPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, prompt_draw_system);
    }
}
