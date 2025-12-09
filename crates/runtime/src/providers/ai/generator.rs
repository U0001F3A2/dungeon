//! Generates all possible action candidates from available actions.

use game_core::{ActionInput, ActionKind, CardinalDirection, EntityId, Position};
use tracing::debug;

use super::AiContext;

/// Generates all possible concrete action candidates.
///
/// For each available ActionKind, this generates all valid ActionInput combinations
/// based on the action's TargetingMode.
pub struct ActionCandidateGenerator;

impl ActionCandidateGenerator {
    /// Generates all possible action candidates.
    ///
    /// # Returns
    ///
    /// Vector of (ActionKind, ActionInput) tuples representing all possible actions
    /// the entity can currently take.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // For available actions [MeleeAttack, Move, Wait]:
    /// [
    ///   (MeleeAttack, Entity(Player)),           // if SingleTarget mode
    ///   (Move, Direction(North)),
    ///   (Move, Direction(South)),
    ///   (Move, Direction(East)),
    ///   (Move, Direction(West)),
    ///   // ... 4 more directions
    ///   (Wait, None),
    /// ]
    /// ```
    pub fn generate(
        available_kinds: &[ActionKind],
        ctx: &AiContext,
    ) -> Vec<(ActionKind, ActionInput)> {
        let mut candidates = Vec::new();

        for &kind in available_kinds {
            // Get action profile to determine targeting mode
            let profile = match ctx.env.actions() {
                Ok(actions) => actions.action_profile(kind),
                Err(e) => {
                    tracing::warn!("Failed to get action profile for {:?}: {}", kind, e);
                    continue;
                }
            };

            match &profile.targeting {
                game_core::TargetingMode::None | game_core::TargetingMode::SelfOnly => {
                    // No target needed
                    candidates.push((kind, ActionInput::None));
                }

                game_core::TargetingMode::SingleTarget {
                    range,
                    requires_los,
                } => {
                    // Generate candidates for each possible target entity
                    let targets = Self::find_valid_targets(ctx.entity, *range, *requires_los, ctx);

                    for target in targets {
                        candidates.push((kind, ActionInput::Target(target)));
                    }

                    // If no valid targets found, still generate a candidate
                    // (validation will fail later, but we want to consider it)
                    if candidates.is_empty() {
                        tracing::debug!(
                            "No valid targets for {:?} (range={}, los={})",
                            kind,
                            range,
                            requires_los
                        );
                    }
                }

                game_core::TargetingMode::Directional { range, width } => {
                    // For movement actions, only generate valid (walkable) directions
                    let valid_dirs = if kind == ActionKind::Move {
                        Self::find_walkable_directions(ctx)
                    } else {
                        // For other directional actions (attacks), generate all directions
                        CardinalDirection::all().to_vec()
                    };

                    for dir in valid_dirs {
                        candidates.push((kind, ActionInput::Direction(dir)));
                    }

                    tracing::trace!(
                        "Generated {} directional candidates for {:?} (range={}, width={:?})",
                        candidates.len(),
                        kind,
                        range,
                        width
                    );
                }
            }
        }

        tracing::debug!(
            "Generated {} action candidates from {} available actions",
            candidates.len(),
            available_kinds.len()
        );

        candidates
    }

    /// Finds all walkable directions from the actor's current position.
    ///
    /// This checks the map to see which adjacent tiles are passable,
    /// preventing the AI from trying to move into walls.
    fn find_walkable_directions(ctx: &AiContext) -> Vec<CardinalDirection> {
        let actor_pos = match ctx.state.entities.actor(ctx.entity) {
            Some(a) => match a.position {
                Some(pos) => pos,
                None => return Vec::new(),
            },
            None => return Vec::new(),
        };

        let map = match ctx.env.map() {
            Ok(m) => m,
            Err(_) => return CardinalDirection::all().to_vec(),
        };

        let mut walkable = Vec::new();

        for dir in CardinalDirection::all() {
            let (dx, dy) = dir.offset();
            let new_x = actor_pos.x + dx;
            let new_y = actor_pos.y + dy;

            // Check bounds (map uses non-negative coordinates)
            if new_x < 0 || new_y < 0 {
                continue;
            }

            let target_pos = Position { x: new_x, y: new_y };

            // Check if tile is walkable
            if let Some(tile) = map.tile(target_pos) {
                if tile.terrain().is_passable() {
                    // Also check if not occupied by another actor
                    let occupied = ctx.state.entities.actors.iter().any(|a| {
                        a.id != ctx.entity && a.position == Some(target_pos)
                    });
                    if !occupied {
                        walkable.push(dir);
                    }
                }
            }
        }

        walkable
    }

    /// Finds all valid target entities within range.
    ///
    /// # Arguments
    ///
    /// * `actor` - The entity performing the action
    /// * `range` - Maximum range in tiles (Chebyshev distance)
    /// * `requires_los` - Whether line of sight is required
    /// * `ctx` - AI context
    ///
    /// # Returns
    ///
    /// Vector of valid target entity IDs.
    fn find_valid_targets(
        actor: EntityId,
        range: u32,
        requires_los: bool,
        ctx: &AiContext,
    ) -> Vec<EntityId> {
        let mut targets = Vec::new();

        let actor_pos = match ctx.state.entities.actor(actor) {
            Some(a) => match a.position {
                Some(pos) => pos,
                None => {
                    debug!("Actor {:?} has no position", actor);
                    return targets;
                }
            },
            None => {
                debug!("Actor {:?} not found in entities", actor);
                return targets;
            }
        };

        // Check player as potential target
        let player = ctx.state.entities.player();
        let Some(player_pos) = player.position else {
            return targets; // Player not on map
        };
        let dist = actor_pos.chebyshev_distance(player_pos);

        if dist <= range {
            // TODO: Add actual LOS check when MapOracle supports it
            if requires_los {
                tracing::trace!("LOS check not implemented yet, assuming LOS exists for player");
            }

            targets.push(EntityId::PLAYER);
            tracing::trace!("Player is valid target: distance={}, range={}", dist, range);
        }

        // TODO: Add other entities when entity iteration is available
        // - Allies (for healing, buffing)
        // - Other enemies
        // - Props (for interaction)

        targets
    }
}
