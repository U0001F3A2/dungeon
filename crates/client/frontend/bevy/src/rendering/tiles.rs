//! Tile rendering systems.

use bevy::prelude::*;

use crate::assets::SpriteAssets;
use crate::components::Tile;
use crate::resources::{GameViewModel, TileSize};

/// Spawn tile sprites from the view model.
pub fn spawn_tiles(
    mut commands: Commands,
    view_model: Option<Res<GameViewModel>>,
    tile_size: Res<TileSize>,
    sprites: Option<Res<SpriteAssets>>,
    mut tiles_spawned: Local<bool>,
    existing_tiles: Query<Entity, With<Tile>>,
) {
    let Some(view_model) = view_model else {
        return;
    };

    let Some(sprites) = sprites else {
        return;
    };

    // Only spawn once (tiles are static)
    if *tiles_spawned {
        return;
    }

    // Clear any existing tiles
    for entity in existing_tiles.iter() {
        commands.entity(entity).despawn();
    }

    let map = &view_model.0.map;
    let tile_px = tile_size.0;

    // Calculate offset to center the map
    let map_width = map.width as f32 * tile_px;
    let map_height = map.height as f32 * tile_px;
    let offset_x = -map_width / 2.0 + tile_px / 2.0;
    let offset_y = -map_height / 2.0 + tile_px / 2.0;

    for (row_idx, row) in map.tiles.iter().enumerate() {
        for (col_idx, tile_view) in row.iter().enumerate() {
            let texture = sprites.tile_sprite(tile_view.terrain);

            // Convert grid position to world position
            // Note: tiles are stored in Y-reversed order (top row first)
            let world_x = col_idx as f32 * tile_px + offset_x;
            let world_y = (map.height as usize - 1 - row_idx) as f32 * tile_px + offset_y;

            commands.spawn((
                Sprite {
                    image: texture,
                    custom_size: Some(Vec2::splat(tile_px)),
                    ..default()
                },
                Transform::from_xyz(world_x, world_y, 0.0),
                Tile {
                    position: tile_view.position,
                },
            ));
        }
    }

    *tiles_spawned = true;
    tracing::info!("Spawned {} tiles with sprites", map.width * map.height);
}
