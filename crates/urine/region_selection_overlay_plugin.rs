use crate::region_selection_plugin::SelectionState;
use bevy::prelude::*;
use bevy_math_utils::prelude::{NegativeYRect, TopLeft};

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
    let Some(world_rect) = state.rect() else {
        // No selection, remove overlays if present
        for (entity, _, _) in sprite_overlay_query.iter_mut() {
            commands.entity(entity).despawn();
        }
        for (entity, _, _) in text_overlay_query.iter_mut() {
            commands.entity(entity).despawn();
        }
        return;
    };
    let host_rect = world_rect.neg_y();

    // Draw overlay using world coordinates
    let rect_color = Color::srgba(0.2, 0.5, 1.0, 0.3); // translucent blue
    let formatted_text = format!(
        "x: {:.0}, y: {:.0}, w: {:.0}, h: {:.0}",
        host_rect.min.x,
        host_rect.min.y,
        host_rect.width(),
        host_rect.height()
    );

    // Handle Sprite Overlay
    if let Ok((_sprite_entity, mut sprite, mut sprite_transform)) =
        sprite_overlay_query.single_mut()
    {
        sprite.color = rect_color;
        sprite.custom_size = Some(world_rect.size());
        sprite.anchor = bevy::sprite::Anchor::Center;
        sprite_transform.translation = world_rect.center().extend(100.0); // z=100 for sprite
    } else {
        // Spawn new sprite overlay using individual components
        commands.spawn((
            Sprite {
                color: rect_color,
                custom_size: Some(world_rect.size()),
                anchor: bevy::sprite::Anchor::Center,
                ..default()
            },
            Transform::from_translation(world_rect.center().extend(100.0)),
            SelectionOverlay, // Marker for the sprite part
            Name::new("Selection Overlay Rectangle"),
        ));
    }

    // Handle Text Overlay
    let text_position = (world_rect.min + world_rect.half_size() + Vec2::Y * 20.0).extend(101.0); // z=101 for text

    if let Ok((_text_entity, mut text, mut text_transform)) = text_overlay_query.single_mut() {
        // Update existing text
        text.0 = formatted_text;
        text_transform.translation = text_position;
    } else {
        // Spawn new text overlay
        commands.spawn((
            Text2d::new(formatted_text.clone()),
            TextFont {
                font: asset_server.load("fonts/FixederSys2x.ttf"),
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::WHITE),
            TextLayout::new_with_justify(JustifyText::Center),
            Transform::from_translation(text_position),
            SelectionOverlayText,
            Name::new("Selection Overlay Text"),
        ));
    }
}

pub struct RegionSelectionOverlayPlugin;

impl Plugin for RegionSelectionOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_selection_overlay);
    }
}
