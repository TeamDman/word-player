use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::{Monitor, MonitorSelection, Window, WindowMode, WindowPosition, WindowRef};

pub struct SetupWindowsPlugin;

impl Plugin for SetupWindowsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_windows);
    }
}

fn setup_windows(mut commands: Commands, monitors: Query<(Entity, &Monitor)>) {
    for (i, (monitor_entity, monitor)) in monitors.iter().enumerate() {
        let position = monitor.physical_position.as_vec2();
        let size = monitor.physical_size().as_vec2();
        let window_entity = commands
            .spawn(Window {
                title: format!("urine-region-select-{}", i),
                resolution: (size.x, size.y).into(),
                position: WindowPosition::At(position.as_ivec2()),
                // mode: WindowMode::BorderlessFullscreen(MonitorSelection::Entity(monitor_entity)),
                mode: WindowMode::Windowed,
                decorations: false,
                transparent: true,
                ..default()
            })
            .id();
        commands.spawn((
            Camera2d {
                ..default()
            },
            Camera {
                target: RenderTarget::Window(WindowRef::Entity(window_entity)),
                ..default()
            },
            Transform::from_translation(Vec3::new(position.x + size.x / 2.0, -(position.y + size.y / 2.0), 999.0)),
        ));
    }
}
