use bevy::app::AppExit;
use bevy::input::mouse::MouseButton;
use bevy::input::ButtonInput;
use bevy::math::IRect;
use bevy::math::IVec2;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::window::Window;

use crate::world_cursor_plugin::WorldCursorPosition; // Added import

#[derive(Resource, Default)]
pub struct SelectionState {
    pub start: Option<Vec2>,
    pub end: Option<Vec2>,
    pub finalized: bool,
}

#[derive(Component)]
pub struct SelectionOverlay;

#[derive(Component)]
pub struct SelectionOverlayText; // New marker component for the text

// --- Selection State Plugin ---
fn region_selection_system(
    mut state: ResMut<SelectionState>,
    mut exit: EventWriter<AppExit>,
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>, // Keep for screen coordinate conversion for output
    world_cursor_position: Res<WorldCursorPosition>, // Added
) {
    if let Some(world_pos) = world_cursor_position.0 {
        // Use the new resource
        if buttons.just_pressed(MouseButton::Left) {
            // Start new selection, clobber any previous
            state.start = Some(world_pos);
            state.end = Some(world_pos);
            state.finalized = false;
        } else if buttons.pressed(MouseButton::Left) {
            state.end = Some(world_pos);
        } else if buttons.just_released(MouseButton::Left) {
            state.end = Some(world_pos);
            // Do not finalize here
        }
    }
    // Finalize only on Enter or NumpadEnter
    if (keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter))
        && state.start.is_some()
        && state.end.is_some()
        && !state.finalized
    {
        state.finalized = true;
    }
    info!(
        "SelectionState: start={:?}, end={:?}, finalized={}",
        state.start, state.end, state.finalized
    );
    if state.finalized {
        if let (Some(start), Some(end)) = (state.start, state.end) {
            // Flip Y back to screen coords for output
            let window = windows.iter().next().unwrap();
            let window_height = window.resolution.height();
            let start_screen = Vec2::new(start.x, window_height - start.y);
            let end_screen = Vec2::new(end.x, window_height - end.y);
            let min = Vec2::new(
                start_screen.x.min(end_screen.x),
                start_screen.y.min(end_screen.y),
            );
            let max = Vec2::new(
                start_screen.x.max(end_screen.x),
                start_screen.y.max(end_screen.y),
            );
            let min_ivec = IVec2::new(min.x.round() as i32, min.y.round() as i32);
            let max_ivec = IVec2::new(max.x.round() as i32, max.y.round() as i32);
            let irect = IRect::from_corners(min_ivec, max_ivec);
            let json = serde_json::to_string(&irect).unwrap();
            println!("{}", json);
        }
        exit.write(AppExit::Success);
    }
}

// --- Plugins ---
pub struct RegionSelectionPlugin;

impl Plugin for RegionSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectionState>()
            .add_systems(Update, region_selection_system);
    }
}
