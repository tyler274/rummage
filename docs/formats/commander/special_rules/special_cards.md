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

Cards that introduce the Monarch mechanic:

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
    pub last_changed: Instant,
    /// How many cards the current monarch has drawn from being monarch
    pub cards_drawn: u32,
}

/// Implementation of Queen Marchesa
pub fn create_queen_marchesa() -> impl Bundle {
    (
        CardName("Queen Marchesa".to_string()),
        CardType::Creature,
        CreatureType(vec!["Human".to_string(), "Assassin".to_string()]),
        Power(3),
        Toughness(3),
        EntersBattlefieldTriggeredAbility {
            id: AbilityId::new(),
            effect: Box::new(BecomeMonarchEffect),
        },
    )
}

/// System that handles monarch card draw
pub fn monarch_upkeep_draw(
    mut commands: Commands,
    monarch_state: Res<MonarchState>,
    mut draw_events: EventWriter<DrawCardEvent>,
    mut phase_events: EventReader<PhaseChangeEvent>,
) {
    for event in phase_events.read() {
        // Check for end step
        if matches!(event.new_phase, Phase::Ending(EndingStep::End)) {
            if let Some(monarch) = monarch_state.current_monarch {
                // Monarch draws a card
                draw_events.send(DrawCardEvent {
                    player: monarch,
                    amount: 1,
                    reason: DrawReason::Ability {
                        name: "Monarch".to_string(),
                    },
                });
            }
        }
    }
}
```

#### Myriad

Cards with the Myriad mechanic:

```rust
/// Component for creatures with Myriad
#[derive(Component)]
pub struct Myriad;

/// System that handles Myriad attacks
pub fn handle_myriad_attacks(
    mut commands: Commands,
    myriad_creatures: Query<(Entity, &Myriad, &AttackingStatus, &Owner)>,
    players: Query<Entity, With<Player>>,
    mut token_events: EventWriter<CreateTokenEvent>,
    mut phase_events: EventReader<PhaseChangeEvent>,
) {
    for event in phase_events.read() {
        // Check for declare attackers step
        if matches!(event.new_phase, Phase::Combat(CombatStep::DeclareAttackers)) {
            for (entity, _, attacking_status, owner) in myriad_creatures.iter() {
                if !attacking_status.attacking {
                    continue;
                }
                
                // Get the player being attacked
                let defending_player = attacking_status.defending_player.unwrap();
                
                // Create token copies attacking each other opponent
                for potential_defender in players.iter() {
                    if potential_defender != owner.0 && potential_defender != defending_player {
                        token_events.send(CreateTokenEvent {
                            token_source: entity,
                            owner: owner.0,
                            attacking: Some(potential_defender),
                            modifications: vec![TokenModification::ExileAtEndOfCombat],
                        });
                    }
                }
            }
        }
    }
}
```

### Commander Format-Specific Cards

Cards that reference specific Commander rules:

#### Command Tax Interaction

```rust
/// Component for effects that interact with command tax
#[derive(Component)]
pub struct CommandTaxInteraction {
    /// How the effect interacts with command tax
    pub interaction_type: CommandTaxEffect,
}

/// Types of command tax effects
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandTaxEffect {
    /// Reduce command tax (e.g., Derevi)
    Ignore,
    /// Set command tax to a specific value
    Set(u32),
    /// Modify command tax by an amount
    Modify(i32),
}

/// Implementation of Derevi, Empyrial Tactician
pub fn create_derevi() -> impl Bundle {
    (
        CardName("Derevi, Empyrial Tactician".to_string()),
        CardType::Creature,
        CreatureType(vec!["Bird".to_string(), "Wizard".to_string()]),
        Power(2),
        Toughness(3),
        Commander,
        StaticAbility {
            id: AbilityId::new(),
            effect: Box::new(IgnoreCommanderTaxEffect),
            condition: StaticCondition::Always,
        },
        ActivatedAbility {
            id: AbilityId::new(),
            cost: AbilityCost::Mana(ManaCost::parse("{1}{G}{W}{U}")),
            effect: Box::new(PutOntoBattlefieldEffect),
            timing_restriction: TimingRestriction::Instant,
            zone_restriction: ZoneRestriction::InCommandZone,
        },
    )
}
```

#### Partner Interaction

```rust
/// Implementation of Thrasios, Triton Hero
pub fn create_thrasios() -> impl Bundle {
    (
        CardName("Thrasios, Triton Hero".to_string()),
        CardType::Creature,
        CreatureType(vec!["Merfolk".to_string(), "Wizard".to_string()]),
        Power(1),
        Toughness(3),
        Commander,
        PartnerType::Universal,
        ActivatedAbility {
            id: AbilityId::new(),
            cost: AbilityCost::Mana(ManaCost::parse("{4}")),
            effect: Box::new(ThrasiosScryRevealEffect),
            timing_restriction: TimingRestriction::Instant,
            zone_restriction: ZoneRestriction::OnBattlefield,
        },
    )
}
```

## Implementation Considerations

### Card Database Integration

Commander-specific cards need special handling in the card database:

```rust
/// Function to load Commander-specific cards
pub fn load_commander_cards(card_db: &mut CardDatabase) {
    // Add Commander-specific cards
    card_db.add_card(create_command_beacon);
    card_db.add_card(create_councils_judgment);
    card_db.add_card(create_thunderfoot_baloth);
    card_db.add_card(create_queen_marchesa);
    card_db.add_card(create_derevi);
    card_db.add_card(create_thrasios);
    // And many more...
}
```

### Legality Checking

Some cards are specifically banned in Commander:

```rust
/// Function to check if a card is legal in Commander
pub fn is_legal_in_commander(card_name: &str) -> bool {
    // List of cards banned in Commander
    const BANNED_CARDS: &[&str] = &[
        "Ancestral Recall",
        "Balance",
        "Biorhythm",
        "Black Lotus",
        // Many more...
    ];
    
    !BANNED_CARDS.contains(&card_name)
}
```

### Testing Commander-Specific Cards

```rust
#[test]
fn test_command_beacon() {
    let mut app = App::new();
    app.add_systems(Startup, setup_test);
    app.add_systems(Update, handle_command_beacon_ability);
    
    // Create test entities
    let player = app.world.spawn_empty().id();
    
    // Create a commander in the command zone
    let commander = app.world.spawn((
        CardName("Test Commander".to_string()),
        Commander,
        Owner(player),
    )).id();
    
    // Add to command zone
    app.world.resource_mut::<Zones>().command.insert(commander);
    
    // Create Command Beacon
    let beacon = app.world.spawn((
        CardName("Command Beacon".to_string()),
        ActivatedAbility {
            id: AbilityId::new(),
            cost: AbilityCost::Sacrifice(SacrificeCost::Self_),
            effect: Box::new(CommandBeaconEffect),
            timing_restriction: TimingRestriction::Sorcery,
            zone_restriction: ZoneRestriction::OnBattlefield,
        },
        Controller(player),
    )).id();
    
    // Activate Command Beacon
    app.world.send_event(ActivateAbilityEvent {
        source: beacon,
        ability_id: app.world.get::<ActivatedAbility>(beacon).unwrap().id,
        controller: player,
        targets: vec![],
    });
    
    app.update();
    
    // Verify commander moved to hand
    let zones = app.world.resource::<Zones>();
    assert!(!zones.command.contains(&commander));
    assert!(zones.hand.contains(&commander));
}
```

## User Interface Considerations

Commander-specific cards often require special UI handling:

1. Command zone access needs proper visibility
2. Voting mechanics need UI for choices
3. Monarch state should be clearly indicated
4. Partner commander interactions need clear UI representation

## Related Documentation

- [Partner Commanders](partner_commanders.md): Detailed implementation of partner mechanics
- [Commander Ninjutsu](commander_ninjutsu.md): Implementation of the Commander Ninjutsu mechanic
- [Commander Death Triggers](commander_death.md): How commander death interactions are handled
- [Command Zone](../zones/command_zone.md): Implementation of the command zone 