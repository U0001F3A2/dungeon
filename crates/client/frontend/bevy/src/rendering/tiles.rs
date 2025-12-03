//! Tile rendering systems.

use bevy::prelude::*;
use game_core::env::TerrainKind;

use crate::components::Tile;
use crate::resources::{GameViewModel, TileSize};

/// Marker to track if tiles have been spawned.
#[derive(Resource, Default)]
pub struct TilesSpawned(pub bool);

/// Spawn tile sprites from the view model.
pub fn spawn_tiles(
    mut commands: Commands,
    view_model: Option<Res<GameViewModel>>,
    tile_size: Res<TileSize>,
    mut tiles_spawned: Local<bool>,
    existing_tiles: Query<Entity, With<Tile>>,
) {
    let Some(view_model) = view_model else {
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
            let color = terrain_color(tile_view.terrain);

            // Convert grid position to world position
            // Note: tiles are stored in Y-reversed order (top row first)
            let world_x = col_idx as f32 * tile_px + offset_x;
            let world_y = (map.height as usize - 1 - row_idx) as f32 * tile_px + offset_y;

            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::splat(tile_px - 1.0)), // Small gap between tiles
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
    tracing::info!("Spawned {} tiles", map.width * map.height);
}

/// Get color for terrain type.
fn terrain_color(terrain: TerrainKind) -> Color {
    match terrain {
        TerrainKind::Floor => Color::srgb(0.3, 0.3, 0.35),
        TerrainKind::Wall => Color::srgb(0.5, 0.4, 0.3),
        TerrainKind::Void => Color::srgb(0.05, 0.05, 0.1),
        TerrainKind::Water => Color::srgb(0.2, 0.4, 0.8),
        TerrainKind::Custom(_) => Color::srgb(0.6, 0.2, 0.6),
    }
}
