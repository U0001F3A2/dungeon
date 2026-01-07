//! UI panel systems for stats, message log, and hover inspection.

use bevy::prelude::*;
use game_core::env::MapOracle;

use crate::components::{HealthText, HelpPanel, ManaText, MessageEntry, MessageLogPanel, StatsPanel, TurnText, UiRoot};
use crate::cursor::{HoverState, HoverTarget};
use crate::input::ShowHelp;
use crate::resources::{GameMessageLog, GameViewModel, OracleBundle};
use super::styles::*;

/// Marker component for examine panel (shown on mouse hover).
#[derive(Component)]
pub struct ExaminePanel;

/// Marker component for examine panel text lines.
#[derive(Component)]
pub struct ExamineText {
    pub line: usize,
}

/// Setup the main UI layout.
pub fn setup_ui(mut commands: Commands) {
    // Root UI container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            UiRoot,
        ))
        .with_children(|parent| {
            // Left panel: Stats
            spawn_stats_panel(parent);

            // Center: Input mode indicator at top
            spawn_center_ui(parent);

            // Right column: Message log and Examine panel
            spawn_right_column(parent);
        });

    // Help panel overlay (hidden by default)
    spawn_help_panel(&mut commands);
}

fn spawn_stats_panel(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Node {
                width: Val::Px(200.0),
                height: Val::Auto,
                padding: UiRect::all(Val::Px(10.0)),
                margin: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                align_self: AlignSelf::FlexStart,
                ..default()
            },
            BackgroundColor(PANEL_BG),
            BorderColor(PANEL_BORDER),
            StatsPanel,
        ))
        .with_children(|panel| {
            // Title
            panel.spawn((
                Text::new("Player Stats"),
                text_style(HEADER_FONT_SIZE, TEXT_COLOR).0,
                text_style(HEADER_FONT_SIZE, TEXT_COLOR).1,
            ));

            // Health
            panel.spawn((
                Text::new("HP: --/--"),
                text_style(TEXT_FONT_SIZE, HEALTH_COLOR).0,
                text_style(TEXT_FONT_SIZE, HEALTH_COLOR).1,
                HealthText,
            ));

            // Mana
            panel.spawn((
                Text::new("MP: --/--"),
                text_style(TEXT_FONT_SIZE, MANA_COLOR).0,
                text_style(TEXT_FONT_SIZE, MANA_COLOR).1,
                ManaText,
            ));

            // Separator
            panel.spawn(Node {
                height: Val::Px(1.0),
                width: Val::Percent(100.0),
                margin: UiRect::vertical(Val::Px(5.0)),
                ..default()
            });

            // Turn info
            panel.spawn((
                Text::new("Turn: --"),
                text_style(TEXT_FONT_SIZE, TURN_COLOR).0,
                text_style(TEXT_FONT_SIZE, TURN_COLOR).1,
                TurnText,
            ));

            // Controls hint
            panel.spawn(Node {
                height: Val::Px(1.0),
                width: Val::Percent(100.0),
                margin: UiRect::vertical(Val::Px(5.0)),
                ..default()
            });

            panel.spawn((
                Text::new("H - Help"),
                text_style(SMALL_FONT_SIZE, TEXT_COLOR).0,
                text_style(SMALL_FONT_SIZE, TEXT_COLOR).1,
            ));
        });
}

fn spawn_center_ui(parent: &mut ChildBuilder) {
    // Center spacer to push right panel to the edge
    parent.spawn(Node {
        flex_grow: 1.0,
        ..default()
    });
}

fn spawn_right_column(parent: &mut ChildBuilder) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::SpaceBetween,
            height: Val::Percent(100.0),
            ..default()
        })
        .with_children(|column| {
            // Top: Examine panel (shown on mouse hover)
            spawn_examine_panel(column);

            // Bottom: Message log
            spawn_message_log_panel(column);
        });
}

fn spawn_examine_panel(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Node {
                width: Val::Px(250.0),
                height: Val::Auto,
                padding: UiRect::all(Val::Px(10.0)),
                margin: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.9)),
            BorderColor(Color::srgb(0.3, 0.5, 0.7)),
            Visibility::Hidden,
            ExaminePanel,
        ))
        .with_children(|panel| {
            // Title
            panel.spawn((
                Text::new("Inspect"),
                text_style(HEADER_FONT_SIZE, Color::srgb(0.5, 0.8, 1.0)).0,
                text_style(HEADER_FONT_SIZE, Color::srgb(0.5, 0.8, 1.0)).1,
            ));

            // 6 lines for examine info
            for i in 0..6 {
                panel.spawn((
                    Text::new(""),
                    text_style(SMALL_FONT_SIZE, TEXT_COLOR).0,
                    text_style(SMALL_FONT_SIZE, TEXT_COLOR).1,
                    ExamineText { line: i },
                ));
            }
        });
}

fn spawn_message_log_panel(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Px(200.0),
                padding: UiRect::all(Val::Px(10.0)),
                margin: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(PANEL_BG),
            BorderColor(PANEL_BORDER),
            MessageLogPanel,
        ))
        .with_children(|panel| {
            // Title
            panel.spawn((
                Text::new("Messages"),
                text_style(HEADER_FONT_SIZE, TEXT_COLOR).0,
                text_style(HEADER_FONT_SIZE, TEXT_COLOR).1,
            ));

            // Message entries will be spawned dynamically
            for i in 0..8 {
                panel.spawn((
                    Text::new(""),
                    text_style(SMALL_FONT_SIZE, TEXT_COLOR).0,
                    text_style(SMALL_FONT_SIZE, TEXT_COLOR).1,
                    MessageEntry { index: i },
                ));
            }
        });
}

fn spawn_help_panel(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                width: Val::Px(300.0),
                padding: UiRect::all(Val::Px(20.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.95)),
            BorderColor(Color::srgb(0.4, 0.4, 0.5)),
            Visibility::Hidden,
            HelpPanel,
        ))
        .with_children(|panel| {
            // Title
            panel.spawn((
                Text::new("Controls"),
                text_style(HEADER_FONT_SIZE, Color::srgb(1.0, 0.9, 0.3)).0,
                text_style(HEADER_FONT_SIZE, Color::srgb(1.0, 0.9, 0.3)).1,
            ));

            // Movement section
            panel.spawn((
                Text::new("Movement"),
                text_style(TEXT_FONT_SIZE, Color::srgb(0.7, 0.9, 1.0)).0,
                text_style(TEXT_FONT_SIZE, Color::srgb(0.7, 0.9, 1.0)).1,
            ));
            panel.spawn((
                Text::new("  Arrow Keys - Move"),
                text_style(SMALL_FONT_SIZE, TEXT_COLOR).0,
                text_style(SMALL_FONT_SIZE, TEXT_COLOR).1,
            ));

            // Actions section
            panel.spawn((
                Text::new("Actions"),
                text_style(TEXT_FONT_SIZE, Color::srgb(0.7, 0.9, 1.0)).0,
                text_style(TEXT_FONT_SIZE, Color::srgb(0.7, 0.9, 1.0)).1,
            ));
            panel.spawn((
                Text::new("  G - Pickup item"),
                text_style(SMALL_FONT_SIZE, TEXT_COLOR).0,
                text_style(SMALL_FONT_SIZE, TEXT_COLOR).1,
            ));
            panel.spawn((
                Text::new("  Space/. - Wait"),
                text_style(SMALL_FONT_SIZE, TEXT_COLOR).0,
                text_style(SMALL_FONT_SIZE, TEXT_COLOR).1,
            ));

            // Mouse section
            panel.spawn((
                Text::new("Mouse"),
                text_style(TEXT_FONT_SIZE, Color::srgb(0.7, 0.9, 1.0)).0,
                text_style(TEXT_FONT_SIZE, Color::srgb(0.7, 0.9, 1.0)).1,
            ));
            panel.spawn((
                Text::new("  Hover - Inspect tile"),
                text_style(SMALL_FONT_SIZE, TEXT_COLOR).0,
                text_style(SMALL_FONT_SIZE, TEXT_COLOR).1,
            ));
            panel.spawn((
                Text::new("  Right-click - Actions menu"),
                text_style(SMALL_FONT_SIZE, TEXT_COLOR).0,
                text_style(SMALL_FONT_SIZE, TEXT_COLOR).1,
            ));

            // Other section
            panel.spawn((
                Text::new("Other"),
                text_style(TEXT_FONT_SIZE, Color::srgb(0.7, 0.9, 1.0)).0,
                text_style(TEXT_FONT_SIZE, Color::srgb(0.7, 0.9, 1.0)).1,
            ));
            panel.spawn((
                Text::new("  H/F1 - Toggle help"),
                text_style(SMALL_FONT_SIZE, TEXT_COLOR).0,
                text_style(SMALL_FONT_SIZE, TEXT_COLOR).1,
            ));
            panel.spawn((
                Text::new("  Esc - Cancel"),
                text_style(SMALL_FONT_SIZE, TEXT_COLOR).0,
                text_style(SMALL_FONT_SIZE, TEXT_COLOR).1,
            ));

            // Close hint
            panel.spawn(Node {
                height: Val::Px(8.0),
                ..default()
            });
            panel.spawn((
                Text::new("Press H to close"),
                text_style(SMALL_FONT_SIZE, Color::srgb(0.5, 0.5, 0.5)).0,
                text_style(SMALL_FONT_SIZE, Color::srgb(0.5, 0.5, 0.5)).1,
            ));
        });
}

/// Update the stats panel with current player info.
#[allow(clippy::type_complexity)]
pub fn update_stats_panel(
    view_model: Option<Res<GameViewModel>>,
    mut health_text: Query<&mut Text, (With<HealthText>, Without<ManaText>, Without<TurnText>)>,
    mut mana_text: Query<&mut Text, (With<ManaText>, Without<HealthText>, Without<TurnText>)>,
    mut turn_text: Query<&mut Text, (With<TurnText>, Without<HealthText>, Without<ManaText>)>,
) {
    let Some(view_model) = view_model else {
        return;
    };

    let player = &view_model.0.player;
    let (hp_current, hp_max) = player.stats.hp();
    let (mp_current, mp_max) = player.stats.mp();

    if let Ok(mut text) = health_text.get_single_mut() {
        **text = format!("HP: {}/{}", hp_current, hp_max);
    }

    if let Ok(mut text) = mana_text.get_single_mut() {
        **text = format!("MP: {}/{}", mp_current, mp_max);
    }

    if let Ok(mut text) = turn_text.get_single_mut() {
        **text = format!("Turn: {}", view_model.0.turn.clock);
    }
}

/// Update the message log with recent messages.
pub fn update_message_log(
    message_log: Option<Res<GameMessageLog>>,
    mut message_entries: Query<(&MessageEntry, &mut Text)>,
) {
    let Some(message_log) = message_log else {
        return;
    };

    let messages: Vec<_> = message_log.0.iter().take(8).collect();

    for (entry, mut text) in message_entries.iter_mut() {
        if let Some(msg) = messages.get(entry.index) {
            **text = msg.text.clone();
        } else {
            **text = String::new();
        }
    }
}

/// Toggle help panel visibility.
pub fn update_help_visibility(
    show_help: Res<ShowHelp>,
    mut help_panel: Query<&mut Visibility, With<HelpPanel>>,
) {
    if let Ok(mut visibility) = help_panel.get_single_mut() {
        *visibility = if show_help.0 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

/// Update examine panel based on mouse hover.
pub fn update_examine_panel(
    hover_state: Res<HoverState>,
    view_model: Option<Res<GameViewModel>>,
    oracle_bundle: Option<Res<OracleBundle>>,
    mut examine_panel: Query<&mut Visibility, With<ExaminePanel>>,
    mut examine_text: Query<(&ExamineText, &mut Text)>,
) {
    let Some(hover_pos) = hover_state.position else {
        // Hide panel when not hovering over map
        if let Ok(mut visibility) = examine_panel.get_single_mut() {
            *visibility = Visibility::Hidden;
        }
        return;
    };

    // Show panel when hovering
    if let Ok(mut visibility) = examine_panel.get_single_mut() {
        *visibility = Visibility::Visible;
    }

    let Some(view_model) = view_model else {
        return;
    };

    let Some(oracle_bundle) = oracle_bundle else {
        return;
    };

    // Get tile info at hover position
    let map_oracle = oracle_bundle.0.map.as_ref();
    let tile_info = map_oracle.tile(hover_pos);
    let terrain_name = tile_info
        .map(|t| format!("{:?}", t.terrain()))
        .unwrap_or_else(|| "Void".to_string());

    // Build display lines
    let mut lines: Vec<String> = vec![
        format!("({}, {}) - {}", hover_pos.x, hover_pos.y, terrain_name),
    ];

    // Add entity info if hovering over something
    match &hover_state.target {
        Some(HoverTarget::Actor { id, is_player }) => {
            // Find the actor in view model
            if let Some(actor) = view_model.0.actors.iter().find(|a| a.id == *id) {
                let (hp_cur, hp_max) = actor.stats.hp();
                let entity_type = if *is_player { "Player" } else { "Enemy" };
                lines.push(entity_type.to_string());
                lines.push(format!("HP: {}/{}", hp_cur, hp_max));
                lines.push(format!("Speed: {}", actor.stats.speed.physical));
            }
        }
        Some(HoverTarget::Item { id }) => {
            if let Some(item) = view_model.0.items.iter().find(|i| i.id == *id) {
                lines.push("Item".to_string());
                lines.push(format!("Handle: {}", item.handle.0));
            }
        }
        Some(HoverTarget::Prop { id }) => {
            if let Some(prop) = view_model.0.props.iter().find(|p| p.id == *id) {
                lines.push("Prop".to_string());
                lines.push(format!("Kind: {:?}", prop.kind));
                lines.push(format!("Active: {}", if prop.is_active { "Yes" } else { "No" }));
            }
        }
        None => {
            // Just show terrain info
        }
    }

    // Update text elements
    for (examine, mut text) in examine_text.iter_mut() {
        if let Some(line_text) = lines.get(examine.line) {
            **text = line_text.clone();
        } else {
            **text = String::new();
        }
    }
}
