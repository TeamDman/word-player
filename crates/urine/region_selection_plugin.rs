use bevy::app::AppExit;
use bevy::input::mouse::MouseButton;
use bevy::input::ButtonInput;
use bevy::math::IRect;
use bevy::math::IVec2;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::window::Window;

#[derive(Resource, Default)]
pub struct SelectionState {
    pub start: Option<Vec2>,
    pub end: Option<Vec2>,
    pub finalized: bool,
}

#[derive(Component)]
pub struct SelectionOverlay;

fn region_selection_system(
    mut state: Local<SelectionState>,
    mut exit: EventWriter<AppExit>,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    let mut found_cursor = false;
    let (camera, camera_transform) = match cameras.iter().next() {
        Some(pair) => pair,
        None => return,
    };
    for window in windows.iter() {
        if let Some(pos) = window.cursor_position() {
            found_cursor = true;
            // Convert from screen (window) coords to world coords (flip Y)
            let window_height = window.resolution.height();
            let flipped_pos = Vec2::new(pos.x, window_height - pos.y);
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, flipped_pos) {
                if buttons.just_pressed(MouseButton::Left) {
                    state.start = Some(world_pos);
                    state.end = Some(world_pos);
                } else if buttons.pressed(MouseButton::Left) {
                    state.end = Some(world_pos);
                } else if buttons.just_released(MouseButton::Left) {
                    state.end = Some(world_pos);
                    state.finalized = true;
                }
            }
        }
    }
    if !found_cursor {
        // Optionally, clear state or handle no cursor present
    }
    info!("SelectionState: start={:?}, end={:?}, finalized={}", state.start, state.end, state.finalized);
    if state.finalized {
        if let (Some(start), Some(end)) = (state.start, state.end) {
            // Flip Y back to screen coords for output
            let window = windows.iter().next().unwrap();
            let window_height = window.resolution.height();
            let start_screen = Vec2::new(start.x, window_height - start.y);
            let end_screen = Vec2::new(end.x, window_height - end.y);
            let min = Vec2::new(start_screen.x.min(end_screen.x), start_screen.y.min(end_screen.y));
            let max = Vec2::new(start_screen.x.max(end_screen.x), start_screen.y.max(end_screen.y));
            let min_ivec = IVec2::new(min.x.round() as i32, min.y.round() as i32);
            let max_ivec = IVec2::new(max.x.round() as i32, max.y.round() as i32);
            let irect = IRect::from_corners(min_ivec, max_ivec);
            let json = serde_json::to_string(&irect).unwrap();
            println!("{}", json);
        }
        exit.write(AppExit::Success);
    }
}

fn update_selection_overlay(
    mut commands: Commands,
    mut overlay_query: Query<(Entity, &mut Sprite, &mut Transform, Option<&mut Children>), With<SelectionOverlay>>,
    mut text2d_query: Query<(Entity, &mut Text), With<SelectionOverlay>>,
    state: Local<SelectionState>,
) {
    // Only show overlay while dragging (not finalized)
    if let (Some(start), Some(end)) = (state.start, state.end) {
        if state.finalized {
            // Remove overlay if present
            for (entity, _, _, _) in overlay_query.iter_mut() {
                commands.entity(entity).despawn();
            }
            for (entity, _) in text2d_query.iter_mut() {
                commands.entity(entity).despawn();
            }
            return;
        }
        let min = Vec2::new(start.x.min(end.x), start.y.min(end.y));
        let max = Vec2::new(start.x.max(end.x), start.y.max(end.y));
        let size = max - min;
        let rect_color = Color::srgba(0.2, 0.5, 1.0, 0.3); // translucent blue
        let text_color = Color::WHITE;
        let text = format!("x: {:.0}, y: {:.0}, w: {:.0}, h: {:.0}", min.x, min.y, size.x, size.y);
        let mut found_overlay = false;
        if let Ok((_entity, mut sprite, mut transform, _)) = overlay_query.single_mut() {
            found_overlay = true;
            sprite.color = rect_color;
            sprite.custom_size = Some(size);
            transform.translation = min.extend(100.0); // z=100 to draw on top
        }
        let mut found_text = false;
        for (_entity, mut text_comp) in text2d_query.iter_mut() {
            found_text = true;
            // Update the text string directly, since Text2d is a wrapper around String
            *text_comp = text.clone().into();
        }
        if !found_overlay {
            commands.spawn((
                Sprite {
                    color: rect_color,
                    custom_size: Some(size),
                    ..Default::default()
                },
                Transform::from_translation(min.extend(100.0)),
                SelectionOverlay,
            ));
        }
        if !found_text {
            commands.spawn((
                Text2d::new(text),
                Transform::from_translation((min + size / 2.0 + Vec2::Y * 20.0).extend(101.0)),
                SelectionOverlay,
            ));
        }
    } else {
        // No selection, remove overlay if present
        for (entity, _, _, _) in overlay_query.iter_mut() {
            commands.entity(entity).despawn();
        }
        for (entity, _) in text2d_query.iter_mut() {
            commands.entity(entity).despawn();
        }
    }
}

pub struct RegionSelectionPlugin;

impl Plugin for RegionSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectionState>()
            .add_systems(Update, region_selection_system)
            .add_systems(Update, update_selection_overlay);
    }
}
