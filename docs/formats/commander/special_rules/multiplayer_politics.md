# Multiplayer Politics

## Overview

The Multiplayer Politics module handles the social and strategic elements unique to multiplayer Commander games. This includes deal-making, alliance formation, temporary agreements, voting mechanics, and other player interactions not codified in the standard Magic rules.

## Core Features

The system includes:

- **Deal Making System**: Framework for players to propose, accept, and track in-game deals
- **Voting Mechanics**: Implementation of cards with Council's Dilemma, Will of the Council, and other voting abilities
- **Threat Assessment**: Tools to analyze and display relative threat levels of players
- **Alliance Tracking**: Temporary cooperative arrangements between players
- **Table Talk Integration**: Support for in-game communication with policy enforcement
- **Goad Mechanics**: Implementation of abilities that force creatures to attack

## Implementation

The politics system is implemented through several interconnected components:

```rust
#[derive(Component)]
pub struct PoliticsComponent {
    // Current deals, alliances, and political state
    pub active_deals: Vec<Deal>,
    pub alliances: HashMap<Entity, AllianceStrength>,
    pub political_capital: f32,
    pub trust_level: HashMap<Entity, TrustLevel>,
    
    // Historical tracking
    pub broken_deals: Vec<BrokenDeal>,
    pub past_alliances: Vec<PastAlliance>,
}

#[derive(Resource)]
pub struct PoliticsSystem {
    // Global politics configuration
    pub enable_deals: bool,
    pub allow_secret_deals: bool,
    pub deal_enforcement_level: DealEnforcementLevel,
    
    // Event history
    pub political_events: VecDeque<PoliticalEvent>,
}
```

### Deal Structure

Deals are structured entities that capture player agreements:

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Deal {
    /// Unique identifier for the deal
    pub id: Uuid,
    /// Player who proposed the deal
    pub proposer: Entity,
    /// Player(s) who accepted the deal
    pub acceptors: Vec<Entity>,
    /// Terms of the deal - what each party agrees to do
    pub terms: Vec<DealTerm>,
    /// When the deal expires (if temporary)
    pub expiration: Option<DealExpiration>,
    /// Current status of the deal
    pub status: DealStatus,
    /// When the deal was created
    pub created_at: f64,
    /// When the deal was last updated
    pub last_updated: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DealTerm {
    /// Promise not to attack a player for N turns
    NonAggression {
        target: Entity,
        duration: u32,
    },
    /// Promise to attack a specific player
    AttackPlayer {
        target: Entity,
        next_turn_only: bool,
    },
    /// Promise not to counter a player's next spell
    NoCounterspell {
        target: Entity,
        duration: u32,
    },
    /// Promise to share resources (e.g. let them draw when you draw)
    ShareResource {
        resource_type: ResourceType,
        target: Entity,
        amount: u32,
    },
    /// Promise to vote with a player on the next vote
    VoteAlignment {
        ally: Entity,
        vote_count: u32,
    },
    /// Custom term with free-form text
    Custom {
        description: String,
    },
}
```

## Deal Making

The deal making system allows players to:

- Propose deals with specific terms and duration
- Accept or reject deals from other players
- Set automatic deal conditions and consequences
- Track deal fulfillment and violations

Deals are non-binding at the rules level but provide framework for player agreements.

```rust
/// System that handles deal proposals
pub fn handle_deal_proposals(
    mut commands: Commands,
    mut deal_events: EventReader<DealProposalEvent>,
    mut deal_response_events: EventWriter<DealResponseEvent>,
    mut politics_components: Query<&mut PoliticsComponent>,
) {
    for event in deal_events.read() {
        if let Ok(mut proposer_politics) = politics_components.get_mut(event.proposer) {
            // Create new deal
            let deal = Deal {
                id: Uuid::new_v4(),
                proposer: event.proposer,
                acceptors: Vec::new(),
                terms: event.terms.clone(),
                expiration: event.expiration.clone(),
                status: DealStatus::Proposed,
                created_at: event.timestamp,
                last_updated: event.timestamp,
            };
            
            // Add to proposer's active deals
            proposer_politics.active_deals.push(deal.clone());
            
            // Notify target players
            for target in &event.targets {
                deal_response_events.send(DealResponseEvent {
                    deal_id: deal.id,
                    response_type: DealResponseType::Offered,
                    player: *target,
                    timestamp: event.timestamp,
                });
            }
        }
    }
}

/// System that handles deal responses
pub fn handle_deal_responses(
    mut commands: Commands,
    mut deal_response_events: EventReader<DealResponseEvent>,
    mut politics_components: Query<&mut PoliticsComponent>,
    mut deal_update_events: EventWriter<DealUpdateEvent>,
) {
    for event in deal_response_events.read() {
        // Handle player responses to deals
        match event.response_type {
            DealResponseType::Accept => {
                // Update the deal status
                for (entity, mut politics) in politics_components.iter_mut() {
                    for deal in &mut politics.active_deals {
                        if deal.id == event.deal_id {
                            deal.acceptors.push(event.player);
                            deal.last_updated = event.timestamp;
                            
                            // If all targets have accepted, activate the deal
                            if deal.acceptors.len() >= deal.terms.len() {
                                deal.status = DealStatus::Active;
                                deal_update_events.send(DealUpdateEvent {
                                    deal_id: deal.id,
                                    new_status: DealStatus::Active,
                                    timestamp: event.timestamp,
                                });
                            }
                            
                            break;
                        }
                    }
                }
            },
            DealResponseType::Reject => {
                // Handle rejection
                for (entity, mut politics) in politics_components.iter_mut() {
                    for deal in &mut politics.active_deals {
                        if deal.id == event.deal_id {
                            deal.status = DealStatus::Rejected;
                            deal.last_updated = event.timestamp;
                            
                            deal_update_events.send(DealUpdateEvent {
                                deal_id: deal.id,
                                new_status: DealStatus::Rejected,
                                timestamp: event.timestamp,
                            });
                            
                            break;
                        }
                    }
                }
            },
            // Handle other response types
            _ => {}
        }
    }
}
```

## Voting Mechanics

Many Commander-specific cards feature voting mechanics (Council's Dilemma, Will of the Council). The voting system handles these cards' abilities:

```rust
/// Component for cards with voting mechanics
#[derive(Component)]
pub struct VotingMechanic {
    /// The type of voting mechanic
    pub voting_type: VotingType,
    /// The available options to vote for
    pub options: Vec<String>,
    /// How the results are applied
    pub resolution: VoteResolutionMethod,
}

/// Types of voting mechanics
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VotingType {
    /// Will of the Council - each player gets one vote
    WillOfCouncil,
    /// Council's Dilemma - each player votes for two different options
    CouncilsDilemma,
    /// Parley - a special voting variant
    Parley,
    /// Custom voting system
    Custom,
}

/// System for handling vote card resolution
pub fn handle_voting_resolution(
    mut commands: Commands,
    mut vote_events: EventReader<VoteCompletionEvent>,
    voting_cards: Query<(Entity, &VotingMechanic)>,
    players: Query<Entity, With<Player>>,
) {
    for event in vote_events.read() {
        if let Ok((card_entity, voting_mechanic)) = voting_cards.get(event.source_card) {
            // Collect and tally votes
            let mut vote_counts: HashMap<String, u32> = HashMap::new();
            
            for (player, vote) in &event.votes {
                *vote_counts.entry(vote.clone()).or_default() += 1;
            }
            
            // Apply effects based on voting results and resolution method
            match voting_mechanic.resolution {
                VoteResolutionMethod::MostVotes => {
                    // Find option with most votes
                    if let Some((winning_option, _)) = vote_counts
                        .iter()
                        .max_by_key(|(_, count)| *count) {
                        
                        // Apply effect for winning option
                        apply_voting_effect(
                            &mut commands, 
                            card_entity, 
                            winning_option, 
                            &event.votes
                        );
                    }
                },
                VoteResolutionMethod::AllVotes => {
                    // Apply effect for each vote
                    for (option, count) in vote_counts {
                        // Apply effect scaled by vote count
                        apply_voting_effect_scaled(
                            &mut commands,
                            card_entity,
                            &option,
                            count,
                            &event.votes
                        );
                    }
                },
                // Handle other resolution methods
                _ => {}
            }
        }
    }
}
```

### Example Voting Card Implementation

```rust
/// Implementation of Councils' Judgment
pub fn create_councils_judgment() -> impl Bundle {
    (
        CardName("Council's Judgment".to_string()),
        CardType::Sorcery,
        ManaCost::parse("{1}{W}{W}"),
        VotingMechanic {
            voting_type: VotingType::WillOfCouncil,
            options: vec!["Exile".to_string()], // Dynamic: each nonland permanent becomes an option
            resolution: VoteResolutionMethod::MostVotes,
        },
        CouncilsJudgmentEffect,
    )
}

#[derive(Component)]
pub struct CouncilsJudgmentEffect;

impl ResolveEffect for CouncilsJudgmentEffect {
    fn resolve(&self, world: &mut World, source: Entity, controller: Entity) {
        // Get all permanents as potential targets
        let targets: Vec<Entity> = get_valid_permanent_targets(world);
        
        // Start voting process
        world.send_event(InitiateVoteEvent {
            source: source,
            voting_type: VotingType::WillOfCouncil,
            options: targets.iter().map(|e| get_name_for_entity(world, *e)).collect(),
            initiator: controller,
        });
    }
}
```

## Threat Assessment

The threat assessment system helps players evaluate relative threats by:

- Displaying board state power metrics
- Tracking win proximity indicators
- Highlighting potential combo pieces
- Providing history of player actions and tendencies

```rust
/// Resource for tracking threat assessment
#[derive(Resource)]
pub struct ThreatAssessment {
    /// Calculated threat level for each player
    pub player_threats: HashMap<Entity, ThreatLevel>,
    /// Factors contributing to threat calculation
    pub threat_factors: HashMap<Entity, Vec<ThreatFactor>>,
    /// Historical threat trends
    pub threat_history: HashMap<Entity, VecDeque<HistoricalThreat>>,
}

/// System that updates threat assessment
pub fn update_threat_assessment(
    mut threat_assessment: ResMut<ThreatAssessment>,
    players: Query<Entity, With<Player>>,
    life_totals: Query<&LifeTotal>,
    permanents: Query<(Entity, &Controller)>,
    commanders: Query<(Entity, &Commander, &Controller)>,
    graveyards: Query<(&Graveyard, &Owner)>,
    hands: Query<(&Hand, &Owner)>,
) {
    // Update threat metrics for each player
    for player in players.iter() {
        let mut threat_factors = Vec::new();
        
        // Factor: Board presence
        let board_presence = permanents
            .iter()
            .filter(|(_, controller)| controller.0 == player)
            .count();
        
        threat_factors.push(ThreatFactor {
            factor_type: ThreatFactorType::BoardPresence,
            value: board_presence as f32 * 0.5,
        });
        
        // Factor: Commander damage potential
        if let Some((_, _, _)) = commanders
            .iter()
            .find(|(_, _, controller)| controller.0 == player) {
            // Calculate commander threat...
            threat_factors.push(ThreatFactor {
                factor_type: ThreatFactorType::CommanderPresence,
                value: 5.0, // Base threat for having commander
            });
        }
        
        // Calculate other factors...
        
        // Update total threat
        let total_threat = threat_factors.iter().map(|f| f.value).sum();
        
        threat_assessment.player_threats.insert(player, ThreatLevel(total_threat));
        threat_assessment.threat_factors.insert(player, threat_factors);
        
        // Update history
        threat_assessment.threat_history
            .entry(player)
            .or_default()
            .push_back(HistoricalThreat {
                level: ThreatLevel(total_threat),
                turn: get_current_turn(),
            });
    }
}
```

## Goad Mechanics

Goad is a Commander-specific mechanic that forces creatures to attack:

```rust
/// Component for Goad effects
#[derive(Component)]
pub struct Goaded {
    /// The player who applied the goad effect
    pub goaded_by: Entity,
    /// When the goad effect expires
    pub expires_at: ExpiryTiming,
}

/// System that enforces Goad attack requirements
pub fn enforce_goad_attack_requirement(
    goaded_creatures: Query<(Entity, &Goaded, &Controller)>,
    mut attack_requirement_events: EventWriter<AttackRequirementEvent>,
    turn_manager: Res<TurnManager>,
) {
    // Only check during active player's combat
    if turn_manager.current_phase != Phase::Combat(CombatStep::DeclareAttackers) {
        return;
    }
    
    let active_player = turn_manager.active_player;
    
    // Find creatures controlled by active player that are goaded
    for (entity, goaded, controller) in goaded_creatures.iter() {
        if controller.0 == active_player {
            // Creature must attack if able, and cannot attack player who goaded it
            attack_requirement_events.send(AttackRequirementEvent {
                creature: entity,
                must_attack: true,
                cannot_attack: vec![goaded.goaded_by],
            });
        }
    }
}
```

## Alliance Tracking

Alliances are temporary arrangements between players:

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Alliance {
    /// Players in the alliance
    pub members: Vec<Entity>,
    /// Strength/type of alliance
    pub strength: AllianceStrength,
    /// Purpose of the alliance
    pub purpose: String,
    /// When the alliance was formed
    pub formed_at: f64,
    /// When the alliance expires (if temporary)
    pub expires_at: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AllianceStrength {
    /// Weak alliance of convenience
    Weak,
    /// Standard alliance
    Moderate,
    /// Strong alliance with significant shared interests
    Strong,
}

/// System for tracking and updating alliances
pub fn update_alliances(
    mut politics_components: Query<&mut PoliticsComponent>,
    time: Res<Time>,
    mut alliance_events: EventReader<AllianceEvent>,
) {
    let current_time = time.elapsed_seconds_f64();
    
    // Process alliance events
    for event in alliance_events.read() {
        match event.event_type {
            AllianceEventType::Form => {
                // Create new alliance
                let alliance = Alliance {
                    members: event.members.clone(),
                    strength: event.strength.clone(),
                    purpose: event.purpose.clone(),
                    formed_at: current_time,
                    expires_at: event.expiration,
                };
                
                // Update politics components for all members
                for member in &event.members {
                    if let Ok(mut politics) = politics_components.get_mut(*member) {
                        for other_member in &event.members {
                            if *other_member != *member {
                                politics.alliances.insert(*other_member, alliance.strength.clone());
                            }
                        }
                    }
                }
            },
            AllianceEventType::Break => {
                // Handle alliance breaking
                for member in &event.members {
                    if let Ok(mut politics) = politics_components.get_mut(*member) {
                        for other_member in &event.members {
                            if *other_member != *member {
                                politics.alliances.remove(other_member);
                                
                                // Record broken alliance in history
                                politics.past_alliances.push(PastAlliance {
                                    with: *other_member,
                                    strength: event.strength.clone(),
                                    purpose: event.purpose.clone(),
                                    formed_at: event.formed_at.unwrap_or(0.0),
                                    broken_at: current_time,
                                    broken_reason: event.reason.clone(),
                                });
                            }
                        }
                    }
                }
            },
        }
    }
    
    // Check for expired alliances
    for mut politics in politics_components.iter_mut() {
        // Implementation for alliance expiration...
    }
}
```

## AI Integration

For games with AI opponents, the politics system:

- Models AI political decision making based on configured personalities
- Evaluates deal proposals based on game state and risk assessment
- Tracks human player tendencies for future political decisions
- Simulates realistic political behavior for different AI difficulty levels

```rust
#[derive(Resource)]
pub struct AIPoliticsConfig {
    /// AI personality profiles
    pub personalities: HashMap<Entity, AIPoliticalPersonality>,
    /// AI decision making parameters
    pub decision_weights: AIPoliticsWeights,
    /// Historic interaction with players
    pub player_interaction_history: HashMap<Entity, PlayerInteractionHistory>,
}

#[derive(Clone, Debug)]
pub struct AIPoliticalPersonality {
    /// How aggressive the AI is
    pub aggression: f32,
    /// How trustworthy the AI is
    pub trustworthiness: f32,
    /// How risk-averse the AI is
    pub risk_aversion: f32,
    /// How vengeful the AI is
    pub vengefulness: f32,
    /// How the AI evaluates deals
    pub deal_evaluation_strategy: DealEvaluationStrategy,
}

/// System for AI deal evaluation
pub fn ai_evaluate_deal(
    ai_politics_config: Res<AIPoliticsConfig>,
    threat_assessment: Res<ThreatAssessment>,
    board_state: Res<BoardState>,
    deal: &Deal,
    ai_player: Entity,
) -> DealEvaluationResult {
    if let Some(personality) = ai_politics_config.personalities.get(&ai_player) {
        // Get AI personality
        let trust_factor = personality.trustworthiness;
        let risk_factor = personality.risk_aversion;
        
        // Calculate deal value
        let mut deal_value = 0.0;
        
        for term in &deal.terms {
            match term {
                DealTerm::NonAggression { target, duration } => {
                    // Value based on target threat and duration
                    let target_threat = threat_assessment.player_threats
                        .get(target)
                        .map(|t| t.0)
                        .unwrap_or(0.0);
                    
                    deal_value += target_threat * (*duration as f32) * 0.5;
                },
                // Evaluate other term types...
                _ => {},
            }
        }
        
        // Adjust for proposer's trustworthiness
        let proposer_trust = ai_politics_config.player_interaction_history
            .get(&ai_player)
            .and_then(|h| h.player_trust.get(&deal.proposer))
            .copied()
            .unwrap_or(0.5);
        
        deal_value *= proposer_trust;
        
        // Make decision based on value
        if deal_value > personality.deal_threshold {
            DealEvaluationResult::Accept
        } else {
            DealEvaluationResult::Reject { reason: "Not valuable enough".to_string() }
        }
    } else {
        // Default rejection if no personality
        DealEvaluationResult::Reject { reason: "No AI personality configured".to_string() }
    }
}
```

## UI Components

The multiplayer politics UI provides:

- Deal proposal interface with customizable terms
- Alliance status indicators
- Threat assessment visualization
- Communication tools with appropriate filters
- Deal history and player reputation tracking

```rust
#[derive(Component)]
pub struct PoliticsUIState {
    /// Currently selected player for political actions
    pub selected_player: Option<Entity>,
    /// Deal being constructed
    pub draft_deal: Option<DraftDeal>,
    /// UI mode
    pub ui_mode: PoliticsUIMode,
}

/// System for rendering politics UI
pub fn render_politics_ui(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    politics_ui_state: Res<PoliticsUIState>,
    politics_components: Query<&PoliticsComponent>,
    players: Query<(Entity, &PlayerName)>,
    threat_assessment: Res<ThreatAssessment>,
) {
    // Implementation of politics UI rendering...
}
```

## Integration with Other Systems

The politics system integrates with several other game systems:

### Combat System Integration

```rust
/// System for applying political factors to combat
pub fn apply_politics_to_combat(
    politics_components: Query<&PoliticsComponent>,
    mut attack_events: EventReader<DeclareAttackerEvent>,
    mut attack_modifiers: EventWriter<AttackModifierEvent>,
    alliances: Query<&Alliance>,
) {
    for event in attack_events.read() {
        if let Ok(attacker_politics) = politics_components.get(event.controller) {
            // Check for deals preventing attacks
            for deal in &attacker_politics.active_deals {
                for term in &deal.terms {
                    if let DealTerm::NonAggression { target, duration } = term {
                        if *target == event.defender && deal.status == DealStatus::Active {
                            // This attack would violate a non-aggression deal
                            attack_modifiers.send(AttackModifierEvent {
                                attacker: event.attacker,
                                defender: event.defender,
                                modification: AttackModification::PreventAttack {
                                    reason: "Non-aggression deal in effect".to_string(),
                                },
                            });
                        }
                    }
                }
            }
            
            // Check for alliances
            if let Some(alliance_strength) = attacker_politics.alliances.get(&event.defender) {
                // Alliance strength affects whether attack is allowed
                if *alliance_strength == AllianceStrength::Strong {
                    attack_modifiers.send(AttackModifierEvent {
                        attacker: event.attacker,
                        defender: event.defender,
                        modification: AttackModification::DissuadeAttack {
                            reason: "Strong alliance in effect".to_string(),
                            penalty: 2.0,
                        },
                    });
                }
            }
        }
    }
}
```

### Card Effect Integration

```rust
/// System for applying political factors to targeted effects
pub fn apply_politics_to_targeting(
    politics_components: Query<&PoliticsComponent>,
    mut targeting_events: EventReader<TargetSelectionEvent>,
    mut targeting_modifiers: EventWriter<TargetingModifierEvent>,
) {
    for event in targeting_events.read() {
        if let Ok(caster_politics) = politics_components.get(event.controller) {
            // Check for deals affecting targeting
            for deal in &caster_politics.active_deals {
                for term in &deal.terms {
                    // Handle various deal terms affecting targeting
                    match term {
                        DealTerm::NoCounterspell { target, .. } => {
                            if *target == event.target_controller && 
                               is_counterspell(event.source) {
                                // This targeting would violate a no-counterspell deal
                                targeting_modifiers.send(TargetingModifierEvent {
                                    source: event.source,
                                    target: event.target,
                                    modification: TargetingModification::PreventTargeting {
                                        reason: "No-counterspell deal in effect".to_string(),
                                    },
                                });
                            }
                        },
                        // Handle other deal terms...
                        _ => {},
                    }
                }
            }
            
            // Check for alliances affecting targeting
            if let Some(alliance_strength) = caster_politics.alliances.get(&event.target_controller) {
                if is_negative_effect(event.source, event.target) {
                    match alliance_strength {
                        AllianceStrength::Strong => {
                            targeting_modifiers.send(TargetingModifierEvent {
                                source: event.source,
                                target: event.target,
                                modification: TargetingModification::DissuadeTargeting {
                                    reason: "Strong alliance in effect".to_string(),
                                    penalty: 3.0,
                                },
                            });
                        },
                        // Handle other alliance levels...
                        _ => {},
                    }
                }
            }
        }
    }
}
```

## Constraints and Limitations

The politics system operates within these constraints:

- No rules enforcement of political agreements (maintaining game integrity)
- Appropriate limits on information sharing for hidden information
- Configurable table talk policies to match playgroup preferences
- Balance between automation and player agency in political decisions

## Testing Politics Features

```rust
#[test]
fn test_deal_creation_and_acceptance() {
    let mut app = App::new();
    setup_test_game(&mut app);
    
    // Create players
    let player1 = app.world.spawn((
        Player,
        PlayerName("Player 1".to_string()),
        PoliticsComponent::default(),
    )).id();
    
    let player2 = app.world.spawn((
        Player,
        PlayerName("Player 2".to_string()),
        PoliticsComponent::default(),
    )).id();
    
    // Create a deal proposal
    let deal_terms = vec![
        DealTerm::NonAggression {
            target: player2,
            duration: 2,
        },
    ];
    
    app.world.send_event(DealProposalEvent {
        proposer: player1,
        targets: vec![player2],
        terms: deal_terms,
        expiration: Some(DealExpiration::Turns(2)),
        timestamp: 0.0,
    });
    
    app.update();
    
    // Check that deal was created
    let politics1 = app.world.get::<PoliticsComponent>(player1).unwrap();
    assert_eq!(politics1.active_deals.len(), 1);
    assert_eq!(politics1.active_deals[0].status, DealStatus::Proposed);
    
    // Accept the deal
    let deal_id = politics1.active_deals[0].id;
    app.world.send_event(DealResponseEvent {
        deal_id,
        response_type: DealResponseType::Accept,
        player: player2,
        timestamp: 1.0,
    });
    
    app.update();
    
    // Verify deal is now active
    let politics1_updated = app.world.get::<PoliticsComponent>(player1).unwrap();
    assert_eq!(politics1_updated.active_deals[0].status, DealStatus::Active);
    assert_eq!(politics1_updated.active_deals[0].acceptors, vec![player2]);
}

#[test]
fn test_goad_mechanics() {
    let mut app = App::new();
    setup_test_game(&mut app);
    
    // Create players
    let player1 = app.world.spawn(Player).id();
    let player2 = app.world.spawn(Player).id();
    
    // Create a creature
    let creature = app.world.spawn((
        CardName("Test Creature".to_string()),
        CardType::Creature,
        Power(3),
        Toughness(3),
        Controller(player1),
        OnBattlefield,
    )).id();
    
    // Goad the creature
    app.world.entity_mut(creature).insert(Goaded {
        goaded_by: player2,
        expires_at: ExpiryTiming::EndOfTurn,
    });
    
    // Set up combat phase
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::DeclareAttackers);
    app.world.resource_mut::<TurnManager>().active_player = player1;
    
    app.update();
    
    // Get attack requirements
    let attack_requirements = app.world.resource::<Events<AttackRequirementEvent>>()
        .get_reader()
        .read(&app.world.resource::<Events<AttackRequirementEvent>>())
        .collect::<Vec<_>>();
    
    // Verify goad requirements are enforced
    assert!(!attack_requirements.is_empty());
    assert_eq!(attack_requirements[0].creature, creature);
    assert!(attack_requirements[0].must_attack);
    assert_eq!(attack_requirements[0].cannot_attack, vec![player2]);
}
```

## Related Resources

- [Politics Testing](politics_testing.md): Details on testing political mechanics
- [Commander-Specific Cards](special_cards.md): Cards with political mechanics
- [Multiplayer Combat](../combat/multiplayer_combat.md): How combat works in multiplayer
- [Goad Implementation](special_cards.md#goad): More details on goad effects 