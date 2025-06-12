use bevy::prelude::*;

use crate::region_selection_plugin::SelectionState;

pub struct ExitOnEscPlugin;

impl Plugin for ExitOnEscPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, exit_on_esc_system);
    }
}

fn exit_on_esc_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
    selection_state: Option<Res<SelectionState>>,
) {
    if keys.just_pressed(KeyCode::Escape)
        || keys.just_pressed(KeyCode::KeyC)
            && (keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight))
    {
        // Only exit if we're not in the middle of a selection
        // or if SelectionState resource doesn't exist yet
        match selection_state {
            Some(state) => {
                if let SelectionState::NotStarted = *state {
                    // Only exit if selection is not active
                    exit.write(AppExit::Success);
                }
                // Otherwise, the region_selection_system will handle the ESC key
            }
            None => {
                // If selection state doesn't exist, always exit
                exit.write(AppExit::Success);
            }
        }
    }
}
