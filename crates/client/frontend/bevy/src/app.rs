//! Bevy frontend implementation.
//!
//! Pure UI layer that communicates with the game via RuntimeHandle only.

use anyhow::Result;
use async_trait::async_trait;
use bevy::prelude::*;
use client_bootstrap::oracles::OracleBundle;
use client_frontend_core::view_model::ViewModel;
use client_frontend_core::{FrontendConfig, MessageLog};
use game_core::{Action, EntityId};
use runtime::{InteractiveKind, ProviderKind, RuntimeHandle, Topic};
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::events::{RuntimeEventReceivers, RuntimeEventsPlugin};
use crate::input::InputPlugin;
use crate::provider::BevyActionProvider;
use crate::rendering::RenderingPlugin;
use crate::resources::*;
use crate::ui::UiPlugin;

/// Bevy frontend (pure UI layer).
///
/// This struct handles:
/// - 2D tile-based rendering
/// - User input collection
/// - Event consumption from runtime
/// - Action submission to runtime
///
/// All communication with the game happens via RuntimeHandle.
pub struct BevyFrontend {
    config: FrontendConfig,
    oracles: OracleBundle,
}

impl BevyFrontend {
    /// Create a new Bevy frontend.
    pub fn new(config: FrontendConfig, oracles: OracleBundle) -> Self {
        Self { config, oracles }
    }
}

#[async_trait]
impl client_frontend_core::Frontend for BevyFrontend {
    async fn run(&mut self, handle: RuntimeHandle) -> Result<()> {
        tracing::info!("Bevy frontend starting...");

        // Setup action provider (interactive input)
        let (tx_action, rx_action) = mpsc::channel::<Action>(self.config.channels.action_buffer);

        let bevy_kind = ProviderKind::Interactive(InteractiveKind::BevyInput);

        // Register Bevy input provider for player
        handle.register_provider(bevy_kind, BevyActionProvider::new(rx_action))?;

        // Bind player to Bevy input
        handle.bind_entity_provider(EntityId::PLAYER, bevy_kind)?;

        // Subscribe to events
        let subscriptions = handle.subscribe_multiple(&[Topic::GameState, Topic::Proof]);
        let initial_state = handle.query_state().await?;

        // Initialize message log
        let mut messages = MessageLog::new(self.config.messages.capacity);
        messages.push_text(format!(
            "[{}] Welcome to the dungeon.",
            initial_state.turn.clock
        ));

        // Create initial view model
        let view_model = ViewModel::from_initial_state(&initial_state, self.oracles.map.as_ref());

        // Prepare resources
        let game_view_model = GameViewModel(view_model);
        let game_message_log = GameMessageLog(messages);
        let action_sender = ActionSender(tx_action);
        let runtime_handle = GameRuntimeHandle(handle.clone());
        let frontend_config = GameFrontendConfig(self.config.clone());
        let tile_size = TileSize(32.0);
        let camera_config = CameraConfig::default();
        let oracle_bundle = crate::resources::OracleBundle(Arc::new(self.oracles.clone()));
        let event_receivers = RuntimeEventReceivers {
            receivers: subscriptions,
        };

        tracing::info!("Bevy app starting with {} actors", game_view_model.0.actors.len());

        // Run Bevy app (blocks until window is closed)
        // Note: Bevy's App::run() takes ownership and doesn't return
        // We run it in a blocking context since Bevy needs the main thread
        App::new()
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Dungeon".to_string(),
                    resolution: (1280.0, 720.0).into(),
                    ..default()
                }),
                ..default()
            }))
            // Insert resources
            .insert_resource(game_view_model)
            .insert_resource(game_message_log)
            .insert_resource(action_sender)
            .insert_resource(runtime_handle)
            .insert_resource(frontend_config)
            .insert_resource(tile_size)
            .insert_resource(camera_config)
            .insert_resource(oracle_bundle)
            .insert_resource(event_receivers)
            // Add plugins
            .add_plugins(RenderingPlugin)
            .add_plugins(UiPlugin)
            .add_plugins(InputPlugin)
            .add_plugins(RuntimeEventsPlugin)
            // Run
            .run();

        tracing::info!("Bevy frontend exiting");
        Ok(())
    }
}
