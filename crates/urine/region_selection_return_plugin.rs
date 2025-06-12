use crate::region_selection_plugin::SelectionState;
use bevy::app::AppExit;
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy_math_utils::prelude::NegativeYRect;

pub struct RegionSelectionReturnPlugin;

fn region_selection_return_system(
    state: Res<SelectionState>,
    mut exit: EventWriter<AppExit>,
    keys: Res<ButtonInput<KeyCode>>,
) -> Result {
    if let SelectionState::Completed { .. } = &*state {
        if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) {
            let world_rect = state.rect().unwrap();
            let host_rect = world_rect.neg_y();
            info!("Host rect selection: {host_rect:?}");
            println!(
                "{x} {y} {w} {h}",
                x = host_rect.min.x,
                y = host_rect.min.y,
                w = host_rect.width(),
                h = host_rect.height()
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
