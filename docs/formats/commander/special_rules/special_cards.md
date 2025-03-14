# Commander-Specific Cards

This document details the implementation of cards designed specifically for the Commander format.

## Overview

Commander has become one of Magic: The Gathering's most popular formats, leading to the creation of cards specifically designed for it. These cards include:

1. Cards that explicitly reference the command zone or commanders
2. Cards designed to support multiplayer politics
3. Cards with mechanics only found in Commander products
4. Cards that interact with format-specific rules

## Card Categories

### Command Zone Interaction Cards

These cards directly interact with the command zone or commanders:

#### Command Zone Access

```rust
/// Component for effects that can access the command zone
#[derive(Component)]
pub struct CommandZoneAccessEffect {
    /// The type of interaction with the command zone
    pub interaction_type: CommandZoneInteraction,
}

/// Types of command zone interactions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandZoneInteraction {
    /// Cast a card from the command zone (e.g., Command Beacon)
    CastFromZone,
    /// Return a card to the command zone (e.g., Leadership Vacuum)
    ReturnToZone,
    /// Copy a commander (e.g., Sakashima of a Thousand Faces)
    CopyCommander,
    /// Modify commander properties (e.g., Nevermore naming a commander)
    ModifyCommander,
}
```

Example implementation of Command Beacon:

```rust
pub fn create_command_beacon() -> impl Bundle {
    (
        CardName("Command Beacon".to_string()),
        CardType::Land,
        EntersTapped(false),
        ActivatedAbility {
            id: AbilityId::new(),
            cost: AbilityCost::Sacrifice(SacrificeCost::Self_),
            effect: Box::new(CommandBeaconEffect),
            timing_restriction: TimingRestriction::Sorcery,
            zone_restriction: ZoneRestriction::OnBattlefield,
        },
    )
}

/// Implementation of Command Beacon's effect
#[derive(Debug, Clone)]
pub struct CommandBeaconEffect;

impl AbilityEffect for CommandBeaconEffect {
    fn resolve(&self, world: &mut World, ability_ctx: &mut AbilityContext) {
        let source = ability_ctx.source;
        let controller = ability_ctx.controller;
        
        // Find commander in command zone
        if let Some(commander) = find_commander_in_command_zone(world, controller) {
            // Move commander to hand
            world.send_event(ZoneTransitionEvent {
                entity: commander,
                from: Zone::Command,
                to: Zone::Hand,
                cause: TransitionCause::Ability {
                    source,
                    ability_name: "Command Beacon".to_string(),
                },
            });
        }
    }
}
```

#### Commander Cost Reduction

Cards that reduce or modify the cost of casting commanders:

```rust
/// Component for effects that modify commander costs
#[derive(Component)]
pub struct CommanderCostModifier {
    /// How the cost is modified
    pub modification: CostModification,
    /// Which commanders this applies to (all or specific)
    pub target: CommanderTarget,
}

/// Implementation of Emerald Medallion (reduces cost of green spells)
pub fn create_emerald_medallion() -> impl Bundle {
    (
        CardName("Emerald Medallion".to_string()),
        CardType::Artifact,
        StaticAbility {
            id: AbilityId::new(),
            effect: Box::new(ColorCostReductionEffect(Color::Green)),
            condition: StaticCondition::Always,
        },
    )
}
```

### Multiplayer Politics Cards

Cards designed for multiplayer political interactions:

```rust
/// Component for voting cards
#[derive(Component)]
pub struct VotingMechanic {
    /// The options players can vote for
    pub options: Vec<String>,
    /// What happens based on voting results
    pub resolution: VoteResolution,
}

/// Implementation of Council's Judgment
pub fn create_councils_judgment() -> impl Bundle {
    (
        CardName("Council's Judgment".to_string()),
        CardType::Sorcery,
        ManaCost::parse("{1}{W}{W}"),
        VotingMechanic {
            options: vec!["Exile".to_string()], // Each nonland permanent is an option
            resolution: VoteResolution::ExileMostVotes,
        },
    )
}

/// System that handles voting
pub fn handle_voting(
    mut commands: Commands,
    mut vote_events: EventReader<VoteEvent>,
    mut vote_results: EventWriter<VoteResultEvent>,
    voting_cards: Query<(Entity, &VotingMechanic, &Owner)>,
    players: Query<Entity, With<Player>>,
) {
    // Implementation details...
}
```

### Commander-Specific Mechanics

Several mechanics were introduced specifically for Commander products:

#### Lieutenant

Cards with abilities that trigger if you control your commander:

```rust
/// Component for Lieutenant abilities
#[derive(Component)]
pub struct Lieutenant {
    /// The effect that happens when you control your commander
    pub effect: Box<dyn LieutenantEffect>,
}

/// Implementation of Thunderfoot Baloth
pub fn create_thunderfoot_baloth() -> impl Bundle {
    (
        CardName("Thunderfoot Baloth".to_string()),
        CardType::Creature,
        CreatureType(vec!["Beast".to_string()]),
        Power(5),
        Toughness(5),
        Lieutenant {
            effect: Box::new(ThunderfootBoostEffect),
        },
    )
}

/// System that checks for Lieutenant conditions
pub fn check_lieutenant_condition(
    mut commands: Commands,
    lieutenants: Query<(Entity, &Lieutenant, &Controller)>,
    commanders: Query<(Entity, &Commander, &Controller)>,
    mut effect_events: EventWriter<LieutenantEffectEvent>,
) {
    // For each lieutenant...
    for (lieutenant_entity, lieutenant, controller) in lieutenants.iter() {
        // Check if controller controls their commander
        let controls_commander = commanders
            .iter()
            .any(|(_, _, cmd_controller)| cmd_controller.0 == controller.0);
        
        // Apply or remove effect based on condition
        effect_events.send(LieutenantEffectEvent {
            lieutenant: lieutenant_entity,
            active: controls_commander,
        });
    }
}
```

#### Join Forces

Cards that allow all players to contribute mana to a spell:

```rust
/// Component for Join Forces mechanic
#[derive(Component)]
pub struct JoinForces {
    /// What happens based on mana contributed
    pub effect: Box<dyn JoinForcesEffect>,
    /// Which players can contribute
    pub contributor_restriction: ContributorRestriction,
}

/// Implementation of Minds Aglow
pub fn create_minds_aglow() -> impl Bundle {
    (
        CardName("Minds Aglow".to_string()),
        CardType::Sorcery,
        ManaCost::parse("{U}"),
        JoinForces {
            effect: Box::new(MindsAglowDrawEffect),
            contributor_restriction: ContributorRestriction::All,
        },
    )
}
```

#### Monarch

The Monarch mechanic introduces a special designation that players can claim, providing card advantage:

```rust
/// Component marking a player as the Monarch
#[derive(Component)]
pub struct Monarch;

/// Resource tracking the current monarch
#[derive(Resource)]
pub struct MonarchState {
    /// The current monarch, if any
    pub current_monarch: Option<Entity>,
    /// When the monarchy was last changed
    pub last_changed: Option<f32>,
}

/// System that handles the Monarch end-of-turn trigger
pub fn monarch_end_step_trigger(
    time: Res<Time>,
    monarch_state: Res<MonarchState>,
    mut turn_events: EventReader<EndStepEvent>,
    mut card_draw_events: EventWriter<DrawCardEvent>,
) {
    for event in turn_events.read() {
        if let Some(monarch) = monarch_state.current_monarch {
            if event.player == monarch {
                // Monarch draws a card at end of their turn
                card_draw_events.send(DrawCardEvent {
                    player: monarch,
                    amount: 1,
                    source: None,
                    reason: DrawReason::Ability("Monarch".to_string()),
                });
            }
        }
    }
}

/// System that transfers the Monarch when combat damage is dealt to them
pub fn monarch_combat_damage_transfer(
    mut commands: Commands,
    mut monarch_state: ResMut<MonarchState>,
    mut combat_damage_events: EventReader<CombatDamageEvent>,
    players: Query<Entity, With<Player>>,
) {
    for event in combat_damage_events.read() {
        if let Some(current_monarch) = monarch_state.current_monarch {
            if event.defending_player == current_monarch {
                // Transfer monarchy to attacking player
                if let Some(monarch_component) = commands.get_entity(current_monarch) {
                    monarch_component.remove::<Monarch>();
                }
                
                if let Some(new_monarch) = commands.get_entity(event.attacking_player_controller) {
                    new_monarch.insert(Monarch);
                    monarch_state.current_monarch = Some(event.attacking_player_controller);
                    monarch_state.last_changed = Some(time.elapsed_seconds());
                }
            }
        }
    }
}
```

#### Myriad

Myriad creates token copies attacking each opponent:

```rust
/// Component for the Myriad ability
#[derive(Component)]
pub struct Myriad;

/// System that handles Myriad triggers
pub fn handle_myriad_attacks(
    mut commands: Commands,
    mut declare_attacker_events: EventReader<DeclareAttackerEvent>,
    myriad_creatures: Query<Entity, With<Myriad>>,
    players: Query<Entity, With<Player>>,
    controllers: Query<&Controller>,
) {
    for event in declare_attacker_events.read() {
        if myriad_creatures.contains(event.attacker) {
            let attacker_controller = controllers.get(event.attacker).map(|c| c.0).unwrap_or_default();
            
            // Create token copies attacking each other opponent
            for potential_defender in players.iter() {
                // Skip the attacking player and the already-targeted defender
                if potential_defender == attacker_controller || potential_defender == event.defender {
                    continue;
                }
                
                // Create a token copy attacking this opponent
                let token = commands.spawn((
                    // Copy relevant components from original
                    // Add attacking status to new opponent
                    AttackingStatus {
                        defending_player: potential_defender,
                    },
                    TokenCopy {
                        original: event.attacker,
                        // Myriad tokens are exiled at end of combat
                        exile_at_end_of_combat: true,
                    },
                )).id();
                
                // More token setup...
            }
        }
    }
}
```

#### Melee

Melee gives a bonus based on how many opponents were attacked:

```rust
/// Component for the Melee ability
#[derive(Component)]
pub struct Melee {
    /// Bonus per opponent attacked
    pub bonus: i32,
}

/// System that calculates Melee bonuses
pub fn calculate_melee_bonuses(
    mut commands: Commands,
    mut declare_attackers_step_events: EventReader<DeclareAttackersStepEvent>,
    melee_creatures: Query<(Entity, &Melee, &Controller)>,
    mut attacking_status: Query<&AttackingStatus>,
    players: Query<Entity, With<Player>>,
) {
    for event in declare_attackers_step_events.read() {
        // For each creature with Melee...
        for (melee_entity, melee, controller) in melee_creatures.iter() {
            // Count distinct opponents attacked
            let opponents_attacked = attacking_status
                .iter()
                .filter(|status| {
                    // Only count attacks from controller's creatures
                    if let Ok(attacker_controller) = controllers.get(status.attacker) {
                        attacker_controller.0 == controller.0 && status.defending_player != controller.0
                    } else {
                        false
                    }
                })
                .map(|status| status.defending_player)
                .collect::<HashSet<Entity>>()
                .len();
                
            // Apply Melee bonus based on opponents attacked
            commands.entity(melee_entity).insert(MeleeBoost {
                power_bonus: melee.bonus * opponents_attacked as i32,
                toughness_bonus: melee.bonus * opponents_attacked as i32,
                expires_at: ExpiryTiming::EndOfTurn,
            });
        }
    }
}
```

#### Goad

Goad forces creatures to attack players other than you:

```rust
/// Component marking a creature as Goaded
#[derive(Component)]
pub struct Goaded {
    /// The player who applied the goad effect
    pub goaded_by: Entity,
    /// When the goad effect expires
    pub expires_at: ExpiryTiming,
}

/// System that enforces Goad restrictions during attacks
pub fn enforce_goad_restrictions(
    goaded_creatures: Query<(Entity, &Goaded)>,
    mut attack_validation_events: EventReader<ValidateAttackEvent>,
    mut attack_response_events: EventWriter<AttackValidationResponse>,
) {
    for event in attack_validation_events.read() {
        if let Ok((_, goaded)) = goaded_creatures.get(event.attacker) {
            // If attacking the player who goaded this creature, attack is invalid
            if event.defender == goaded.goaded_by {
                attack_response_events.send(AttackValidationResponse {
                    attacker: event.attacker,
                    defender: event.defender,
                    is_valid: false,
                    reason: "This creature is goaded and must attack a different player if able".to_string(),
                });
            }
        }
    }
}

/// System that enforces Goad requirements to attack if able
pub fn enforce_goad_attack_requirement(
    goaded_creatures: Query<(Entity, &Goaded, &CanAttack, &Controller)>,
    mut attack_requirement_events: EventReader<AttackRequirementCheckEvent>,
    mut attack_required_events: EventWriter<AttackRequiredEvent>,
    players: Query<Entity, With<Player>>,
) {
    for event in attack_requirement_events.read() {
        for (goaded_entity, goaded, can_attack, controller) in goaded_creatures.iter() {
            // If creature can attack and is controlled by current player
            if can_attack.0 && controller.0 == event.attacking_player {
                // Find valid attack targets (not the player who goaded)
                let valid_targets: Vec<Entity> = players
                    .iter()
                    .filter(|player| *player != goaded.goaded_by && *player != controller.0)
                    .collect();
                
                // If there are valid targets, this creature must attack
                if !valid_targets.is_empty() {
                    attack_required_events.send(AttackRequiredEvent {
                        creature: goaded_entity,
                        valid_targets,
                    });
                }
            }
        }
    }
}
```

### Commander-Specific Cycles and Card Groups

#### Medallion Cycle

The Medallion cycle (Ruby Medallion, Sapphire Medallion, etc.) reduces costs for spells of specific colors:

```rust
pub fn create_ruby_medallion() -> impl Bundle {
    (
        CardName("Ruby Medallion".to_string()),
        CardType::Artifact,
        StaticAbility {
            id: AbilityId::new(),
            effect: Box::new(ColorCostReductionEffect(Color::Red)),
            condition: StaticCondition::Always,
        },
    )
}
```

#### Commander's Plate

A special equipment that gives protection from colors outside your commander's color identity:

```rust
pub fn create_commanders_plate() -> impl Bundle {
    (
        CardName("Commander's Plate".to_string()),
        CardType::Artifact,
        ArtifactType(vec!["Equipment".to_string()]),
        EquipCost(ManaCost::parse("{3}")),
        StaticAbility {
            id: AbilityId::new(),
            effect: Box::new(CommandersPlateEffect),
            condition: StaticCondition::IsEquipped,
        },
    )
}

#[derive(Debug, Clone)]
pub struct CommandersPlateEffect;

impl StaticEffect for CommandersPlateEffect {
    fn apply(&self, world: &mut World, source: Entity, target: Entity) {
        // Get equipped creature's controller
        let controller = world.get::<Controller>(target).unwrap().0;
        
        // Get controller's commanders' color identity
        let commander_identity = get_commander_color_identity(world, controller);
        
        // Grant protection from colors outside that identity
        let protection_colors = ALL_COLORS.iter()
            .filter(|color| !commander_identity.contains(**color))
            .copied()
            .collect();
        
        world.entity_mut(target).insert(Protection {
            protection_from: ProtectionType::Colors(protection_colors),
            source,
        });
        
        // Also grant stat boosts
        world.entity_mut(target).insert(StatBoost {
            power: 3,
            toughness: 3,
            source,
        });
    }
}
```

## Testing Commander-Specific Cards

Testing commander-specific cards requires special test fixtures and scenarios:

```rust
#[test]
fn test_command_beacon() {
    let mut app = App::new();
    setup_test_game(&mut app);
    
    let player = app.world.spawn(Player).id();
    
    // Create a commander with tax
    let commander = app.world.spawn((
        CardName("Test Commander".to_string()),
        Commander,
        InCommandZone(player),
        CommanderCastCount(2),  // Commander has been cast twice before
    )).id();
    
    // Create Command Beacon
    let beacon = app.world.spawn(create_command_beacon()).id();
    
    // Use Command Beacon's ability
    use_ability(beacon, player, &mut app.world);
    
    // Verify commander moved to hand
    assert!(has_component::<InHand>(&app.world, commander));
    assert!(!has_component::<InCommandZone>(&app.world, commander));
    
    // Verify commander can be cast without tax
    let cast_cost = calculate_commander_cast_cost(&app.world, commander, player);
    assert_eq!(cast_cost, commander_base_cost(&app.world, commander));
}
```

## UI Considerations

Commander-specific cards require special UI treatment:

1. Command zone interactions need visual clarity
2. Political mechanics need multiplayer-aware UI elements
3. Special effects like Monarch need distinctive visual indicators

```rust
/// UI component for displaying monarchy status
#[derive(Component)]
pub struct MonarchyIndicator {
    pub entity: Entity,
}

/// System to update monarchy UI
pub fn update_monarchy_ui(
    monarch_state: Res<MonarchState>,
    mut indicators: Query<(&mut Visibility, &MonarchyIndicator)>,
) {
    for (mut visibility, indicator) in indicators.iter_mut() {
        visibility.is_visible = monarch_state.current_monarch == Some(indicator.entity);
    }
}
```

## Commander Preconstructed Deck Integration

Our system includes support for preconstructed Commander decks:

```rust
/// Resource containing preconstructed Commander deck definitions
#[derive(Resource)]
pub struct PreconstructedDecks {
    pub decks: HashMap<String, PreconstructedDeckData>,
}

/// Data for a preconstructed Commander deck
#[derive(Clone, Debug)]
pub struct PreconstructedDeckData {
    /// Deck name
    pub name: String,
    /// The set/product the deck is from
    pub product: String,
    /// Year released
    pub year: u32,
    /// Primary commander
    pub commander: String,
    /// Secondary commander/partner (if any)
    pub partner: Option<String>,
    /// List of all cards in the deck
    pub card_list: Vec<String>,
    /// Deck color identity
    pub color_identity: ColorIdentity,
    /// Deck theme or strategy
    pub theme: String,
}

/// Function to load all preconstructed Commander decks
pub fn load_preconstructed_decks() -> PreconstructedDecks {
    // Load deck definitions from files
    // ...
}
```

## Conclusion

Commander-specific cards are a vital part of the format's identity. By implementing these cards correctly, we ensure that our game engine provides an authentic Commander experience. The implementation must balance rules accuracy with performance considerations, especially for complex political mechanics in multiplayer games.

## Related Resources

- [Commander Format Rules](../overview/format_rules.md)
- [Command Zone Implementation](../zones/command_zone.md)
- [Multiplayer Politics](multiplayer_politics.md)
- [Commander Tax](../player_mechanics/commander_tax.md) 