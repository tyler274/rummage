use crate::card::{Card, CardTypes};
use crate::game_engine::Phase;
use crate::game_engine::zones::{Zone, ZoneChangeEvent, ZoneManager};
use crate::mana::Color;
use crate::mana::Mana;
use crate::menu::GameMenuState;
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

/// Enum indicating where a commander is currently located
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommanderZoneLocation {
    CommandZone,
    Battlefield,
    Graveyard,
    Exile,
    Hand,
    Library,
    Stack,
}

/// Manager for the Command Zone and Commander state
#[derive(Resource, Debug, Default)]
pub struct CommandZoneManager {
    /// Maps player entities to their commander entities
    pub player_commanders: HashMap<Entity, Vec<Entity>>,

    /// Maps commander entities to their current zone
    pub commander_zone_status: HashMap<Entity, CommanderZoneLocation>,

    /// Tracks how many times a commander has moved zones
    pub zone_transition_count: HashMap<Entity, u32>,

    /// Tracks commander partnerships
    pub commander_partners: HashMap<Entity, Entity>,

    /// Maps commander entities to their color identity
    pub commander_colors: HashMap<Entity, HashSet<Color>>,
}

impl CommandZoneManager {
    /// Initialize with a list of players and their commanders
    pub fn initialize(&mut self, player_commanders: HashMap<Entity, Vec<Entity>>) {
        self.player_commanders = player_commanders.clone();

        // Initialize all commanders as being in the command zone
        for commanders in player_commanders.values() {
            for &commander in commanders {
                self.commander_zone_status
                    .insert(commander, CommanderZoneLocation::CommandZone);
                self.zone_transition_count.insert(commander, 0);
            }
        }
    }

    /// Set a commander's color identity
    pub fn set_commander_color_identity(
        &mut self,
        commander: Entity,
        color_identity: HashSet<Color>,
    ) {
        self.commander_colors.insert(commander, color_identity);
    }

    /// Get a player's commanders
    pub fn get_player_commanders(&self, player: Entity) -> Vec<Entity> {
        self.player_commanders
            .get(&player)
            .cloned()
            .unwrap_or_default()
    }

    /// Get a commander's current zone
    pub fn get_commander_zone(&self, commander: Entity) -> CommanderZoneLocation {
        self.commander_zone_status
            .get(&commander)
            .cloned()
            .unwrap_or(CommanderZoneLocation::CommandZone)
    }

    /// Get the number of times a commander has been cast
    pub fn get_cast_count(&self, commander: Entity) -> u32 {
        self.zone_transition_count
            .get(&commander)
            .cloned()
            .unwrap_or(0)
    }

    /// Update a commander's zone
    pub fn update_commander_zone(&mut self, commander: Entity, new_zone: CommanderZoneLocation) {
        self.commander_zone_status.insert(commander, new_zone);
    }
}

/// Event that represents combat damage being dealt
#[derive(Event, Clone)]
pub struct CombatDamageEvent {
    /// The source of the damage (usually a creature)
    pub source: Entity,
    /// The target of the damage (player or creature)
    pub target: Entity,
    /// The amount of damage dealt
    pub damage: u32,
    /// Whether this is combat damage (vs. direct damage from spells/abilities)
    pub is_combat_damage: bool,
    /// Whether the source is a commander (for commander damage tracking)
    pub source_is_commander: bool,
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
    commands.insert_resource(CommandZoneManager::default());
}

/// Calculate the mana cost of a Commander including the Commander tax
pub fn calculate_commander_cost(
    commander: Entity,
    base_cost: Mana,
    cmd_zone_manager: &CommandZoneManager,
) -> Mana {
    let mut final_cost = base_cost.clone();

    // Get the commander's cast count and add tax
    let cast_count = cmd_zone_manager.get_cast_count(commander);
    final_cost.colorless += CommanderRules::calculate_tax(cast_count);

    final_cost
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
                if damage.1 >= CommanderRules::COMMANDER_DAMAGE_THRESHOLD {
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
        if !event.source_is_commander || !event.is_combat_damage || event.damage == 0 {
            continue;
        }

        if let Ok(mut commander) = commander_query.get_mut(event.source) {
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

/// Handle commander changing zones
pub fn handle_commander_zone_change(
    mut commands: Commands,
    mut zone_manager: ResMut<ZoneManager>,
    mut cmd_zone_manager: ResMut<CommandZoneManager>,
    mut zone_events: EventReader<ZoneChangeEvent>,
    commander_query: Query<(Entity, &Commander)>,
) {
    for event in zone_events.read() {
        // Check if the card is a commander
        if let Ok((entity, commander)) = commander_query.get(event.card) {
            // Update the commander's zone status
            let new_zone = match event.destination {
                Zone::CommandZone => CommanderZoneLocation::CommandZone,
                Zone::Battlefield => CommanderZoneLocation::Battlefield,
                Zone::Graveyard => CommanderZoneLocation::Graveyard,
                Zone::Exile => CommanderZoneLocation::Exile,
                Zone::Hand => CommanderZoneLocation::Hand,
                Zone::Library => CommanderZoneLocation::Library,
                Zone::Stack => CommanderZoneLocation::Stack,
                _ => CommanderZoneLocation::CommandZone, // Default case
            };

            cmd_zone_manager.update_commander_zone(entity, new_zone);

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

            // Increase zone transition count if moved to command zone
            if event.destination == Zone::CommandZone {
                let count = cmd_zone_manager
                    .zone_transition_count
                    .entry(entity)
                    .or_insert(0);
                *count += 1;
            }
        }
    }
}

/// Process commander zone choice events
pub fn process_commander_zone_choices(
    mut commands: Commands,
    mut choice_events: EventReader<CommanderZoneChoiceEvent>,
    mut zone_manager: ResMut<ZoneManager>,
    mut cmd_zone_manager: ResMut<CommandZoneManager>,
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

            // Update the commander zone status
            cmd_zone_manager
                .update_commander_zone(event.commander, CommanderZoneLocation::CommandZone);

            // Increment zone transition count
            let count = cmd_zone_manager
                .zone_transition_count
                .entry(event.commander)
                .or_insert(0);
            *count += 1;

            // Notify that the commander moved to the command zone
            info!("Commander moved to command zone");
        }
    }
}

/// Handle casting a commander from the command zone
pub fn handle_commander_casting(
    mut commands: Commands,
    mut zone_manager: ResMut<ZoneManager>,
    mut cmd_zone_manager: ResMut<CommandZoneManager>,
    mut commander_query: Query<&mut Commander>,
    cards: Query<(Entity, &Card)>,
    // We would need other queries and inputs here
) {
    // In a full implementation, this would:
    // 1. Apply commander tax based on cast count
    // 2. Move the commander from command zone to stack
    // 3. Update the commander zone status
    // 4. Increment cast count if successfully cast
}

/// Check for violation of color identity in a commander deck
pub fn validate_commander_deck(
    card_query: Query<(Entity, &Card)>,
    cmd_zone_manager: Res<CommandZoneManager>,
    player_query: Query<(Entity, &Player)>,
) -> HashMap<Entity, Vec<Entity>> {
    // Map to store players and their illegal cards
    let mut illegal_cards = HashMap::new();

    // For each player, check their deck against their commander's color identity
    for (player_entity, _) in player_query.iter() {
        let commanders = cmd_zone_manager.get_player_commanders(player_entity);
        if commanders.is_empty() {
            continue;
        }

        // Get combined color identity of all commanders
        let mut combined_identity = HashSet::new();
        for &commander in &commanders {
            if let Some(colors) = cmd_zone_manager.commander_colors.get(&commander) {
                combined_identity.extend(colors.iter().cloned());
            }
        }

        // TODO: This would need to check all cards in a player's deck
        // For now, this is just a placeholder implementation
    }

    illegal_cards
}

/// System to handle Commander damage tracking
pub fn track_commander_damage(
    mut commands: Commands,
    mut game_state: ResMut<GameMenuState>,
    commanders: Query<(Entity, &Commander)>,
    players: Query<Entity, With<Player>>,
    cmd_zone_manager: Res<CommandZoneManager>,
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
            )
                .run_if(crate::game_engine::game_state_condition),
        );
}
