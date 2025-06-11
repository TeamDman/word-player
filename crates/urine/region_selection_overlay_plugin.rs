use crate::region_selection_plugin::SelectionState;
use bevy::prelude::*;

#[derive(Component)]
pub struct SelectionOverlay;

#[derive(Component)]
pub struct SelectionOverlayText; // New marker component for the text
fn update_selection_overlay(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut sprite_overlay_query: Query<
        (Entity, &mut Sprite, &mut Transform),
        (With<SelectionOverlay>, Without<SelectionOverlayText>),
    >,
    mut text_overlay_query: Query<
        (Entity, &mut Text2d, &mut Transform),
        With<SelectionOverlayText>,
    >,
    state: Res<SelectionState>,
) {
    match &*state {
        SelectionState::Dragging {
            rect_world,
            rect_screen,
            ..
        }
        | SelectionState::Completed {
            rect_world,
            rect_screen,
            ..
        } => {
            // Draw overlay using world coordinates
            let min = rect_world.min.as_vec2();
            let max = rect_world.max.as_vec2();
            let size = max - min;
            let rect_color = Color::srgba(0.2, 0.5, 1.0, 0.3); // translucent blue

            // Display text using screen coordinates
            let screen_min = rect_screen.min.as_vec2();
            let screen_max = rect_screen.max.as_vec2();
            let screen_size = screen_max - screen_min;
            let formatted_text = format!(
                "screen: x: {:.0}, y: {:.0}, w: {:.0}, h: {:.0}",
                screen_min.x, screen_min.y, screen_size.x, screen_size.y
            );

            // Handle Sprite Overlay
            if let Ok((_entity, mut sprite, mut transform)) = sprite_overlay_query.single_mut() {
                sprite.color = rect_color;
                sprite.custom_size = Some(size);
                sprite.anchor = bevy::sprite::Anchor::TopRight;
                transform.translation = max.extend(100.0); // anchor at top-right
            } else {
                // Spawn new sprite overlay using individual components
                commands.spawn((
                    Sprite {
                        color: rect_color,
                        custom_size: Some(size),
                        anchor: bevy::sprite::Anchor::TopRight,
                        ..default()
                    },
                    Transform::from_translation(max.extend(100.0)),
                    SelectionOverlay, // Marker for the sprite part
                    Name::new("Selection Overlay Rectangle"),
                ));
            }

            // Handle Text Overlay
            let text_position = (min + size / 2.0 + Vec2::Y * 20.0).extend(101.0); // z=101 for text

            if let Ok((_entity, mut text_2d_component, mut transform_component)) =
                text_overlay_query.single_mut()
            {
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
        }
        _ => {
            // No selection, remove overlays if present
            for (entity, _, _) in sprite_overlay_query.iter_mut() {
                commands.entity(entity).despawn();
            }
            for (entity, _, _) in text_overlay_query.iter_mut() {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub struct RegionSelectionOverlayPlugin;

impl Plugin for RegionSelectionOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_selection_overlay);
    }
}
