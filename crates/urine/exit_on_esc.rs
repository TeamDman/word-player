use bevy::prelude::*;

pub struct ExitOnEscPlugin;

impl Plugin for ExitOnEscPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, exit_on_esc_system);
    }
}

fn exit_on_esc_system(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
