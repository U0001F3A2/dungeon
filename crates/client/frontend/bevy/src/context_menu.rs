//! Context menu system for right-click interactions.
//!
//! Provides a context menu that appears when right-clicking on entities,
//! showing available actions like Attack, Pickup, Interact.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use game_core::{Action, ActionInput, ActionKind, CharacterAction, EntityId};

use crate::cursor::{HoverState, HoverTarget};
use crate::resources::{ActionSender, GameViewModel};

/// Plugin for context menu system.
pub struct ContextMenuPlugin;

impl Plugin for ContextMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ContextMenuState>()
            .add_systems(Startup, spawn_context_menu)
            .add_systems(
                Update,
                (
                    handle_right_click,
                    update_context_menu_position,
                    handle_menu_buttons,
                    close_menu_on_click_outside,
                ),
            );
    }
}

/// Current state of the context menu.
#[derive(Resource, Default)]
pub struct ContextMenuState {
    /// Whether the menu is currently visible.
    pub visible: bool,
    /// The target entity for the context menu.
    pub target: Option<ContextMenuTarget>,
    /// Screen position where menu should appear.
    pub screen_position: Vec2,
}

/// Target of the context menu.
#[derive(Clone, Debug)]
pub struct ContextMenuTarget {
    pub entity_id: EntityId,
    pub is_player: bool,
    pub target_type: TargetType,
}

/// Type of entity being targeted.
#[derive(Clone, Debug, PartialEq)]
pub enum TargetType {
    Actor,
    Item,
    Prop,
}

/// Available actions in the context menu.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MenuAction {
    Attack,
    Pickup,
}

impl MenuAction {
    fn label(&self) -> &'static str {
        match self {
            MenuAction::Attack => "Attack",
            MenuAction::Pickup => "Pick Up",
        }
    }

    fn color(&self) -> Color {
        match self {
            MenuAction::Attack => Color::srgb(1.0, 0.4, 0.4),
            MenuAction::Pickup => Color::srgb(0.4, 1.0, 0.4),
        }
    }
}

/// Marker for the context menu root.
#[derive(Component)]
pub struct ContextMenuRoot;

/// Marker for context menu buttons with their action.
#[derive(Component)]
pub struct ContextMenuButton(pub MenuAction);

/// Marker for the context menu title text.
#[derive(Component)]
pub struct ContextMenuTitle;

/// Spawn the context menu UI (initially hidden).
fn spawn_context_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                min_width: Val::Px(120.0),
                padding: UiRect::all(Val::Px(8.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.95)),
            BorderColor(Color::srgb(0.4, 0.4, 0.5)),
            BorderRadius::all(Val::Px(4.0)),
            Visibility::Hidden,
            ContextMenuRoot,
            // High z-index to appear above everything
            ZIndex(100),
        ))
        .with_children(|menu| {
            // Title
            menu.spawn((
                Text::new("Actions"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                ContextMenuTitle,
            ));

            // Separator
            menu.spawn(Node {
                height: Val::Px(1.0),
                width: Val::Percent(100.0),
                margin: UiRect::vertical(Val::Px(4.0)),
                ..default()
            })
            .insert(BackgroundColor(Color::srgb(0.3, 0.3, 0.4)));

            // Action buttons will be spawned dynamically
            for action in [MenuAction::Attack, MenuAction::Pickup] {
                spawn_menu_button(menu, action);
            }
        });
}

fn spawn_menu_button(parent: &mut ChildBuilder, action: MenuAction) {
    parent
        .spawn((
            Button,
            Node {
                padding: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(4.0), Val::Px(4.0)),
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            BackgroundColor(Color::srgba(0.2, 0.2, 0.25, 0.8)),
            BorderRadius::all(Val::Px(2.0)),
            ContextMenuButton(action),
            Visibility::Hidden, // Will be shown based on available actions
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(action.label()),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(action.color()),
            ));
        });
}

/// Handle right-click to open context menu.
fn handle_right_click(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    hover_state: Res<HoverState>,
    view_model: Option<Res<GameViewModel>>,
    mut menu_state: ResMut<ContextMenuState>,
) {
    // Close menu on left click anywhere
    if mouse.just_pressed(MouseButton::Left) && menu_state.visible {
        // Will be handled by close_menu_on_click_outside
        return;
    }

    if !mouse.just_pressed(MouseButton::Right) {
        return;
    }

    let Some(view_model) = view_model else {
        return;
    };

    let Ok(window) = windows.get_single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    // Check if we're hovering over something
    let Some(ref target) = hover_state.target else {
        // Close menu if right-clicking on nothing
        menu_state.visible = false;
        menu_state.target = None;
        return;
    };

    // Don't show menu for player (can't attack yourself)
    let (entity_id, is_player, target_type) = match target {
        HoverTarget::Actor { id, is_player } => (*id, *is_player, TargetType::Actor),
        HoverTarget::Item { id } => (*id, false, TargetType::Item),
        HoverTarget::Prop { id } => (*id, false, TargetType::Prop),
    };

    // Check if target is within attack range for actors (info message only)
    if target_type == TargetType::Actor
        && !is_player
        && view_model.0.player.position.is_some()
        && hover_state.position.is_some()
    {
        let player_pos = view_model.0.player.position.unwrap();
        let hover_pos = hover_state.position.unwrap();
        let distance = player_pos.chebyshev_distance(hover_pos);
        if distance > 1 {
            tracing::info!("Target out of attack range");
        }
    }

    menu_state.visible = true;
    menu_state.target = Some(ContextMenuTarget {
        entity_id,
        is_player,
        target_type,
    });
    menu_state.screen_position = cursor_pos;
}

/// Update context menu position and visibility.
fn update_context_menu_position(
    menu_state: Res<ContextMenuState>,
    view_model: Option<Res<GameViewModel>>,
    mut menu_root: Query<(&mut Node, &mut Visibility), With<ContextMenuRoot>>,
    mut menu_buttons: Query<(&ContextMenuButton, &mut Visibility), Without<ContextMenuRoot>>,
) {
    let Ok((mut node, mut visibility)) = menu_root.get_single_mut() else {
        return;
    };

    if !menu_state.visible {
        *visibility = Visibility::Hidden;
        // Also hide all buttons explicitly
        for (_, mut btn_visibility) in menu_buttons.iter_mut() {
            *btn_visibility = Visibility::Hidden;
        }
        return;
    }

    *visibility = Visibility::Visible;
    node.left = Val::Px(menu_state.screen_position.x);
    node.top = Val::Px(menu_state.screen_position.y);

    // Update button visibility based on target type and game state
    let Some(ref target) = menu_state.target else {
        return;
    };

    let view_model = view_model.as_ref();
    let is_player_turn = view_model
        .map(|vm| vm.0.turn.current_actor == EntityId::PLAYER)
        .unwrap_or(false);

    // Get player position for range checks
    let player_pos = view_model.and_then(|vm| vm.0.player.position);

    // Check attack range (melee = 1)
    let in_attack_range = if let (Some(vm), Some(player_pos)) = (view_model, player_pos) {
        vm.0.actors
            .iter()
            .find(|a| a.id == target.entity_id)
            .and_then(|a| a.position)
            .map(|pos| player_pos.chebyshev_distance(pos) <= 1)
            .unwrap_or(false)
    } else {
        false
    };

    // Check pickup range (must be standing on item, range = 0)
    let in_pickup_range = if let (Some(vm), Some(player_pos)) = (view_model, player_pos) {
        vm.0.items
            .iter()
            .find(|i| i.id == target.entity_id)
            .map(|i| i.position == player_pos)
            .unwrap_or(false)
    } else {
        false
    };

    for (button, mut btn_visibility) in menu_buttons.iter_mut() {
        let should_show = match button.0 {
            MenuAction::Attack => {
                target.target_type == TargetType::Actor
                    && !target.is_player
                    && is_player_turn
                    && in_attack_range
            }
            MenuAction::Pickup => {
                target.target_type == TargetType::Item && is_player_turn && in_pickup_range
            }
        };

        *btn_visibility = if should_show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

/// Handle clicks on menu buttons.
fn handle_menu_buttons(
    mut interaction_query: Query<
        (&Interaction, &ContextMenuButton, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut menu_state: ResMut<ContextMenuState>,
    action_sender: Option<Res<ActionSender>>,
) {
    let Some(action_sender) = action_sender else {
        return;
    };

    for (interaction, button, mut bg_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                let Some(ref target) = menu_state.target.clone() else {
                    continue;
                };

                // Execute the action based on button type
                let action = match button.0 {
                    MenuAction::Attack => {
                        if target.target_type == TargetType::Actor && !target.is_player {
                            Some(Action::Character(CharacterAction::new(
                                EntityId::PLAYER,
                                ActionKind::MeleeAttack,
                                ActionInput::Target(target.entity_id),
                            )))
                        } else {
                            None
                        }
                    }
                    MenuAction::Pickup => {
                        if target.target_type == TargetType::Item {
                            Some(Action::Character(CharacterAction::new(
                                EntityId::PLAYER,
                                ActionKind::PickupItem,
                                ActionInput::Target(target.entity_id),
                            )))
                        } else {
                            None
                        }
                    }
                };

                if let Some(action) = action {
                    if let Err(e) = action_sender.0.try_send(action) {
                        tracing::warn!("Failed to send action: {}", e);
                    }
                }

                // Always close menu after clicking a button
                menu_state.visible = false;
                menu_state.target = None;
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgba(0.3, 0.3, 0.35, 0.9));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgba(0.2, 0.2, 0.25, 0.8));
            }
        }
    }
}

/// Close menu when clicking outside.
fn close_menu_on_click_outside(
    mouse: Res<ButtonInput<MouseButton>>,
    mut menu_state: ResMut<ContextMenuState>,
    interaction_query: Query<&Interaction, With<ContextMenuButton>>,
) {
    if !mouse.just_pressed(MouseButton::Left) || !menu_state.visible {
        return;
    }

    // Check if any button is being interacted with
    let clicking_button = interaction_query
        .iter()
        .any(|i| *i == Interaction::Pressed || *i == Interaction::Hovered);

    if !clicking_button {
        menu_state.visible = false;
        menu_state.target = None;
    }
}
