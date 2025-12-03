//! Bevy resources for game state and runtime communication.

use bevy::prelude::*;
use client_frontend_core::view_model::ViewModel;
use client_frontend_core::{MessageLog, FrontendConfig};
use runtime::RuntimeHandle;
use std::sync::Arc;
use tokio::sync::mpsc;
use game_core::Action;

/// Game view model resource, synchronized with runtime events.
#[derive(Resource)]
pub struct GameViewModel(pub ViewModel);

/// Message log for displaying game events.
#[derive(Resource)]
pub struct GameMessageLog(pub MessageLog);

/// Channel for sending player actions to the runtime.
#[derive(Resource)]
pub struct ActionSender(pub mpsc::Sender<Action>);

/// Runtime handle for querying state and subscribing to events.
#[derive(Resource)]
pub struct GameRuntimeHandle(pub RuntimeHandle);

/// Frontend configuration.
#[derive(Resource)]
pub struct GameFrontendConfig(pub FrontendConfig);

/// Tile size in pixels for rendering.
#[derive(Resource)]
pub struct TileSize(pub f32);

impl Default for TileSize {
    fn default() -> Self {
        Self(32.0)
    }
}

/// Camera configuration.
#[derive(Resource)]
pub struct CameraConfig {
    pub zoom: f32,
    pub follow_player: bool,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            follow_player: true,
        }
    }
}

/// Flag indicating the view model needs to be re-synced.
#[derive(Resource, Default)]
pub struct ViewModelDirty(pub bool);

/// Oracle bundle for map lookups (wrapped in Arc for thread safety).
#[derive(Resource)]
pub struct OracleBundle(pub Arc<client_bootstrap::OracleBundle>);
