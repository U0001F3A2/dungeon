//! Bevy ECS components for game entities.

use bevy::prelude::*;
use game_core::{EntityId, Position};

/// Marker component for tile sprites.
#[derive(Component)]
pub struct Tile {
    pub position: Position,
}

/// Marker component for actor sprites (player and NPCs).
#[derive(Component)]
pub struct Actor {
    pub entity_id: EntityId,
}

/// Marker component for the player entity.
#[derive(Component)]
pub struct Player;

/// Marker component for NPC entities.
#[derive(Component)]
pub struct Npc;

/// Marker component for item sprites.
#[derive(Component)]
pub struct Item {
    pub entity_id: EntityId,
}

/// Marker component for prop sprites.
#[derive(Component)]
pub struct Prop {
    pub entity_id: EntityId,
}

/// Marker component for the main game camera.
#[derive(Component)]
pub struct MainCamera;

/// Marker component for UI root.
#[derive(Component)]
pub struct UiRoot;

/// Marker component for the stats panel.
#[derive(Component)]
pub struct StatsPanel;

/// Marker component for the message log panel.
#[derive(Component)]
pub struct MessageLogPanel;

/// Marker component for health text.
#[derive(Component)]
pub struct HealthText;

/// Marker component for mana text.
#[derive(Component)]
pub struct ManaText;

/// Marker component for turn text.
#[derive(Component)]
pub struct TurnText;

/// Marker component for message entries.
#[derive(Component)]
pub struct MessageEntry {
    pub index: usize,
}
