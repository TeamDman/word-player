use crate::region_selection_plugin::SelectionState;
use bevy::app::AppExit;
use bevy::input::ButtonInput;
use bevy::prelude::*;

pub struct RegionSelectionReturnPlugin;

fn region_selection_return_system(
    state: Res<SelectionState>,
    mut exit: EventWriter<AppExit>,
    keys: Res<ButtonInput<KeyCode>>,
) -> Result {
    if let SelectionState::Completed { rect_screen, .. } = &*state {
        if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) {
            let json_screen = serde_json::to_string(&rect_screen)?;
            info!("screen coordinates: {json_screen}");
            println!("{json_screen}");
            exit.write(AppExit::Success);
        }
    }
    Ok(())
}

impl Plugin for RegionSelectionReturnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, region_selection_return_system);
    }
}
