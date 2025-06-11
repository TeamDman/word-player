use bevy::prelude::*;

use crate::region_selection_plugin::SelectionOverlay;
use crate::region_selection_plugin::SelectionOverlayText;
use crate::region_selection_plugin::SelectionState;

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
    if let (Some(start), Some(end)) = (state.start, state.end) {
        if state.finalized {
            // Remove sprite overlay if present
            for (entity, _, _) in sprite_overlay_query.iter_mut() {
                commands.entity(entity).despawn();
            }
            // Remove text overlay if present
            for (entity, _, _) in text_overlay_query.iter_mut() {
                commands.entity(entity).despawn();
            }
            return;
        }
        let min = Vec2::new(start.x.min(end.x), start.y.min(end.y));
        let max = Vec2::new(start.x.max(end.x), start.y.max(end.y));
        let size = max - min;
        let rect_color = Color::srgba(0.2, 0.5, 1.0, 0.3); // translucent blue
        let formatted_text = format!(
            "x: {:.0}, y: {:.0}, w: {:.0}, h: {:.0}",
            min.x, min.y, size.x, size.y
        );

        // Handle Sprite Overlay
        if let Ok((_entity, mut sprite, mut transform)) = sprite_overlay_query.single_mut() {
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
    } else {
        // No selection, remove overlays if present
        for (entity, _, _) in sprite_overlay_query.iter_mut() {
            commands.entity(entity).despawn();
        }
        for (entity, _, _) in text_overlay_query.iter_mut() {
            commands.entity(entity).despawn();
        }
    }
}

pub struct RegionSelectionOverlayPlugin;

impl Plugin for RegionSelectionOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_selection_overlay);
    }
}
