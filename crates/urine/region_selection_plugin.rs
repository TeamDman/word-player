use crate::cursor_position_plugin::CursorPosition;
use bevy::input::ButtonInput;
use bevy::input::mouse::MouseButton;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::Window;
use bevy_math_utils::prelude::NegativeYIVec2;

#[derive(Resource, Debug, Clone, Reflect, Copy)]
#[reflect(Resource)]
#[derive(Default)]
pub enum SelectionState {
    #[default]
    NotStarted,
    Dragging {
        start: Vec2,
        end: Vec2,
    },
    Completed {
        start: Vec2,
        end: Vec2,
    },
}
impl SelectionState {
    pub fn rect(self) -> Option<Rect> {
        match self {
            SelectionState::NotStarted => None,
            SelectionState::Dragging { start, end } => Some(Rect::from_corners(start, end)),
            SelectionState::Completed { start, end } => Some(Rect::from_corners(start, end)),
        }
    }
}

fn region_selection_system(
    mut state: ResMut<SelectionState>,
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    cursor_position: Res<CursorPosition>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let Some(cursor_world_position) = cursor_position.world_position.as_ref() else {
        return;
    };
    // Handle Ctrl+A to select the entire window
    if (keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight))
        && keys.just_pressed(KeyCode::KeyA)
    {
        if let Ok(window) = windows.single() {
            match window.position {
                bevy::window::WindowPosition::At(window_host_pos) => {
                    let window_world_pos = window_host_pos.neg_y().as_vec2();
                    let start = window_world_pos;
                    let end = start + window.size();
                    *state = SelectionState::Completed { start, end };
                }
                _ => {}
            }
        }
    }
    match &mut *state {
        SelectionState::NotStarted => {
            if buttons.just_pressed(MouseButton::Left) {
                *state = SelectionState::Dragging {
                    start: *cursor_world_position,
                    end: *cursor_world_position,
                };
            }
        }
        SelectionState::Dragging {
            start,
            end,
        } => {
            if keys.just_pressed(KeyCode::Escape) {
                *state = SelectionState::NotStarted;
            } else if buttons.pressed(MouseButton::Left) {
                *end = *cursor_world_position;
            } else if buttons.just_released(MouseButton::Left) {
                if *start != *end {
                    *state = SelectionState::Completed {
                        start: *start,
                        end: *end,
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
                *state = SelectionState::Dragging {
                    start: *cursor_world_position,
                    end: *cursor_world_position,
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
