use crate::world_cursor_plugin::CursorPosition;
use bevy::input::mouse::MouseButton;
use bevy::input::ButtonInput;
use bevy::math::IRect;
use bevy::math::IVec2;
use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
#[derive(Default)]
pub enum SelectionState {
    #[default]
    NotStarted,
    Dragging {
        rect_screen: IRect,
        rect_world: IRect,
        /// Initial point where drag started
        start_screen: IVec2, 
        /// Initial point where drag started in world coords
        start_world: IVec2,  
    },
    Completed {
        rect_screen: IRect,
        rect_world: IRect,
    },
}

fn region_selection_system(
    mut state: ResMut<SelectionState>,
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    cursor_position: Res<CursorPosition>,
) {
    let (world_pos, screen_pos) = match &*cursor_position {
        CursorPosition::Some { world, screen } => (*world, *screen),
        _ => return,
    };
    match &mut *state {
        SelectionState::NotStarted => {
            if buttons.just_pressed(MouseButton::Left) {
                let start_screen =
                    IVec2::new(screen_pos.x.round() as i32, screen_pos.y.round() as i32);
                let start_world =
                    IVec2::new(world_pos.x.round() as i32, world_pos.y.round() as i32);

                let rect_screen = IRect::from_corners(start_screen, start_screen);
                let rect_world = IRect::from_corners(start_world, start_world);

                *state = SelectionState::Dragging {
                    rect_screen,
                    rect_world,
                    start_screen,
                    start_world,
                };
            }
        }
        SelectionState::Dragging {
            rect_screen,
            rect_world,
            start_screen,
            start_world,
        } => {
            if keys.just_pressed(KeyCode::Escape) {
                *state = SelectionState::NotStarted;
            } else if buttons.pressed(MouseButton::Left) {
                let current_screen =
                    IVec2::new(screen_pos.x.round() as i32, screen_pos.y.round() as i32);
                let current_world =
                    IVec2::new(world_pos.x.round() as i32, world_pos.y.round() as i32);
                *rect_screen = IRect::from_corners(*start_screen, current_screen);
                *rect_world = IRect::from_corners(*start_world, current_world);
            } else if buttons.just_released(MouseButton::Left) {
                if rect_screen.min != rect_screen.max && rect_world.min != rect_world.max {
                    let rect_screen = *rect_screen;
                    let rect_world = *rect_world;
                    *state = SelectionState::Completed {
                        rect_screen,
                        rect_world,
                    };
                } else {
                    *state = SelectionState::NotStarted;
                }
            }
        }
        SelectionState::Completed { .. } => {
            if keys.just_pressed(KeyCode::Escape) {
                *state = SelectionState::NotStarted;
            } else if buttons.just_pressed(MouseButton::Left) {
                let start_screen =
                    IVec2::new(screen_pos.x.round() as i32, screen_pos.y.round() as i32);
                let start_world =
                    IVec2::new(world_pos.x.round() as i32, world_pos.y.round() as i32);
                let rect_screen = IRect::from_corners(start_screen, start_screen);
                let rect_world = IRect::from_corners(start_world, start_world);
                *state = SelectionState::Dragging {
                    rect_screen,
                    rect_world,
                    start_screen,
                    start_world,
                };
            }
        }
    }
}

pub struct RegionSelectionPlugin;

impl Plugin for RegionSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectionState>()
            .register_type::<SelectionState>()
            .add_systems(Update, region_selection_system);
    }
}
