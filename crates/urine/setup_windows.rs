use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::Monitor;
use bevy::window::Window;
use bevy::window::WindowMode;
use bevy::window::WindowPosition;
use bevy::window::WindowRef;

pub struct SetupWindowsPlugin;

impl Plugin for SetupWindowsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_windows);
    }
}

fn setup_windows(mut commands: Commands, monitors: Query<(Entity, &Monitor)>) {
    // Find the bounding rectangle covering all monitors
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    for (_entity, monitor) in monitors.iter() {
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
        let window_entity = commands
            .spawn(Window {
                title: "urine-region-select-all".to_string(),
                resolution: (size.x, size.y).into(),
                position: WindowPosition::At(position.as_ivec2()),
                mode: WindowMode::Windowed,
                decorations: false,
                transparent: true,
                ..default()
            })
            .id();
        commands.spawn((
            Camera2d { ..default() },
            Camera {
                target: RenderTarget::Window(WindowRef::Entity(window_entity)),
                ..default()
            },
            Transform::from_translation(Vec3::new(
                position.x + size.x / 2.0,
                -(position.y + size.y / 2.0),
                999.0,
            )),
            Name::new("Camera2d for all monitors"),
        ));
    }
}
