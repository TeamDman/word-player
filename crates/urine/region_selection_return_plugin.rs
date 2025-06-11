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
            info!("screen coordinates: {rect_screen:?}");
            println!(
                "{x} {y} {w} {h}",
                x = rect_screen.min.x,
                y = rect_screen.min.y,
                w = rect_screen.width(),
                h = rect_screen.height()
            );
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
