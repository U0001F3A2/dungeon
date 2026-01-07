//! Item rendering systems.

use bevy::prelude::*;

use crate::assets::SpriteAssets;
use crate::components::Item;
use crate::resources::{GameViewModel, TileSize};

/// Spawn item sprites from the view model.
pub fn spawn_items(
    mut commands: Commands,
    view_model: Option<Res<GameViewModel>>,
    tile_size: Res<TileSize>,
    sprites: Option<Res<SpriteAssets>>,
    existing_items: Query<Entity, With<Item>>,
    mut items_spawned: Local<bool>,
) {
    let Some(view_model) = view_model else {
        return;
    };

    let Some(sprites) = sprites else {
        return;
    };

    // Only spawn once initially (updates handled separately)
    if *items_spawned {
        return;
    }

    // Clear existing items
    for entity in existing_items.iter() {
        commands.entity(entity).despawn();
    }

    let tile_px = tile_size.0;
    let map = &view_model.0.map;

    // Calculate offset (same as tiles)
    let map_width = map.width as f32 * tile_px;
    let map_height = map.height as f32 * tile_px;
    let offset_x = -map_width / 2.0 + tile_px / 2.0;
    let offset_y = -map_height / 2.0 + tile_px / 2.0;

    for item in &view_model.0.items {
        let world_x = item.position.x as f32 * tile_px + offset_x;
        let world_y = item.position.y as f32 * tile_px + offset_y;

        // Get sprite texture based on item type
        let texture = sprites.item_sprite(item.handle.0);

        // Items are slightly smaller than a tile
        let item_size = tile_px * 0.75;

        commands.spawn((
            Sprite {
                image: texture,
                custom_size: Some(Vec2::splat(item_size)),
                ..default()
            },
            Transform::from_xyz(world_x, world_y, 0.3), // Z = 0.3 below props but above tiles
            Item {
                entity_id: item.id,
            },
        ));
    }

    *items_spawned = true;
    if !view_model.0.items.is_empty() {
        tracing::info!("Spawned {} items with sprites", view_model.0.items.len());
    }
}

/// Update item positions when the view model changes.
/// Also despawns items that were picked up (no longer in view model).
pub fn update_item_positions(
    mut commands: Commands,
    view_model: Option<Res<GameViewModel>>,
    tile_size: Res<TileSize>,
    items: Query<(Entity, &Item, &Transform)>,
) {
    let Some(view_model) = view_model else {
        return;
    };

    if !view_model.is_changed() {
        return;
    }

    let tile_px = tile_size.0;
    let map = &view_model.0.map;

    let map_width = map.width as f32 * tile_px;
    let map_height = map.height as f32 * tile_px;
    let offset_x = -map_width / 2.0 + tile_px / 2.0;
    let offset_y = -map_height / 2.0 + tile_px / 2.0;

    for (entity, item_component, transform) in items.iter() {
        // Find the item in the view model
        if let Some(item_view) = view_model
            .0
            .items
            .iter()
            .find(|i| i.id == item_component.entity_id)
        {
            let world_x = item_view.position.x as f32 * tile_px + offset_x;
            let world_y = item_view.position.y as f32 * tile_px + offset_y;

            // Only update if position changed
            if (transform.translation.x - world_x).abs() > 0.01
                || (transform.translation.y - world_y).abs() > 0.01
            {
                commands.entity(entity).insert(Transform::from_xyz(
                    world_x,
                    world_y,
                    transform.translation.z,
                ));
            }
        } else {
            // Item was picked up - despawn the entity
            commands.entity(entity).despawn();
        }
    }
}
