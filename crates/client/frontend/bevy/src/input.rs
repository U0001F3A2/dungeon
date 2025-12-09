//! Input handling systems for player controls.

use bevy::prelude::*;
use game_core::{Action, ActionInput, ActionKind, CardinalDirection, CharacterAction, EntityId};

use crate::cursor::CursorState;
use crate::resources::{ActionSender, GameViewModel};

/// Plugin for input handling systems.
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputMode>()
            .init_resource::<ShowHelp>()
            .add_systems(Update, (handle_keyboard_input, toggle_help, handle_examine_input));
    }
}

/// Current input mode for the player.
#[derive(Resource, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Normal,
    /// Attack mode - use cursor to select target, Enter to confirm
    Attack,
    Pickup,
    Examine,
}

/// Whether to show the help overlay.
#[derive(Resource, Default)]
pub struct ShowHelp(pub bool);

/// Handle keyboard input and send actions to the runtime.
fn handle_keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    action_sender: Option<Res<ActionSender>>,
    view_model: Option<Res<GameViewModel>>,
    mut input_mode: ResMut<InputMode>,
    mut cursor_state: ResMut<CursorState>,
) {
    let Some(action_sender) = action_sender else {
        return;
    };

    let Some(view_model) = view_model else {
        return;
    };

    // Toggle examine mode with X
    if keys.just_pressed(KeyCode::KeyX) {
        if *input_mode == InputMode::Examine {
            *input_mode = InputMode::Normal;
            cursor_state.visible = false;
        } else {
            *input_mode = InputMode::Examine;
            cursor_state.visible = true;
            // Initialize cursor at player position
            if let Some(player_pos) = view_model.0.player.position {
                cursor_state.init_at_player(
                    player_pos,
                    view_model.0.map.width,
                    view_model.0.map.height,
                );
            }
        }
        return;
    }

    // Handle attack mode
    if *input_mode == InputMode::Attack {
        // Move cursor with arrow keys
        if keys.just_pressed(KeyCode::ArrowUp) {
            cursor_state.move_by(0, 1);
        }
        if keys.just_pressed(KeyCode::ArrowDown) {
            cursor_state.move_by(0, -1);
        }
        if keys.just_pressed(KeyCode::ArrowRight) {
            cursor_state.move_by(1, 0);
        }
        if keys.just_pressed(KeyCode::ArrowLeft) {
            cursor_state.move_by(-1, 0);
        }

        // Confirm attack with Enter or Space
        if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space) {
            // Find target at cursor position
            if let Some(target_id) = find_attackable_target(&cursor_state, &view_model.0) {
                let action = Action::Character(CharacterAction::new(
                    EntityId::PLAYER,
                    ActionKind::MeleeAttack,
                    ActionInput::Target(target_id),
                ));

                if let Err(e) = action_sender.0.try_send(action) {
                    tracing::warn!("Failed to send attack action: {}", e);
                }
            } else {
                tracing::info!("No valid target at cursor position");
            }

            // Reset to normal mode
            *input_mode = InputMode::Normal;
            cursor_state.visible = false;
        }

        // Cancel with Escape
        if keys.just_pressed(KeyCode::Escape) {
            *input_mode = InputMode::Normal;
            cursor_state.visible = false;
        }

        return;
    }

    // In examine mode, don't process game actions
    if *input_mode == InputMode::Examine {
        return;
    }

    // Check if it's the player's turn
    if view_model.0.turn.current_actor != EntityId::PLAYER {
        return;
    }

    // Mode switching
    if keys.just_pressed(KeyCode::KeyA) {
        *input_mode = InputMode::Attack;
        cursor_state.visible = true;
        // Initialize cursor at player position
        if let Some(player_pos) = view_model.0.player.position {
            cursor_state.init_at_player(
                player_pos,
                view_model.0.map.width,
                view_model.0.map.height,
            );
        }
        tracing::info!("Attack mode - move cursor to target and press Enter");
        return;
    }
    if keys.just_pressed(KeyCode::KeyG) {
        *input_mode = InputMode::Pickup;
        tracing::info!("Pickup mode - press arrow key to pick up item");
        return;
    }
    if keys.just_pressed(KeyCode::Escape) {
        *input_mode = InputMode::Normal;
        cursor_state.visible = false;
        return;
    }

    // Movement/action with arrow keys only
    let direction = get_direction(&keys);

    if let Some(dir) = direction {
        let action = match *input_mode {
            InputMode::Normal => Action::Character(CharacterAction::new(
                EntityId::PLAYER,
                ActionKind::Move,
                ActionInput::Direction(dir),
            )),
            InputMode::Attack => return, // Handled above
            InputMode::Pickup => Action::Character(CharacterAction::new(
                EntityId::PLAYER,
                ActionKind::PickupItem,
                ActionInput::Direction(dir),
            )),
            InputMode::Examine => return, // Handled in handle_examine_input
        };

        // Reset to normal mode after action
        *input_mode = InputMode::Normal;

        // Try to send the action (non-blocking)
        if let Err(e) = action_sender.0.try_send(action) {
            tracing::warn!("Failed to send action: {}", e);
        }
    }

    // Wait action with period only (space is used for attack confirm)
    if keys.just_pressed(KeyCode::Period) {
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

/// Find an attackable target at the cursor position.
fn find_attackable_target(
    cursor_state: &CursorState,
    view_model: &client_frontend_core::view_model::ViewModel,
) -> Option<EntityId> {
    let cursor_pos = cursor_state.position;
    let player_pos = view_model.player.position?;

    // Check if cursor is within attack range (1 tile for melee)
    let distance = player_pos.chebyshev_distance(cursor_pos);
    if distance > 1 {
        tracing::info!("Target out of range (distance: {})", distance);
        return None;
    }

    // Find actor at cursor position
    for actor in &view_model.actors {
        if !actor.is_player && actor.position == Some(cursor_pos) {
            return Some(actor.id);
        }
    }

    None
}

/// Handle cursor movement in examine mode.
fn handle_examine_input(
    keys: Res<ButtonInput<KeyCode>>,
    input_mode: Res<InputMode>,
    mut cursor_state: ResMut<CursorState>,
) {
    if *input_mode != InputMode::Examine {
        return;
    }

    // Move cursor with arrow keys
    if keys.just_pressed(KeyCode::ArrowUp) {
        cursor_state.move_by(0, 1);
    }
    if keys.just_pressed(KeyCode::ArrowDown) {
        cursor_state.move_by(0, -1);
    }
    if keys.just_pressed(KeyCode::ArrowRight) {
        cursor_state.move_by(1, 0);
    }
    if keys.just_pressed(KeyCode::ArrowLeft) {
        cursor_state.move_by(-1, 0);
    }
}

/// Toggle help overlay with H or F1.
fn toggle_help(keys: Res<ButtonInput<KeyCode>>, mut show_help: ResMut<ShowHelp>) {
    if keys.just_pressed(KeyCode::KeyH) || keys.just_pressed(KeyCode::F1) {
        show_help.0 = !show_help.0;
    }
}

/// Get direction from arrow keys only.
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
