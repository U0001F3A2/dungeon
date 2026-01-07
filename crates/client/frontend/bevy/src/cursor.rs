//! Cursor and hover systems for tile examination.
//!
//! Provides mouse hover detection for examining tiles/entities.
//! Attack targeting is handled via context menu (right-click).

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use game_core::Position;

use crate::components::MainCamera;
use crate::resources::{GameViewModel, TileSize};

/// Plugin for hover systems.
pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HoverState>()
            .add_systems(Update, update_hover_position);
    }
}

/// Mouse hover state for tile/entity inspection.
#[derive(Resource, Default)]
pub struct HoverState {
    /// Current hovered grid position (if any).
    pub position: Option<Position>,
    /// What entity is being hovered (if any).
    pub target: Option<HoverTarget>,
}

/// What the mouse is hovering over.
#[derive(Clone, Debug)]
pub enum HoverTarget {
    Actor {
        id: game_core::EntityId,
        is_player: bool,
    },
    Item {
        id: game_core::EntityId,
    },
    Prop {
        id: game_core::EntityId,
    },
}

impl HoverState {
    /// Update hover state from view model based on current position.
    pub fn update_target(&mut self, view_model: &client_frontend_core::view_model::ViewModel) {
        let Some(pos) = self.position else {
            self.target = None;
            return;
        };

        // Check actors first
        for actor in &view_model.actors {
            if actor.position == Some(pos) {
                self.target = Some(HoverTarget::Actor {
                    id: actor.id,
                    is_player: actor.is_player,
                });
                return;
            }
        }

        // Check items
        for item in &view_model.items {
            if item.position == pos {
                self.target = Some(HoverTarget::Item { id: item.id });
                return;
            }
        }

        // Check props
        for prop in &view_model.props {
            if prop.position == pos {
                self.target = Some(HoverTarget::Prop { id: prop.id });
                return;
            }
        }

        self.target = None;
    }
}

/// Update hover position from mouse cursor.
fn update_hover_position(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    view_model: Option<Res<GameViewModel>>,
    tile_size: Res<TileSize>,
    mut hover_state: ResMut<HoverState>,
) {
    let Some(view_model) = view_model else {
        hover_state.position = None;
        hover_state.target = None;
        return;
    };

    let Ok(window) = windows.get_single() else {
        return;
    };

    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        hover_state.position = None;
        hover_state.target = None;
        return;
    };

    // Convert screen position to world position
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        hover_state.position = None;
        hover_state.target = None;
        return;
    };

    let map = &view_model.0.map;
    let tile_px = tile_size.0;

    // Calculate map offset (same as tile rendering)
    let map_width_px = map.width as f32 * tile_px;
    let map_height_px = map.height as f32 * tile_px;
    let offset_x = -map_width_px / 2.0;
    let offset_y = -map_height_px / 2.0;

    // Convert world position to grid coordinates
    let grid_x = ((world_pos.x - offset_x) / tile_px).floor() as i32;
    let grid_y = ((world_pos.y - offset_y) / tile_px).floor() as i32;

    // Check bounds
    if grid_x >= 0 && grid_x < map.width as i32 && grid_y >= 0 && grid_y < map.height as i32 {
        let new_pos = Position::new(grid_x, grid_y);
        let position_changed = hover_state.position != Some(new_pos);

        // Update position
        if position_changed {
            hover_state.position = Some(new_pos);
        }

        // Update target if position changed OR game state changed (actors moved, etc.)
        if position_changed || view_model.is_changed() {
            hover_state.update_target(&view_model.0);
        }
    } else {
        hover_state.position = None;
        hover_state.target = None;
    }
}
