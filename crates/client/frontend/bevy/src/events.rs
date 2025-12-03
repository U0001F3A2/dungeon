//! Runtime event integration for Bevy.
//!
//! This module provides systems for receiving events from the runtime
//! and updating the Bevy game state accordingly.

use bevy::prelude::*;
use client_frontend_core::{MessageLog, ViewModelUpdater};
use runtime::events::{Event, GameStateEvent, Topic};
use std::collections::HashMap;
use tokio::sync::broadcast;

use crate::resources::{GameMessageLog, GameViewModel, OracleBundle, ViewModelDirty};

/// Resource holding event receivers from the runtime.
#[derive(Resource)]
pub struct RuntimeEventReceivers {
    pub receivers: HashMap<Topic, broadcast::Receiver<Event>>,
}

/// Plugin for runtime event integration.
pub struct RuntimeEventsPlugin;

impl Plugin for RuntimeEventsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ViewModelDirty::default())
            .add_systems(Update, poll_runtime_events);
    }
}

/// Poll runtime events and update game state.
///
/// This system runs every frame and checks for new events from the runtime.
/// When events are received, it updates the ViewModel and marks it as dirty
/// for re-rendering.
fn poll_runtime_events(
    mut receivers: Option<ResMut<RuntimeEventReceivers>>,
    mut view_model: Option<ResMut<GameViewModel>>,
    mut message_log: Option<ResMut<GameMessageLog>>,
    oracles: Option<Res<OracleBundle>>,
    mut dirty: ResMut<ViewModelDirty>,
) {
    let Some(ref mut receivers) = receivers else {
        return;
    };
    let Some(ref mut view_model) = view_model else {
        return;
    };
    let Some(ref oracles) = oracles else {
        return;
    };

    // Poll game state events
    if let Some(rx) = receivers.receivers.get_mut(&Topic::GameState) {
        // Process all available events
        loop {
            match rx.try_recv() {
                Ok(event) => {
                    // Update view model
                    let _scope = ViewModelUpdater::update(
                        &mut view_model.0,
                        &event,
                        oracles.0.map.as_ref(),
                    );

                    // Log messages for significant events
                    if let Some(ref mut log) = message_log {
                        log_event(&event, &mut log.0);
                    }

                    dirty.0 = true;
                }
                Err(broadcast::error::TryRecvError::Empty) => break,
                Err(broadcast::error::TryRecvError::Lagged(n)) => {
                    tracing::warn!("Lagged {} events", n);
                    break;
                }
                Err(broadcast::error::TryRecvError::Closed) => {
                    tracing::error!("Runtime event channel closed");
                    break;
                }
            }
        }
    }
}

/// Log an event to the message log.
fn log_event(event: &Event, log: &mut MessageLog) {
    match event {
        Event::GameState(GameStateEvent::ActionExecuted {
            action,
            action_result,
            ..
        }) => {
            // Format action for display
            let msg = format!("Action executed: {:?}", action);
            log.push_text(msg);

            // Log combat results if any
            if action_result.summary.total_damage > 0 {
                log.push_text(format!("Dealt {} damage", action_result.summary.total_damage));
            }
        }
        Event::GameState(GameStateEvent::ActionFailed { action, error, .. }) => {
            log.push_text(format!("Action failed: {:?} - {}", action, error));
        }
        Event::GameState(GameStateEvent::StateRestored {
            from_nonce,
            to_nonce,
        }) => {
            log.push_text(format!("State restored: {} -> {}", from_nonce, to_nonce));
        }
        Event::Proof(_) => {
            // Proof events are not logged to the message log
        }
        Event::ActionRef(_) => {
            // Action refs are not logged (they're just references)
        }
    }
}
