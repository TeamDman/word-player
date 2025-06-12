use bevy::prelude::*;
use bevy::window::Monitor;
use bevy::window::PrimaryWindow;
use bevy::window::Window;
use bevy::window::WindowMode;
use bevy::window::WindowPosition;

pub struct SetupWindowsPlugin;

impl Plugin for SetupWindowsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_windows);
    }
}

fn setup_windows(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    monitors: Query<&Monitor>,
    mut commands: Commands,
) {
    // Find the bounding rectangle covering all monitors
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    for monitor in monitors.iter() {
        let pos = monitor.physical_position.as_vec2();
        let size = monitor.physical_size().as_vec2();
        min_x = min_x.min(pos.x);
        min_y = min_y.min(pos.y);
        max_x = max_x.max(pos.x + size.x);
        max_y = max_y.max(pos.y + size.y);
    }

    if min_x.is_finite() && min_y.is_finite() && max_x.is_finite() && max_y.is_finite() {
        let position = Vec2::new(min_x, min_y);
        let size = Vec2::new(max_x - min_x, max_y - min_y);
        if let Ok(mut window) = windows.single_mut() {
            window.title = "urine-region-select-all".to_string();
            window.resolution.set(size.x, size.y);
            window.position = WindowPosition::At(position.as_ivec2());
            window.mode = WindowMode::Windowed;
            commands.spawn((
                Camera2d,
                Camera::default(),
                Transform::from_translation(Vec3::new(
                    position.x + size.x / 2.0,
                    -(position.y + size.y / 2.0),
                    999.0,
                )),
                Name::new("Camera2d for all monitors"),
            ));
        }
    }
}
