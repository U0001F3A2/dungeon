//! UI systems for stats panel, message log, and other HUD elements.

mod panels;
mod styles;

pub use panels::*;

use bevy::prelude::*;

/// Plugin for UI systems.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, (update_stats_panel, update_message_log));
    }
}
