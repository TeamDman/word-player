use bevy::app::AppExit;
use bevy::input::mouse::MouseButton;
use bevy::input::ButtonInput;
use bevy::math::IRect;
use bevy::math::IVec2;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::window::Window;

use crate::world_cursor_plugin::CursorPosition;

#[derive(Resource, Debug, Clone)]
pub enum SelectionState {
    NotStarted,
    Dragging {
        rect_screen: IRect,
        rect_world: IRect,
    },
    Completed {
        rect_screen: IRect,
        rect_world: IRect,
    },
}

impl Default for SelectionState {
    fn default() -> Self {
        SelectionState::NotStarted
    }
}

// #[derive(Component)]
// pub struct SelectionOverlay;

// #[derive(Component)]
// pub struct SelectionOverlayText; // New marker component for the text

// --- Selection State Plugin ---
fn region_selection_system(
    mut state: ResMut<SelectionState>,
    mut exit: EventWriter<AppExit>,
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    cursor_position: Res<CursorPosition>,
) {
    let (world_pos, screen_pos) = match &*cursor_position {
        CursorPosition::Some { world, screen } => (*world, *screen),
        _ => return,
    };
    match &mut *state {
        SelectionState::NotStarted => {
            if buttons.just_pressed(MouseButton::Left) {
                // Start new selection with zero-size rects
                let rect_screen = IRect::from_corners(
                    IVec2::new(screen_pos.x.round() as i32, screen_pos.y.round() as i32),
                    IVec2::new(screen_pos.x.round() as i32, screen_pos.y.round() as i32),
                );
                let rect_world = IRect::from_corners(
                    IVec2::new(world_pos.x.round() as i32, world_pos.y.round() as i32),
                    IVec2::new(world_pos.x.round() as i32, world_pos.y.round() as i32),
                );
                *state = SelectionState::Dragging { rect_screen, rect_world };
            }
        }
        SelectionState::Dragging { rect_screen, rect_world } => {
            if buttons.pressed(MouseButton::Left) {
                // Update rects as mouse drags
                let start_screen = rect_screen.min;
                let start_world = rect_world.min;
                let end_screen = IVec2::new(screen_pos.x.round() as i32, screen_pos.y.round() as i32);
                let end_world = IVec2::new(world_pos.x.round() as i32, world_pos.y.round() as i32);
                *rect_screen = IRect::from_corners(start_screen, end_screen);
                *rect_world = IRect::from_corners(start_world, end_world);
            } else if buttons.just_released(MouseButton::Left) {
                // Wait for Enter to finalize
            }
            if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) {
                // Finalize selection
                let rect_screen = *rect_screen;
                let rect_world = *rect_world;
                *state = SelectionState::Completed { rect_screen, rect_world };
            }
        }
        SelectionState::Completed { rect_screen, rect_world } => {
            // Output and exit
            let json_screen = serde_json::to_string(&rect_screen).unwrap();
            let json_world = serde_json::to_string(&rect_world).unwrap();
            println!("screen: {}\nworld: {}", json_screen, json_world);
            exit.write(AppExit::Success);
        }
    }
    info!("SelectionState: {:?}", *state);
}

// --- Plugins ---
pub struct RegionSelectionPlugin;

impl Plugin for RegionSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectionState>()
            .add_systems(Update, region_selection_system);
    }
}
