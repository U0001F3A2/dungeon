//! Input handling systems for player controls.
//!
//! Keyboard controls for movement, pickup, and wait actions.
//! Attack is handled via context menu (right-click).

use bevy::prelude::*;
use game_core::{Action, ActionInput, ActionKind, CardinalDirection, CharacterAction, EntityId};

use crate::context_menu::ContextMenuState;
use crate::resources::{ActionSender, GameViewModel};

/// Plugin for input handling systems.
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShowHelp>()
            .add_systems(Update, (handle_keyboard_input, toggle_help));
    }
}

/// Whether to show the help overlay.
#[derive(Resource, Default)]
pub struct ShowHelp(pub bool);

/// Handle keyboard input and send actions to the runtime.
fn handle_keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    action_sender: Option<Res<ActionSender>>,
    view_model: Option<Res<GameViewModel>>,
    context_menu: Res<ContextMenuState>,
) {
    // Don't process keyboard input when context menu is open
    if context_menu.visible {
        return;
    }

    let Some(action_sender) = action_sender else {
        return;
    };

    let Some(view_model) = view_model else {
        return;
    };

    // Check if it's the player's turn
    if view_model.0.turn.current_actor != EntityId::PLAYER {
        return;
    }

    // Pickup item at current position with G
    if keys.just_pressed(KeyCode::KeyG) {
        // Find item at player's position
        if let Some(player_pos) = view_model.0.player.position {
            if let Some(item) = view_model.0.items.iter().find(|i| i.position == player_pos) {
                let action = Action::Character(CharacterAction::new(
                    EntityId::PLAYER,
                    ActionKind::PickupItem,
                    ActionInput::Target(item.id),
                ));

                if let Err(e) = action_sender.0.try_send(action) {
                    tracing::warn!("Failed to send pickup action: {}", e);
                }
            } else {
                tracing::info!("No item at current position to pick up");
            }
        }
        return;
    }

    // Movement with arrow keys
    if let Some(dir) = get_direction(&keys) {
        let action = Action::Character(CharacterAction::new(
            EntityId::PLAYER,
            ActionKind::Move,
            ActionInput::Direction(dir),
        ));

        if let Err(e) = action_sender.0.try_send(action) {
            tracing::warn!("Failed to send action: {}", e);
        }
    }

    // Wait action with period or space
    if keys.just_pressed(KeyCode::Period) || keys.just_pressed(KeyCode::Space) {
        let action = Action::Character(CharacterAction::new(
            EntityId::PLAYER,
            ActionKind::Wait,
            ActionInput::None,
        ));

        if let Err(e) = action_sender.0.try_send(action) {
            tracing::warn!("Failed to send wait action: {}", e);
        }
    }
}

/// Toggle help overlay with H or F1.
fn toggle_help(keys: Res<ButtonInput<KeyCode>>, mut show_help: ResMut<ShowHelp>) {
    if keys.just_pressed(KeyCode::KeyH) || keys.just_pressed(KeyCode::F1) {
        show_help.0 = !show_help.0;
    }
}

/// Get direction from arrow keys.
fn get_direction(keys: &ButtonInput<KeyCode>) -> Option<CardinalDirection> {
    if keys.just_pressed(KeyCode::ArrowUp) {
        return Some(CardinalDirection::North);
    }
    if keys.just_pressed(KeyCode::ArrowDown) {
        return Some(CardinalDirection::South);
    }
    if keys.just_pressed(KeyCode::ArrowRight) {
        return Some(CardinalDirection::East);
    }
    if keys.just_pressed(KeyCode::ArrowLeft) {
        return Some(CardinalDirection::West);
    }
    None
}
