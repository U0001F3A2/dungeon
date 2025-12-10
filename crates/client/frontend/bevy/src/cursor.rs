//! Cursor system for tile examination and target selection.
//!
//! Provides a visual cursor that can be moved around the map to examine
//! tiles and entities, or to select targets for actions.

use bevy::prelude::*;
use game_core::Position;

use crate::resources::{GameViewModel, TileSize};

/// Plugin for cursor systems.
pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorState>()
            .add_systems(Startup, spawn_cursor)
            .add_systems(Update, (update_cursor_position, update_cursor_visibility));
    }
}

/// Cursor state resource tracking position and visibility.
#[derive(Resource)]
pub struct CursorState {
    /// Current cursor position in world grid coordinates.
    pub position: Position,
    /// Whether the cursor is currently visible (examine mode active).
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

    /// Get entity at cursor position from view model.
    pub fn get_entity_at_cursor<'a>(
        &self,
        view_model: &'a client_frontend_core::view_model::ViewModel,
    ) -> Option<CursorTarget<'a>> {
        // Check actors first
        for actor in &view_model.actors {
            if actor.position == Some(self.position) {
                return Some(CursorTarget::Actor(actor));
            }
        }

        // Check items
        for item in &view_model.items {
            if item.position == self.position {
                return Some(CursorTarget::Item(item));
            }
        }

        // Check props
        for prop in &view_model.props {
            if prop.position == self.position {
                return Some(CursorTarget::Prop(prop));
            }
        }

        None
    }
}

/// What the cursor is currently targeting.
pub enum CursorTarget<'a> {
    Actor(&'a client_frontend_core::view_model::entities::ActorView),
    Item(&'a client_frontend_core::view_model::entities::ItemView),
    Prop(&'a client_frontend_core::view_model::entities::PropView),
}

/// Marker component for the cursor sprite.
#[derive(Component)]
pub struct CursorSprite;

/// Spawn the cursor sprite (initially hidden).
fn spawn_cursor(mut commands: Commands) {
    // Create a simple colored square for the cursor
    commands.spawn((
        Sprite {
            color: Color::srgba(1.0, 1.0, 0.0, 0.5), // Semi-transparent yellow
            custom_size: Some(Vec2::splat(32.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0), // High Z to render on top
        Visibility::Hidden,
        CursorSprite,
    ));
}

/// Update cursor sprite position to match cursor state.
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

/// Update cursor visibility based on state.
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
