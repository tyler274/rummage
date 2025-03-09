use crate::card::{Card, CardTypes};
use crate::game_engine::GameState;
use crate::game_engine::Phase;
use crate::game_engine::zones::{Zone, ZoneChangeEvent, ZoneManager};
use crate::mana::Color;
use crate::mana::Mana;
use crate::player::Player;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

/// Component that marks a card as a Commander
#[derive(Component, Debug, Clone)]
pub struct Commander {
    /// The original owner of this commander
    pub owner: Entity,

    /// How many times this commander has been cast from the command zone
    pub cast_count: u32,

    /// Tracks commander damage dealt to each player
    pub damage_dealt: Vec<(Entity, u32)>,

    /// Commander's color identity (for deck validation)
    pub color_identity: HashSet<Color>,

    /// Commander-specific flags
    pub is_partner: bool,
    pub is_background: bool,

    /// Track if commander has dealt combat damage this turn
    pub dealt_combat_damage_this_turn: HashSet<Entity>,
}

impl Default for Commander {
    fn default() -> Self {
        Self {
            owner: Entity::PLACEHOLDER,
            cast_count: 0,
            damage_dealt: Vec::new(),
            color_identity: HashSet::new(),
            is_partner: false,
            is_background: false,
            dealt_combat_damage_this_turn: HashSet::new(),
        }
    }
}

/// The Command Zone where commanders reside when not in play
#[derive(Resource, Debug, Default)]
pub struct CommandZone {
    /// Cards in the command zone
    pub cards: Vec<Entity>,
}

impl CommandZone {
    /// Add a card to the command zone
    pub fn add_card(&mut self, card: Entity) {
        if !self.cards.contains(&card) {
            self.cards.push(card);
        }
    }

    /// Remove a card from the command zone
    pub fn remove_card(&mut self, card: Entity) -> bool {
        if let Some(index) = self.cards.iter().position(|&c| c == card) {
            self.cards.remove(index);
            true
        } else {
            false
        }
    }

    /// Check if a card is in the command zone
    pub fn contains(&self, card: Entity) -> bool {
        self.cards.contains(&card)
    }
}

/// Event that represents combat damage being dealt
#[derive(Event)]
pub struct CombatDamageEvent {
    /// The source of the damage (usually a creature)
    pub source: Entity,
    /// The target of the damage (player or creature)
    pub target: Entity,
    /// The amount of damage dealt
    pub damage: u32,
    /// Whether this is combat damage (vs. direct damage from spells/abilities)
    pub is_combat_damage: bool,
}

/// Event that triggers when a player needs to decide if their commander
/// should go to the command zone instead of another zone
#[derive(Event)]
pub struct CommanderZoneChoiceEvent {
    /// The commander card entity
    pub commander: Entity,
    /// The owner of the commander
    pub owner: Entity,
    /// The zone the commander is currently in
    pub current_zone: Zone,
    /// Whether the commander can go to the command zone
    pub can_go_to_command_zone: bool,
}

/// Reason why a player was eliminated from the game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EliminationReason {
    /// Player lost due to having 0 or less life
    LifeLoss,
    /// Player lost due to trying to draw from an empty library
    EmptyLibrary,
    /// Player lost due to receiving 21+ commander damage from a single commander
    CommanderDamage(Entity), // The commander that dealt the lethal damage
    /// Player conceded
    Concede,
    /// Player lost due to a specific card effect
    CardEffect(Entity), // The card that caused the elimination
}

/// Event that triggers when a player is eliminated from the game
#[derive(Event)]
pub struct PlayerEliminatedEvent {
    /// The player that was eliminated
    pub player: Entity,
    /// The reason the player was eliminated
    pub reason: EliminationReason,
}

/// Commander-specific game rules and constants
pub struct CommanderRules;

impl CommanderRules {
    /// The amount of Commander damage needed to eliminate a player
    pub const COMMANDER_DAMAGE_THRESHOLD: u32 = 21;

    /// The number of players in a standard Commander game
    pub const STANDARD_PLAYER_COUNT: usize = 4;

    /// The starting life total in Commander
    pub const STARTING_LIFE: i32 = 40;

    /// Calculate the Commander tax (additional mana cost) for casting a Commander
    pub fn calculate_tax(cast_count: u32) -> u64 {
        // Convert to u64 to match the Mana.colorless type
        2u64 * cast_count as u64
    }

    /// Check if a player has been eliminated by Commander damage
    pub fn check_commander_damage_elimination(commander: &Commander, player: Entity) -> bool {
        commander
            .damage_dealt
            .iter()
            .any(|(p, damage)| *p == player && *damage >= Self::COMMANDER_DAMAGE_THRESHOLD)
    }

    /// Check if a card can be a Commander
    pub fn can_be_commander(card: &Card) -> bool {
        // Legendary creatures can be commanders
        if card.types.contains(CardTypes::LEGENDARY) && card.types.contains(CardTypes::CREATURE) {
            return true;
        }

        // Cards with "can be your commander" text can also be commanders
        // For simplicity, we're just checking if the text contains the phrase
        card.rules_text
            .to_lowercase()
            .contains("can be your commander")
    }

    /// Extract the color identity of a card
    pub fn extract_color_identity(card: &Card) -> HashSet<Color> {
        let mut colors = HashSet::new();

        // Add colors from mana cost
        if card.cost.white > 0 {
            colors.insert(Color::WHITE);
        }
        if card.cost.blue > 0 {
            colors.insert(Color::BLUE);
        }
        if card.cost.black > 0 {
            colors.insert(Color::BLACK);
        }
        if card.cost.red > 0 {
            colors.insert(Color::RED);
        }
        if card.cost.green > 0 {
            colors.insert(Color::GREEN);
        }

        // In a full implementation, we would also:
        // - Check mana symbols in rules text
        // - Check color indicators
        // - Check for land types that implicitly add colors

        colors
    }
}

/// Initialize Commander-specific resources and components
pub fn setup_commander(mut commands: Commands) {
    commands.insert_resource(CommandZone::default());
}

/// Calculate the mana cost of a Commander including the Commander tax
pub fn calculate_commander_tax(commander: &Commander) -> Mana {
    // Create a new Mana cost based on the card's original cost plus tax
    let tax = CommanderRules::calculate_tax(commander.cast_count);

    // In a real implementation, we would get the card's cost and add tax
    // For now, we'll create a simple colorless mana cost with the tax
    let mut mana = Mana::default();
    mana.colorless = tax;
    mana
}

/// Check if any player has lost due to commander damage
pub fn check_commander_damage_loss(
    mut commands: Commands,
    commander_query: Query<&Commander>,
    player_query: Query<(Entity, &Player)>,
) {
    for (player_entity, _player) in player_query.iter() {
        // Check each commander for damage dealt to this player
        for commander in commander_query.iter() {
            if let Some(damage) = commander
                .damage_dealt
                .iter()
                .find(|(p, _)| p == &player_entity)
            {
                if damage.1 >= 21 {
                    // Player has lost due to commander damage
                    commands.spawn(PlayerEliminatedEvent {
                        player: player_entity,
                        reason: EliminationReason::CommanderDamage(commander.owner),
                    });
                    break;
                }
            }
        }
    }
}

/// Record commander damage from combat
pub fn record_commander_damage(
    mut commander_query: Query<&mut Commander>,
    mut damage_events: EventReader<CombatDamageEvent>,
) {
    for event in damage_events.read() {
        // Only process commander combat damage
        if let Ok(mut commander) = commander_query.get_mut(event.source) {
            if event.is_combat_damage && event.damage > 0 {
                // Update the commander's damage tracking
                if let Some(damage_entry) = commander
                    .damage_dealt
                    .iter_mut()
                    .find(|(p, _)| *p == event.target)
                {
                    // Update existing damage entry
                    damage_entry.1 += event.damage;
                } else {
                    // Add a new damage entry
                    commander.damage_dealt.push((event.target, event.damage));
                }

                // Record that the commander dealt damage to this player this turn
                commander.dealt_combat_damage_this_turn.insert(event.target);
            }
        }
    }
}

/// Handle commander changing zones
pub fn handle_commander_zone_change(
    mut commands: Commands,
    mut zone_manager: ResMut<ZoneManager>,
    mut zone_events: EventReader<ZoneChangeEvent>,
    commander_query: Query<(Entity, &Commander)>,
) {
    for event in zone_events.read() {
        // Check if the card is a commander
        if let Ok((entity, commander)) = commander_query.get(event.card) {
            // Special handling for commander death/exile
            if (event.destination == Zone::Graveyard || event.destination == Zone::Exile)
                && (event.source == Zone::Battlefield || event.source == Zone::Stack)
            {
                // Spawn a choice event for the player
                commands.spawn(CommanderZoneChoiceEvent {
                    commander: entity,
                    owner: commander.owner,
                    current_zone: event.destination,
                    can_go_to_command_zone: true,
                });
            }
        }
    }
}

/// Process commander zone choice events
pub fn process_commander_zone_choices(
    mut commands: Commands,
    mut choice_events: EventReader<CommanderZoneChoiceEvent>,
    mut zone_manager: ResMut<ZoneManager>,
    mut commander_query: Query<&mut Commander>,
) {
    for event in choice_events.read() {
        if event.can_go_to_command_zone {
            // Move the commander to the command zone
            zone_manager.move_card(
                event.commander,
                event.owner,
                event.current_zone,
                Zone::CommandZone,
            );

            // Notify that the commander moved to the command zone
            info!("Commander moved to command zone");
        }
    }
}

/// Handle casting a commander from the command zone
pub fn handle_commander_casting(
    mut commands: Commands,
    mut zone_manager: ResMut<ZoneManager>,
    mut commander_query: Query<&mut Commander>,
    // We would need other queries and inputs here
) {
    // In a full implementation, this would:
    // 1. Apply commander tax based on cast count
    // 2. Move the commander from command zone to stack
    // 3. Increment cast count
}

/// Check for violation of color identity in a commander deck
pub fn validate_commander_deck(
    card_query: Query<&Card>,
    commander_query: Query<(Entity, &Commander, &Card)>,
) -> bool {
    // For each player's deck, ensure all cards match their commander's color identity
    // This is a placeholder implementation
    true
}

/// System to handle Commander damage tracking
pub fn track_commander_damage(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    commanders: Query<(Entity, &Commander, &crate::card::CreatureOnField)>,
    players: Query<Entity, With<Player>>,
    // We'll need a damage event/component to track actual damage
) {
    // This would be implemented to:
    // 1. Detect when a Commander deals combat damage to a player
    // 2. Track the damage in the Commander component
    // 3. Check if any player has been eliminated by Commander damage
}

/// Register all Commander-related systems and events
pub fn register_commander_systems(app: &mut App) {
    app.add_event::<CommanderZoneChoiceEvent>()
        .add_event::<PlayerEliminatedEvent>()
        .add_event::<CombatDamageEvent>()
        .add_systems(
            Update,
            (
                track_commander_damage,
                handle_commander_zone_change,
                process_commander_zone_choices,
                check_commander_damage_loss,
                record_commander_damage,
            ),
        );
}
