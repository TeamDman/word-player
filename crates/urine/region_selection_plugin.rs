use bevy::app::AppExit;
use bevy::input::mouse::MouseButton;
use bevy::input::ButtonInput;
use bevy::math::IRect;
use bevy::math::IVec2;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::window::Window;

use crate::world_cursor_plugin::CursorPosition;

#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub enum SelectionState {
    NotStarted,
    Dragging {
        rect_screen: IRect,
        rect_world: IRect,
        start_screen: IVec2,  // Initial point where drag started
        start_world: IVec2,   // Initial point where drag started in world coords
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
                // Start new selection with the initial cursor position
                let start_screen = IVec2::new(screen_pos.x.round() as i32, screen_pos.y.round() as i32);
                let start_world = IVec2::new(world_pos.x.round() as i32, world_pos.y.round() as i32);
                
                // Initial rect is just a point (zero size)
                let rect_screen = IRect::from_corners(start_screen, start_screen);
                let rect_world = IRect::from_corners(start_world, start_world);
                
                *state = SelectionState::Dragging { 
                    rect_screen, 
                    rect_world,
                    start_screen,
                    start_world
                };
            }
        }
        SelectionState::Dragging { rect_screen, rect_world, start_screen, start_world } => {
            if keys.just_pressed(KeyCode::Escape) {
                // Cancel selection immediately on Escape
                *state = SelectionState::NotStarted;
            } else if buttons.pressed(MouseButton::Left) {
                // Current cursor position
                let current_screen = IVec2::new(screen_pos.x.round() as i32, screen_pos.y.round() as i32);
                let current_world = IVec2::new(world_pos.x.round() as i32, world_pos.y.round() as i32);
                // Create rectangle from the start corner and current position
                *rect_screen = IRect::from_corners(*start_screen, current_screen);
                *rect_world = IRect::from_corners(*start_world, current_world);
            } else if buttons.just_released(MouseButton::Left) {
                // Finalize selection on mouse release
                if rect_screen.min != rect_screen.max && rect_world.min != rect_world.max {
                    let rect_screen = *rect_screen;
                    let rect_world = *rect_world;
                    *state = SelectionState::Completed { rect_screen, rect_world };
                } else {
                    // If selection has zero area, cancel it
                    *state = SelectionState::NotStarted;
                }
            }
        }
        SelectionState::Completed { rect_screen, rect_world } => {
            if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) {
                // Output and exit
                let json_screen = serde_json::to_string(&rect_screen).unwrap();
                let json_world = serde_json::to_string(&rect_world).unwrap();
                println!("screen: {}\nworld: {}", json_screen, json_world);
                exit.write(AppExit::Success);
            } else if keys.just_pressed(KeyCode::Escape) {
                // Reset to NotStarted on Escape
                *state = SelectionState::NotStarted;
            } else if buttons.just_pressed(MouseButton::Left) {
                // Restart dragging with new selection
                let start_screen = IVec2::new(screen_pos.x.round() as i32, screen_pos.y.round() as i32);
                let start_world = IVec2::new(world_pos.x.round() as i32, world_pos.y.round() as i32);
                let rect_screen = IRect::from_corners(start_screen, start_screen);
                let rect_world = IRect::from_corners(start_world, start_world);
                *state = SelectionState::Dragging {
                    rect_screen,
                    rect_world,
                    start_screen,
                    start_world
                };
            }
        }
    }
    info!("SelectionState: {:?}", *state);
}

pub struct RegionSelectionPlugin;

impl Plugin for RegionSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectionState>()
            .register_type::<SelectionState>()
            .add_systems(Update, region_selection_system);
    }
}