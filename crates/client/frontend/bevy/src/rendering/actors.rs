//! Actor (player and NPC) rendering systems.

use bevy::prelude::*;

use crate::components::{Actor, MainCamera, Npc, Player};
use crate::resources::{CameraConfig, GameViewModel, TileSize};

/// Spawn actor sprites from the view model.
pub fn spawn_actors(
    mut commands: Commands,
    view_model: Option<Res<GameViewModel>>,
    tile_size: Res<TileSize>,
    existing_actors: Query<Entity, With<Actor>>,
    mut actors_spawned: Local<bool>,
) {
    let Some(view_model) = view_model else {
        return;
    };

    // Only spawn once initially (updates handled separately)
    if *actors_spawned {
        return;
    }

    // Clear existing actors
    for entity in existing_actors.iter() {
        commands.entity(entity).despawn();
    }

    let tile_px = tile_size.0;
    let map = &view_model.0.map;

    // Calculate offset (same as tiles)
    let map_width = map.width as f32 * tile_px;
    let map_height = map.height as f32 * tile_px;
    let offset_x = -map_width / 2.0 + tile_px / 2.0;
    let offset_y = -map_height / 2.0 + tile_px / 2.0;

    for actor in &view_model.0.actors {
        let Some(pos) = actor.position else {
            continue;
        };

        let world_x = pos.x as f32 * tile_px + offset_x;
        let world_y = pos.y as f32 * tile_px + offset_y;

        let (color, size) = if actor.is_player {
            (Color::srgb(0.2, 0.8, 0.3), tile_px * 0.8)
        } else {
            (Color::srgb(0.8, 0.2, 0.2), tile_px * 0.6)
        };

        let mut entity_commands = commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::splat(size)),
                ..default()
            },
            Transform::from_xyz(world_x, world_y, 1.0), // Z = 1 to render above tiles
            Actor {
                entity_id: actor.id,
            },
        ));

        if actor.is_player {
            entity_commands.insert(Player);
        } else {
            entity_commands.insert(Npc);
        }
    }

    *actors_spawned = true;
    tracing::info!("Spawned {} actors", view_model.0.actors.len());
}

/// Update actor positions when the view model changes.
pub fn update_actor_positions(
    view_model: Option<Res<GameViewModel>>,
    tile_size: Res<TileSize>,
    mut actors: Query<(&Actor, &mut Transform)>,
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

    for (actor_component, mut transform) in actors.iter_mut() {
        // Find the actor in the view model
        if let Some(actor_view) = view_model
            .0
            .actors
            .iter()
            .find(|a| a.id == actor_component.entity_id)
        {
            if let Some(pos) = actor_view.position {
                let world_x = pos.x as f32 * tile_px + offset_x;
                let world_y = pos.y as f32 * tile_px + offset_y;
                transform.translation.x = world_x;
                transform.translation.y = world_y;
            }
        }
    }
}

/// Update camera to follow the player.
pub fn update_camera_follow(
    view_model: Option<Res<GameViewModel>>,
    tile_size: Res<TileSize>,
    camera_config: Res<CameraConfig>,
    mut camera: Query<&mut Transform, (With<MainCamera>, Without<Actor>)>,
) {
    if !camera_config.follow_player {
        return;
    }

    let Some(view_model) = view_model else {
        return;
    };

    let Some(player_pos) = view_model.0.player.position else {
        return;
    };

    let Ok(mut camera_transform) = camera.get_single_mut() else {
        return;
    };

    let tile_px = tile_size.0;
    let map = &view_model.0.map;

    let map_width = map.width as f32 * tile_px;
    let map_height = map.height as f32 * tile_px;
    let offset_x = -map_width / 2.0 + tile_px / 2.0;
    let offset_y = -map_height / 2.0 + tile_px / 2.0;

    let target_x = player_pos.x as f32 * tile_px + offset_x;
    let target_y = player_pos.y as f32 * tile_px + offset_y;

    // Smooth camera follow
    let lerp_factor = 0.1;
    camera_transform.translation.x +=
        (target_x - camera_transform.translation.x) * lerp_factor;
    camera_transform.translation.y +=
        (target_y - camera_transform.translation.y) * lerp_factor;
}
