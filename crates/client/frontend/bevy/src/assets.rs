//! Sprite asset loading and management.

use bevy::prelude::*;
use game_core::env::TerrainKind;
use game_core::PropKind;

/// Resource holding all loaded sprite handles.
#[derive(Resource)]
pub struct SpriteAssets {
    // Terrain tiles
    pub tile_floor: Handle<Image>,
    pub tile_wall: Handle<Image>,
    pub tile_void: Handle<Image>,
    pub tile_water: Handle<Image>,
    pub tile_custom: Handle<Image>,

    // Actors
    pub actor_player: Handle<Image>,
    pub actor_goblin: Handle<Image>,
    pub actor_skeleton: Handle<Image>,
    pub actor_slime: Handle<Image>,

    // Props
    pub prop_door_closed: Handle<Image>,
    pub prop_door_open: Handle<Image>,
    pub prop_switch_off: Handle<Image>,
    pub prop_switch_on: Handle<Image>,
    pub prop_hazard: Handle<Image>,

    // Items
    pub item_potion_health: Handle<Image>,
    pub item_potion_mana: Handle<Image>,
    pub item_sword: Handle<Image>,
    pub item_shield: Handle<Image>,
    pub item_key: Handle<Image>,
    pub item_gold: Handle<Image>,
}

impl SpriteAssets {
    /// Load all sprite assets from the assets/sprites directory.
    pub fn load(asset_server: &AssetServer) -> Self {
        Self {
            // Terrain tiles
            tile_floor: asset_server.load("sprites/tile_floor.png"),
            tile_wall: asset_server.load("sprites/tile_wall.png"),
            tile_void: asset_server.load("sprites/tile_void.png"),
            tile_water: asset_server.load("sprites/tile_water.png"),
            tile_custom: asset_server.load("sprites/tile_custom.png"),

            // Actors
            actor_player: asset_server.load("sprites/actor_player.png"),
            actor_goblin: asset_server.load("sprites/actor_goblin.png"),
            actor_skeleton: asset_server.load("sprites/actor_skeleton.png"),
            actor_slime: asset_server.load("sprites/actor_slime.png"),

            // Props
            prop_door_closed: asset_server.load("sprites/prop_door_closed.png"),
            prop_door_open: asset_server.load("sprites/prop_door_open.png"),
            prop_switch_off: asset_server.load("sprites/prop_switch_off.png"),
            prop_switch_on: asset_server.load("sprites/prop_switch_on.png"),
            prop_hazard: asset_server.load("sprites/prop_hazard.png"),

            // Items
            item_potion_health: asset_server.load("sprites/item_potion_health.png"),
            item_potion_mana: asset_server.load("sprites/item_potion_mana.png"),
            item_sword: asset_server.load("sprites/item_sword.png"),
            item_shield: asset_server.load("sprites/item_shield.png"),
            item_key: asset_server.load("sprites/item_key.png"),
            item_gold: asset_server.load("sprites/item_gold.png"),
        }
    }

    /// Get the sprite handle for a terrain type.
    pub fn tile_sprite(&self, terrain: TerrainKind) -> Handle<Image> {
        match terrain {
            TerrainKind::Floor => self.tile_floor.clone(),
            TerrainKind::Wall => self.tile_wall.clone(),
            TerrainKind::Void => self.tile_void.clone(),
            TerrainKind::Water => self.tile_water.clone(),
            TerrainKind::Custom(_) => self.tile_custom.clone(),
        }
    }

    /// Get the sprite handle for a prop type.
    pub fn prop_sprite(&self, kind: &PropKind, is_active: bool) -> Handle<Image> {
        match kind {
            PropKind::Door => {
                if is_active {
                    self.prop_door_open.clone()
                } else {
                    self.prop_door_closed.clone()
                }
            }
            PropKind::Switch => {
                if is_active {
                    self.prop_switch_on.clone()
                } else {
                    self.prop_switch_off.clone()
                }
            }
            PropKind::Hazard => self.prop_hazard.clone(),
            PropKind::Other => self.prop_hazard.clone(), // Fallback
        }
    }

    /// Get the player sprite handle.
    pub fn player_sprite(&self) -> Handle<Image> {
        self.actor_player.clone()
    }

    /// Get an NPC sprite handle (cycles through available enemy types based on entity ID).
    pub fn npc_sprite(&self, entity_id: u32) -> Handle<Image> {
        // Cycle through available enemy sprites for variety
        match entity_id % 3 {
            0 => self.actor_goblin.clone(),
            1 => self.actor_skeleton.clone(),
            _ => self.actor_slime.clone(),
        }
    }

    /// Get a default item sprite (gold coins for now).
    pub fn item_sprite(&self, _handle: u32) -> Handle<Image> {
        // TODO: Map item handles to specific sprites based on item type
        // For now, return gold as a default
        self.item_gold.clone()
    }
}

/// System to load sprite assets at startup.
pub fn load_sprite_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprites = SpriteAssets::load(&asset_server);
    commands.insert_resource(sprites);
    tracing::info!("Sprite assets loaded");
}
