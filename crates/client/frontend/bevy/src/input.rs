//! Input handling systems for player controls.

use bevy::prelude::*;
use game_core::{Action, ActionInput, ActionKind, CardinalDirection, CharacterAction, EntityId};

use crate::resources::{ActionSender, GameViewModel};

/// Plugin for input handling systems.
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_keyboard_input);
    }
}

/// Handle keyboard input and send actions to the runtime.
fn handle_keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    action_sender: Option<Res<ActionSender>>,
    view_model: Option<Res<GameViewModel>>,
) {
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

    // Movement with arrow keys or WASD/numpad
    let direction = get_movement_direction(&keys);

    if let Some(dir) = direction {
        let action = Action::Character(CharacterAction::new(
            EntityId::PLAYER,
            ActionKind::Move,
            ActionInput::Direction(dir),
        ));

        // Try to send the action (non-blocking)
        if let Err(e) = action_sender.0.try_send(action) {
            tracing::warn!("Failed to send action: {}", e);
        }
    }

    // Wait action with space or period
    if keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::Period) {
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

/// Get movement direction from keyboard input.
fn get_movement_direction(keys: &ButtonInput<KeyCode>) -> Option<CardinalDirection> {
    // Arrow keys
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

    // WASD (vi-like: hjkl also supported)
    if keys.just_pressed(KeyCode::KeyW) || keys.just_pressed(KeyCode::KeyK) {
        return Some(CardinalDirection::North);
    }
    if keys.just_pressed(KeyCode::KeyS) || keys.just_pressed(KeyCode::KeyJ) {
        return Some(CardinalDirection::South);
    }
    if keys.just_pressed(KeyCode::KeyD) || keys.just_pressed(KeyCode::KeyL) {
        return Some(CardinalDirection::East);
    }
    if keys.just_pressed(KeyCode::KeyA) || keys.just_pressed(KeyCode::KeyH) {
        return Some(CardinalDirection::West);
    }

    // Diagonals with numpad
    if keys.just_pressed(KeyCode::Numpad7) {
        return Some(CardinalDirection::NorthWest);
    }
    if keys.just_pressed(KeyCode::Numpad9) {
        return Some(CardinalDirection::NorthEast);
    }
    if keys.just_pressed(KeyCode::Numpad1) {
        return Some(CardinalDirection::SouthWest);
    }
    if keys.just_pressed(KeyCode::Numpad3) {
        return Some(CardinalDirection::SouthEast);
    }

    // Diagonals with YUBN (traditional roguelike)
    if keys.just_pressed(KeyCode::KeyY) {
        return Some(CardinalDirection::NorthWest);
    }
    if keys.just_pressed(KeyCode::KeyU) {
        return Some(CardinalDirection::NorthEast);
    }
    if keys.just_pressed(KeyCode::KeyB) {
        return Some(CardinalDirection::SouthWest);
    }
    if keys.just_pressed(KeyCode::KeyN) {
        return Some(CardinalDirection::SouthEast);
    }

    None
}
