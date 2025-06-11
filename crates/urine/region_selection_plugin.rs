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

#[derive(Component)]
pub struct SelectionOverlayText; // New marker component for the text

fn region_selection_system(
    mut state: ResMut<SelectionState>,
    mut exit: EventWriter<AppExit>,
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
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
        }
    }
    // Finalize only on Enter or NumpadEnter
    if (keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter))
        && state.start.is_some() && state.end.is_some() && !state.finalized {
        state.finalized = true;
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
    asset_server: Res<AssetServer>, // Added AssetServer
    mut sprite_overlay_query: Query<(Entity, &mut Sprite, &mut Transform), (With<SelectionOverlay>, Without<SelectionOverlayText>)>,
    mut text_overlay_query: Query<(Entity, &mut Text2d, &mut Transform), With<SelectionOverlayText>>, // Changed Text to Text2d
    state: Res<SelectionState>,
) {
    if let (Some(start), Some(end)) = (state.start, state.end) {
        if state.finalized {
            // Remove sprite overlay if present
            for (entity, _, _) in sprite_overlay_query.iter_mut() {
                commands.entity(entity).despawn_recursive();
            }
            // Remove text overlay if present
            for (entity, _, _) in text_overlay_query.iter_mut() {
                commands.entity(entity).despawn_recursive();
            }
            return;
        }
        let min = Vec2::new(start.x.min(end.x), start.y.min(end.y));
        let max = Vec2::new(start.x.max(end.x), start.y.max(end.y));
        let size = max - min;
        let rect_color = Color::srgba(0.2, 0.5, 1.0, 0.3); // translucent blue
        let formatted_text = format!("x: {:.0}, y: {:.0}, w: {:.0}, h: {:.0}", min.x, min.y, size.x, size.y);

        // Handle Sprite Overlay
        if let Ok((_entity, mut sprite, mut transform)) = sprite_overlay_query.get_single_mut() {
            sprite.color = rect_color;
            sprite.custom_size = Some(size);
            transform.translation = min.extend(100.0); // z=100 to draw on top
        } else {
            // Spawn new sprite overlay using individual components
            commands.spawn((
                Sprite {
                    color: rect_color,
                    custom_size: Some(size),
                    ..default()
                },
                Transform::from_translation(min.extend(100.0)),
                SelectionOverlay, // Marker for the sprite part
                Name::new("Selection Overlay Rectangle"),
            ));
        }

        // Handle Text Overlay
        let text_position = (min + size / 2.0 + Vec2::Y * 20.0).extend(101.0); // z=101 for text

        if let Ok((_entity, mut text_2d_component, mut transform_component)) = text_overlay_query.get_single_mut() {
            // Update existing text
            text_2d_component.0 = formatted_text.clone(); // Update the String content of Text2d
            transform_component.translation = text_position;
        } else {
            // Spawn new text overlay
            commands.spawn((
                Text2d::new(formatted_text.clone()), // Creates Text2d(String) component
                TextFont {
                    font: asset_server.load("fonts/FixederSys2x.ttf"), // Ensure this font is in assets
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(JustifyText::Center), // Center text, corrected argument
                Transform::from_translation(text_position),
                SelectionOverlayText, // New marker for the text part
                Name::new("Selection Overlay Text"),
            ));
        }
    } else {
        // No selection, remove overlays if present
        for (entity, _, _) in sprite_overlay_query.iter_mut() {
            commands.entity(entity).despawn_recursive();
        }
        for (entity, _, _) in text_overlay_query.iter_mut() {
            commands.entity(entity).despawn_recursive();
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
