//! Cursor and hover systems for tile examination and target selection.
//!
//! Provides:
//! - Mouse hover detection for examining tiles/entities
//! - Keyboard cursor for attack target selection

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use game_core::Position;

use crate::components::MainCamera;
use crate::resources::{GameViewModel, TileSize};

/// Plugin for cursor and hover systems.
pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorState>()
            .init_resource::<HoverState>()
            .add_systems(Startup, spawn_cursor)
            .add_systems(Update, (
                update_hover_position,
                update_cursor_position,
                update_cursor_visibility,
            ));
    }
}

/// Keyboard cursor state for attack targeting.
#[derive(Resource)]
pub struct CursorState {
    /// Current cursor position in world grid coordinates.
    pub position: Position,
    /// Whether the cursor is currently visible (attack mode active).
    pub visible: bool,
    /// Map dimensions for bounds checking.
    pub map_width: u32,
    pub map_height: u32,
}

impl Default for CursorState {
    fn default() -> Self {
        Self {
            position: Position::new(0, 0),
            visible: false,
            map_width: 20,
            map_height: 20,
        }
    }
}

impl CursorState {
    /// Initialize cursor at player position.
    pub fn init_at_player(&mut self, player_pos: Position, map_width: u32, map_height: u32) {
        self.position = player_pos;
        self.map_width = map_width;
        self.map_height = map_height;
    }

    /// Move cursor by delta, clamped to map bounds.
    pub fn move_by(&mut self, dx: i32, dy: i32) {
        let new_x = (self.position.x + dx).clamp(0, self.map_width as i32 - 1);
        let new_y = (self.position.y + dy).clamp(0, self.map_height as i32 - 1);
        self.position = Position::new(new_x, new_y);
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
    Actor { id: game_core::EntityId, is_player: bool },
    Item { id: game_core::EntityId },
    Prop { id: game_core::EntityId },
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

/// Marker component for the attack cursor sprite.
#[derive(Component)]
pub struct CursorSprite;

/// Spawn the attack cursor sprite (initially hidden).
fn spawn_cursor(mut commands: Commands) {
    // Create a simple colored square for the attack cursor
    commands.spawn((
        Sprite {
            color: Color::srgba(1.0, 0.3, 0.3, 0.6), // Semi-transparent red for attack
            custom_size: Some(Vec2::splat(32.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0), // High Z to render on top
        Visibility::Hidden,
        CursorSprite,
    ));
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

/// Update attack cursor sprite position to match cursor state.
fn update_cursor_position(
    cursor_state: Res<CursorState>,
    tile_size: Res<TileSize>,
    view_model: Option<Res<GameViewModel>>,
    mut cursor_query: Query<&mut Transform, With<CursorSprite>>,
) {
    let Some(view_model) = view_model else {
        return;
    };

    let Ok(mut transform) = cursor_query.get_single_mut() else {
        return;
    };

    let map = &view_model.0.map;
    let tile_px = tile_size.0;

    // Calculate offset (same as tile rendering)
    let map_width = map.width as f32 * tile_px;
    let map_height = map.height as f32 * tile_px;
    let offset_x = -map_width / 2.0 + tile_px / 2.0;
    let offset_y = -map_height / 2.0 + tile_px / 2.0;

    // Convert grid position to world position
    let world_x = cursor_state.position.x as f32 * tile_px + offset_x;
    let world_y = cursor_state.position.y as f32 * tile_px + offset_y;

    transform.translation.x = world_x;
    transform.translation.y = world_y;
}

/// Update attack cursor visibility based on state.
fn update_cursor_visibility(
    cursor_state: Res<CursorState>,
    mut cursor_query: Query<&mut Visibility, With<CursorSprite>>,
) {
    let Ok(mut visibility) = cursor_query.get_single_mut() else {
        return;
    };

    *visibility = if cursor_state.visible {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
}
