use bevy::prelude::*;
use bevy::window::Monitor;

pub struct DimmerPlugin;

impl Plugin for DimmerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_dimmers);
    }
}

fn spawn_dimmers(mut commands: Commands, monitors: Query<&Monitor>) {
    for monitor in monitors.iter() {
        let position = monitor.physical_position.as_vec2();
        let size = monitor.physical_size().as_vec2();
        commands.spawn((
            Sprite {
                color: Color::srgba(0.0, 0.0, 0.0, 0.5), // semi-transparent black
                custom_size: Some(size),
                ..default()
            },
            Transform::from_translation(Vec3::new(
                position.x + size.x / 2.0,
                -(position.y + size.y / 2.0),
                0.0,
            )),
            Name::new("Dimmer Overlay"),
        ));
    }
}
