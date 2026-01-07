//! Prop (doors, switches, hazards) rendering systems.

use bevy::prelude::*;

use crate::assets::SpriteAssets;
use crate::components::Prop;
use crate::resources::{GameViewModel, TileSize};

/// Spawn prop sprites from the view model.
pub fn spawn_props(
    mut commands: Commands,
    view_model: Option<Res<GameViewModel>>,
    tile_size: Res<TileSize>,
    sprites: Option<Res<SpriteAssets>>,
    existing_props: Query<Entity, With<Prop>>,
    mut props_spawned: Local<bool>,
) {
    let Some(view_model) = view_model else {
        return;
    };

    let Some(sprites) = sprites else {
        return;
    };

    // Only spawn once initially (updates handled separately)
    if *props_spawned {
        return;
    }

    // Clear existing props
    for entity in existing_props.iter() {
        commands.entity(entity).despawn();
    }

    let tile_px = tile_size.0;
    let map = &view_model.0.map;

    // Calculate offset (same as tiles)
    let map_width = map.width as f32 * tile_px;
    let map_height = map.height as f32 * tile_px;
    let offset_x = -map_width / 2.0 + tile_px / 2.0;
    let offset_y = -map_height / 2.0 + tile_px / 2.0;

    for prop in &view_model.0.props {
        let world_x = prop.position.x as f32 * tile_px + offset_x;
        let world_y = prop.position.y as f32 * tile_px + offset_y;

        // Get sprite texture based on prop type and state
        let texture = sprites.prop_sprite(&prop.kind, prop.is_active);

        commands.spawn((
            Sprite {
                image: texture,
                custom_size: Some(Vec2::splat(tile_px)),
                ..default()
            },
            Transform::from_xyz(world_x, world_y, 0.5), // Z = 0.5 between tiles and actors
            Prop {
                entity_id: prop.id,
            },
        ));
    }

    *props_spawned = true;
    if !view_model.0.props.is_empty() {
        tracing::info!("Spawned {} props with sprites", view_model.0.props.len());
    }
}

/// Update prop sprites when their state changes.
/// Also despawns props that were destroyed (no longer in view model).
pub fn update_prop_states(
    mut commands: Commands,
    view_model: Option<Res<GameViewModel>>,
    sprites: Option<Res<SpriteAssets>>,
    mut props: Query<(Entity, &Prop, &mut Sprite)>,
) {
    let Some(view_model) = view_model else {
        return;
    };

    let Some(sprites) = sprites else {
        return;
    };

    if !view_model.is_changed() {
        return;
    }

    for (entity, prop_component, mut sprite) in props.iter_mut() {
        // Find the prop in the view model
        if let Some(prop_view) = view_model
            .0
            .props
            .iter()
            .find(|p| p.id == prop_component.entity_id)
        {
            // Update sprite based on current state
            sprite.image = sprites.prop_sprite(&prop_view.kind, prop_view.is_active);
        } else {
            // Prop was destroyed - despawn the entity
            commands.entity(entity).despawn();
        }
    }
}
