# Multiplayer Politics System

## Overview

The Multiplayer Politics System handles the unique social dynamics of Commander games with multiple players. It manages game mechanics like voting, deal-making, monarchy, and other political interactions that are core to the Commander experience. This system is especially important for games with many players (up to 13), where alliances and politics become central to gameplay.

## Core Components

### Politics State Resource

```rust
#[derive(Resource)]
pub struct PoliticsSystem {
    // Game-wide political state
    pub monarch: Option<Entity>,
    pub initiative_holder: Option<Entity>,
    pub goad_effects: HashMap<Entity, Vec<GoadEffect>>,
    pub vow_effects: HashMap<Entity, Vec<VowEffect>>,
    
    // Voting system
    pub active_vote: Option<Vote>,
    pub votes_cast: HashMap<Entity, VoteChoice>,
    pub vote_weights: HashMap<Entity, u32>,
    
    // Deal tracking
    pub pending_deals: Vec<Deal>,
    pub active_deals: Vec<Deal>,
    
    // Alliance tracking (for team play variants)
    pub alliances: HashMap<Entity, Vec<Entity>>,
    
    // Combat modification effects
    pub combat_restrictions: HashMap<Entity, Vec<CombatRestriction>>,
}

#[derive(Debug, Clone)]
pub struct Vote {
    pub id: uuid::Uuid,
    pub title: String,
    pub source: Entity,
    pub controller: Entity,
    pub choices: Vec<VoteChoice>,
    pub eligible_voters: Vec<Entity>,
    pub requires_all_players: bool,
    pub timer: Option<std::time::Duration>,
    pub created_at: std::time::Instant,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VoteChoice {
    pub id: usize,
    pub text: String,
    pub target: Option<Entity>,
}

#[derive(Debug, Clone)]
pub struct Deal {
    pub id: uuid::Uuid,
    pub proposer: Entity,
    pub target: Entity,
    pub terms: Vec<DealTerm>,
    pub duration: DealDuration,
    pub status: DealStatus,
    pub created_at: std::time::Instant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DealTerm {
    DoNotAttack(u32), // For N turns
    Truce(u32), // No harmful actions for N turns
    SharedDefense { against: Entity, turns: u32 },
    TargetOtherPlayer { target: Entity, turns: u32 },
    AllowAction { action_type: ActionType, turns: u32 },
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DealStatus {
    Proposed,
    Accepted,
    Rejected,
    Broken(Entity), // By which player
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DealDuration {
    Turns(u32),
    UntilEndOfGame,
    UntilPlayerEliminated(Entity),
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct GoadEffect {
    pub target: Entity, // Creature that is goaded
    pub source: Entity, // Player who goaded it
    pub duration: u32, // How many turns it lasts
    pub created_at: u32, // Turn number when created
}

#[derive(Debug, Clone)]
pub struct VowEffect {
    pub creature: Entity, // Creature with the vow
    pub protected_player: Entity, // Player it can't attack
    pub source: Entity, // Card that created the vow
    pub permanent: bool, // Whether it lasts for the whole game
}
```

### Political Events

```rust
#[derive(Event)]
pub struct VoteStartedEvent {
    pub vote: Vote,
}

#[derive(Event)]
pub struct VoteCastEvent {
    pub vote_id: uuid::Uuid,
    pub voter: Entity,
    pub choice: VoteChoice,
}

#[derive(Event)]
pub struct VoteResolvedEvent {
    pub vote_id: uuid::Uuid,
    pub winning_choice: VoteChoice,
    pub vote_counts: HashMap<VoteChoice, u32>,
}

#[derive(Event)]
pub struct DealProposedEvent {
    pub deal: Deal,
}

#[derive(Event)]
pub struct DealResponseEvent {
    pub deal_id: uuid::Uuid,
    pub responder: Entity,
    pub accepted: bool,
    pub counter_offer: Option<Vec<DealTerm>>,
}

#[derive(Event)]
pub struct DealBrokenEvent {
    pub deal_id: uuid::Uuid,
    pub violator: Entity,
    pub reason: String,
}

#[derive(Event)]
pub struct MonarchChangedEvent {
    pub previous: Option<Entity>,
    pub new: Entity,
    pub reason: MonarchChangeReason,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MonarchChangeReason {
    CardEffect(Entity),
    CombatDamage(Entity),
    PlayerEliminated,
    GameStart,
}
```

## Key Systems

### Monarchy System

```rust
fn monarchy_system(
    mut commands: Commands,
    mut politics: ResMut<PoliticsSystem>,
    mut monarchy_events: EventReader<MonarchChangedEvent>,
    mut damage_events: EventReader<CombatDamageEvent>,
    mut elimination_events: EventReader<PlayerEliminationEvent>,
    players: Query<Entity, With<CommanderPlayer>>,
    game_state: Res<CommanderGameState>,
) {
    // Handle explicit monarchy changes
    for event in monarchy_events.read() {
        // Update the monarchy
        politics.monarch = Some(event.new);
        
        // Grant card draw at end of turn
        commands.spawn(EndOfTurnTriggerEvent {
            player: event.new,
            effect: Box::new(MonarchCardDrawEffect { player: event.new }),
        });
    }
    
    // Check for monarchy changes due to combat damage
    for event in damage_events.read() {
        if event.is_combat_damage && 
           event.target == politics.monarch.unwrap_or(Entity::PLACEHOLDER) {
            
            let attacker_controller = get_controller(event.source, &players);
            
            if let Some(controller) = attacker_controller {
                commands.spawn(MonarchChangedEvent {
                    previous: politics.monarch,
                    new: controller,
                    reason: MonarchChangeReason::CombatDamage(event.source),
                });
            }
        }
    }
    
    // Check for monarchy changes due to player elimination
    for event in elimination_events.read() {
        if Some(event.player) == politics.monarch {
            // Monarch was eliminated, choose random new monarch
            let remaining_players: Vec<Entity> = players.iter()
                .filter(|&p| p != event.player)
                .collect();
                
            if !remaining_players.is_empty() {
                let new_monarch = remaining_players[
                    rand::thread_rng().gen_range(0..remaining_players.len())
                ];
                
                commands.spawn(MonarchChangedEvent {
                    previous: politics.monarch,
                    new: new_monarch,
                    reason: MonarchChangeReason::PlayerEliminated,
                });
            } else {
                // No players left
                politics.monarch = None;
            }
        }
    }
}
```

### Voting System

```rust
fn voting_system(
    mut commands: Commands,
    mut politics: ResMut<PoliticsSystem>,
    mut vote_started_events: EventReader<VoteStartedEvent>,
    mut vote_cast_events: EventReader<VoteCastEvent>,
    time: Res<Time>,
    players: Query<(Entity, &CommanderPlayer)>,
) {
    // Handle new votes
    for event in vote_started_events.read() {
        // Set up new vote
        politics.active_vote = Some(event.vote.clone());
        politics.votes_cast.clear();
        
        // Initialize vote weights
        politics.vote_weights.clear();
        for player in event.vote.eligible_voters.iter() {
            // Default weight is 1, but modify based on card effects
            let weight = get_vote_weight(*player, &players);
            politics.vote_weights.insert(*player, weight);
        }
    }
    
    // Process cast votes
    for event in vote_cast_events.read() {
        if let Some(vote) = &politics.active_vote {
            if vote.id == event.vote_id {
                // Record the vote
                politics.votes_cast.insert(event.voter, event.choice.clone());
                
                // Check if all votes are in
                if politics.votes_cast.len() == vote.eligible_voters.len() {
                    resolve_vote(&mut commands, &mut politics, &players);
                }
            }
        }
    }
    
    // Check for vote timeout
    if let Some(vote) = &politics.active_vote {
        if let Some(timer) = vote.timer {
            if time.elapsed() - vote.created_at.elapsed() > timer {
                // Time expired, resolve with votes cast so far
                resolve_vote(&mut commands, &mut politics, &players);
            }
        }
    }
}

fn resolve_vote(
    commands: &mut Commands,
    politics: &mut PoliticsSystem,
    players: &Query<(Entity, &CommanderPlayer)>,
) {
    if let Some(vote) = &politics.active_vote {
        // Count votes with weights
        let mut vote_counts: HashMap<VoteChoice, u32> = HashMap::new();
        
        for (voter, choice) in politics.votes_cast.iter() {
            let weight = politics.vote_weights.get(voter).unwrap_or(&1);
            *vote_counts.entry(choice.clone()).or_insert(0) += weight;
        }
        
        // Find the winning choice
        let winning_choice = vote_counts.iter()
            .max_by_key(|(_, count)| *count)
            .map(|(choice, _)| choice.clone())
            .unwrap_or_else(|| vote.choices[0].clone());
            
        // Emit resolution event
        commands.spawn(VoteResolvedEvent {
            vote_id: vote.id,
            winning_choice: winning_choice.clone(),
            vote_counts,
        });
        
        // Apply vote effects
        apply_vote_result(commands, vote, &winning_choice);
        
        // Clear active vote
        politics.active_vote = None;
    }
}

fn apply_vote_result(
    commands: &mut Commands,
    vote: &Vote,
    winning_choice: &VoteChoice,
) {
    // This would implement specific card effects based on the vote result
    // For example, Council's Judgment, Expropriate, etc.
    commands.spawn(VoteEffectEvent {
        vote_id: vote.id,
        winning_choice: winning_choice.clone(),
        source: vote.source,
        controller: vote.controller,
    });
}
```

### Deal-Making System

```rust
fn deal_system(
    mut commands: Commands,
    mut politics: ResMut<PoliticsSystem>,
    mut deal_proposed_events: EventReader<DealProposedEvent>,
    mut deal_response_events: EventReader<DealResponseEvent>,
    mut deal_broken_events: EventReader<DealBrokenEvent>,
    mut game_state: ResMut<CommanderGameState>,
    time: Res<Time>,
) {
    // Handle new deal proposals
    for event in deal_proposed_events.read() {
        politics.pending_deals.push(event.deal.clone());
    }
    
    // Handle deal responses
    for event in deal_response_events.read() {
        let deal_index = politics.pending_deals.iter()
            .position(|d| d.id == event.deal_id);
            
        if let Some(index) = deal_index {
            let mut deal = politics.pending_deals.remove(index);
            
            if event.accepted {
                // Activate the deal
                deal.status = DealStatus::Accepted;
                politics.active_deals.push(deal);
                
                // Apply deal effects if needed
                apply_deal_effects(&mut commands, &deal);
            } else if let Some(counter) = &event.counter_offer {
                // Create counter offer
                let counter_deal = Deal {
                    id: uuid::Uuid::new_v4(),
                    proposer: event.responder,
                    target: deal.proposer,
                    terms: counter.clone(),
                    duration: deal.duration.clone(),
                    status: DealStatus::Proposed,
                    created_at: time.elapsed(),
                };
                
                politics.pending_deals.push(counter_deal.clone());
                
                // Notify about counter offer
                commands.spawn(DealProposedEvent {
                    deal: counter_deal,
                });
            } else {
                // Deal rejected, nothing to do
            }
        }
    }
    
    // Handle broken deals
    for event in deal_broken_events.read() {
        let deal_index = politics.active_deals.iter()
            .position(|d| d.id == event.deal_id);
            
        if let Some(index) = deal_index {
            let mut deal = politics.active_deals.remove(index);
            deal.status = DealStatus::Broken(event.violator);
            
            // Remove deal effects
            remove_deal_effects(&mut commands, &deal);
            
            // Notify players
            commands.spawn(DealEffectEvent {
                deal_id: deal.id,
                effect_type: DealEffectType::Broken,
                violator: Some(event.violator),
                reason: event.reason.clone(),
            });
        }
    }
    
    // Check for expired deals
    let current_turn = game_state.turn_number;
    let mut expired_deals = Vec::new();
    
    for (i, deal) in politics.active_deals.iter().enumerate() {
        let is_expired = match deal.duration {
            DealDuration::Turns(turns) => {
                let created_turn = deal.created_at.elapsed().as_secs() / 
                                  game_state.turn_duration.as_secs();
                (current_turn as u64 - created_turn) >= turns as u64
            },
            DealDuration::UntilPlayerEliminated(player) => {
                game_state.eliminated_players.contains(&player)
            },
            DealDuration::UntilEndOfGame => false,
            DealDuration::Custom(_) => false, // Would require custom logic
        };
        
        if is_expired {
            expired_deals.push(i);
        }
    }
    
    // Process expired deals in reverse order to avoid index issues
    for &index in expired_deals.iter().rev() {
        let mut deal = politics.active_deals.remove(index);
        deal.status = DealStatus::Expired;
        
        // Remove deal effects
        remove_deal_effects(&mut commands, &deal);
        
        // Notify players
        commands.spawn(DealEffectEvent {
            deal_id: deal.id,
            effect_type: DealEffectType::Expired,
            violator: None,
            reason: "Deal duration expired".to_string(),
        });
    }
}

fn apply_deal_effects(
    commands: &mut Commands,
    deal: &Deal,
) {
    for term in &deal.terms {
        match term {
            DealTerm::DoNotAttack(turns) => {
                commands.spawn(CombatRestrictionEvent {
                    source: deal.proposer,
                    target: deal.target,
                    restriction: CombatRestriction::CannotAttack(deal.proposer),
                    duration: *turns,
                    source_type: RestrictionSource::Deal(deal.id),
                });
            },
            DealTerm::TargetOtherPlayer { target, turns } => {
                commands.spawn(CombatRestrictionEvent {
                    source: deal.proposer,
                    target: *target,
                    restriction: CombatRestriction::MustAttackIfAble,
                    duration: *turns,
                    source_type: RestrictionSource::Deal(deal.id),
                });
            },
            // Handle other deal terms
            _ => {}
        }
    }
}
```

### Goad System

```rust
fn goad_system(
    mut commands: Commands,
    mut politics: ResMut<PoliticsSystem>,
    mut goad_events: EventReader<GoadEvent>,
    game_state: Res<CommanderGameState>,
    combat_state: Res<CombatState>,
) {
    // Process new goad effects
    for event in goad_events.read() {
        // Add goad effect
        let goad_effect = GoadEffect {
            target: event.target,
            source: event.source,
            duration: event.duration,
            created_at: game_state.turn_number,
        };
        
        politics.goad_effects
            .entry(event.target)
            .or_default()
            .push(goad_effect);
            
        // Apply immediate combat restriction
        commands.spawn(CombatRestrictionEvent {
            source: event.source,
            target: event.target,
            restriction: CombatRestriction::MustAttackOtherPlayer(event.source),
            duration: event.duration,
            source_type: RestrictionSource::Goad,
        });
    }
    
    // Check for expired goad effects
    let current_turn = game_state.turn_number;
    
    for (creature, effects) in politics.goad_effects.iter_mut() {
        // Filter out expired effects
        effects.retain(|effect| {
            (effect.created_at + effect.duration) >= current_turn
        });
        
        // If all effects expired, remove combat restrictions
        if effects.is_empty() {
            commands.spawn(CombatRestrictionRemovalEvent {
                target: *creature,
                restriction_type: RestrictionSource::Goad,
            });
        }
    }
    
    // Remove empty entries
    politics.goad_effects.retain(|_, effects| !effects.is_empty());
}
```

## Multiplayer Decision Systems

### Group Hug and Help Effects

```rust
fn group_effect_system(
    mut commands: Commands,
    mut group_effect_events: EventReader<GroupEffectEvent>,
    players: Query<Entity, With<CommanderPlayer>>,
) {
    for event in group_effect_events.read() {
        match event.effect_type {
            GroupEffectType::AllDraw(amount) => {
                // Generate draw events for all players
                for player in players.iter() {
                    commands.spawn(DrawCardEvent {
                        player,
                        amount,
                    });
                }
            },
            GroupEffectType::AllGainLife(amount) => {
                // Generate life gain events for all players
                for player in players.iter() {
                    commands.spawn(LifeChangeEvent {
                        player,
                        amount: amount as i32,
                        source: event.source,
                        source_type: LifeChangeSource::CardEffect,
                    });
                }
            },
            GroupEffectType::AllPutLandFromLibrary => {
                // Generate land search events for all players
                for player in players.iter() {
                    commands.spawn(SearchLibraryEvent {
                        player,
                        filter: CardFilter::Type(CardTypes::LAND),
                        destination: Zone::Battlefield,
                        revealed: true,
                        shuffle: true,
                    });
                }
            },
            // Handle other group effects
            _ => {}
        }
    }
}
```

### Player Targeting Analysis

```rust
fn player_threat_analysis_system(
    politics: Res<PoliticsSystem>,
    players: Query<(Entity, &CommanderPlayer)>,
    permanents: Query<(Entity, &Permanent)>,
    mut threat_events: EventWriter<ThreatAssessmentEvent>,
) {
    // This system would analyze the game state to determine who is the biggest threat
    // and could provide suggestions for political decisions
    
    let mut threat_scores: HashMap<Entity, u32> = HashMap::new();
    
    // Calculate threat scores based on various factors
    for (entity, player) in players.iter() {
        let mut score = 0;
        
        // Life total factor - lower is less threatening
        score += player.life as u32 / 5;
        
        // Board presence factor
        let board_presence = permanents.iter()
            .filter(|(_, perm)| perm.controller == entity)
            .count();
        score += board_presence as u32;
        
        // Commander damage factor
        for (_, damage) in player.commander_damage_received.iter() {
            // More damage received = less threatening
            score = score.saturating_sub(*damage / 3);
        }
        
        // Monarch bonus
        if Some(entity) == politics.monarch {
            score += 10;
        }
        
        // Store the score
        threat_scores.insert(entity, score);
    }
    
    // Emit threat assessment event
    threat_events.send(ThreatAssessmentEvent {
        scores: threat_scores,
        assessment_time: std::time::Instant::now(),
    });
}
```

## Integration with Combat System

```rust
fn apply_political_combat_effects(
    politics: Res<PoliticsSystem>,
    mut combat_state: ResMut<CombatState>,
    creatures: Query<(Entity, &Permanent, &CreatureCard)>,
    players: Query<Entity, With<CommanderPlayer>>,
) {
    // Apply goad effects to combat
    for (creature, effects) in politics.goad_effects.iter() {
        if !effects.is_empty() {
            // Creature must attack if able
            combat_state.must_attack.insert(*creature, 
                players.iter().filter(|&p| {
                    // Filter out players that goaded this creature
                    !effects.iter().any(|e| e.source == p)
                }).collect()
            );
            
            // Creature cannot attack players who goaded it
            let cannot_attack: Vec<Entity> = effects.iter()
                .map(|e| e.source)
                .collect();
                
            combat_state.cannot_attack.insert(*creature, cannot_attack);
        }
    }
    
    // Apply vow effects to combat
    for (creature, vows) in politics.vow_effects.iter() {
        let protected_players: Vec<Entity> = vows.iter()
            .map(|v| v.protected_player)
            .collect();
            
        combat_state.cannot_attack.insert(*creature, protected_players);
    }
    
    // Apply other political combat restrictions
    for (target, restrictions) in politics.combat_restrictions.iter() {
        for restriction in restrictions {
            match restriction {
                CombatRestriction::CannotAttack(player) => {
                    combat_state.cannot_attack
                        .entry(*target)
                        .or_default()
                        .push(*player);
                },
                CombatRestriction::MustAttackIfAble => {
                    // Mark creature as having to attack if able
                    combat_state.must_attack_if_able.insert(*target);
                },
                CombatRestriction::MustAttackOtherPlayer(except_player) => {
                    // Must attack any player other than the specified one
                    let other_players: Vec<Entity> = players.iter()
                        .filter(|&p| p != *except_player)
                        .collect();
                        
                    combat_state.must_attack
                        .entry(*target)
                        .or_default()
                        .extend(other_players);
                        
                    combat_state.cannot_attack
                        .entry(*target)
                        .or_default()
                        .push(*except_player);
                },
                // Handle other restrictions
                _ => {}
            }
        }
    }
}
```

## UI Integration

```rust
fn politics_ui_events_system(
    mut commands: Commands,
    politics: Res<PoliticsSystem>,
    mut deal_ui_events: EventReader<DealUiEvent>,
    mut vote_ui_events: EventReader<VoteUiEvent>,
) {
    // Handle deal UI events
    for event in deal_ui_events.read() {
        match event {
            DealUiEvent::ProposeDeal { proposer, target, terms, duration } => {
                let deal = Deal {
                    id: uuid::Uuid::new_v4(),
                    proposer: *proposer,
                    target: *target,
                    terms: terms.clone(),
                    duration: duration.clone(),
                    status: DealStatus::Proposed,
                    created_at: std::time::Instant::now(),
                };
                
                commands.spawn(DealProposedEvent { deal });
            },
            DealUiEvent::RespondToDeal { deal_id, responder, accept, counter_offer } => {
                commands.spawn(DealResponseEvent {
                    deal_id: *deal_id,
                    responder: *responder,
                    accepted: *accept,
                    counter_offer: counter_offer.clone(),
                });
            },
            // Other UI events
        }
    }
    
    // Handle vote UI events
    for event in vote_ui_events.read() {
        match event {
            VoteUiEvent::CastVote { vote_id, voter, choice } => {
                commands.spawn(VoteCastEvent {
                    vote_id: *vote_id,
                    voter: *voter,
                    choice: choice.clone(),
                });
            },
            // Other vote UI events
        }
    }
}
```

## Integration Points

- **Game State Module**: Political effects can modify game state
- **Player Module**: Tracks player relationships and alliances
- **Combat System**: Applies political restrictions to combat
- **Turn Structure**: Handles political effects with timing restrictions
- **UI System**: Presents political choices and deals to players

## Testing Strategy

1. **Unit Tests**:
   - Verify voting mechanics and outcomes
   - Test deal-making logic and enforcement
   - Validate monarchy transitions
   
2. **Integration Tests**:
   - Test political effects on combat
   - Verify interaction between politics and state-based actions
   - Test multi-player political scenarios
   - Validate handling of complex politics with many players

## Performance Considerations

For Commander games with many players:

- Batch processing of political effects
- Efficient tracking of deals and restrictions
- Distributed voting process for many players
- Smart update system that only refreshes changed political state
- Client-server architecture for multiplayer politics 

## Multiplayer Politics Edge Cases

### Joint Victory and Optional Draw Conditions

Commander introduces the possibility of joint victory or draws based on player agreement:

```rust
fn handle_joint_victory_conditions(
    mut commands: Commands,
    mut politics: ResMut<PoliticsSystem>,
    players: Query<(Entity, &CommanderPlayer)>,
    mut victory_events: EventReader<VictoryProposalEvent>,
    mut victory_response_events: EventReader<VictoryResponseEvent>,
) {
    // Handle proposed joint victories
    for event in victory_events.read() {
        match event.proposal_type {
            VictoryProposalType::JointVictory { players: proposed_winners } => {
                // Set up the proposal
                politics.active_victory_proposal = Some(VictoryProposal {
                    id: event.id,
                    proposer: event.proposer,
                    proposal_type: event.proposal_type.clone(),
                    responses: HashMap::new(),
                    created_at: time.elapsed(),
                });
                
                // Require unanimous agreement from all players
                for (player, _) in players.iter() {
                    // Skip eliminated players
                    if !proposed_winners.contains(&player) {
                        politics.active_victory_proposal.as_mut().unwrap()
                            .responses.insert(player, VictoryResponseType::Pending);
                    }
                }
            },
            VictoryProposalType::Draw => {
                // For a draw, ALL players must agree
                politics.active_victory_proposal = Some(VictoryProposal {
                    id: event.id,
                    proposer: event.proposer,
                    proposal_type: event.proposal_type.clone(),
                    responses: HashMap::new(),
                    created_at: time.elapsed(),
                });
                
                // Initialize all players as pending
                for (player, _) in players.iter() {
                    politics.active_victory_proposal.as_mut().unwrap()
                        .responses.insert(player, VictoryResponseType::Pending);
                }
            },
            // Other proposal types...
        }
    }
    
    // Process responses to victory proposals
    for event in victory_response_events.read() {
        if let Some(proposal) = &mut politics.active_victory_proposal {
            if proposal.id == event.proposal_id {
                // Record the response
                proposal.responses.insert(event.responder, event.response.clone());
                
                // Check if we have all responses
                let all_responded = proposal.responses.values()
                    .all(|r| *r != VictoryResponseType::Pending);
                    
                if all_responded {
                    // For joint victory, all non-winners must accept
                    // For draw, everyone must accept
                    let is_accepted = proposal.responses.values()
                        .all(|r| *r == VictoryResponseType::Accept);
                    
                    if is_accepted {
                        // Apply the victory condition
                        match &proposal.proposal_type {
                            VictoryProposalType::JointVictory { players } => {
                                for &player in players {
                                    commands.spawn(PlayerVictoryEvent {
                                        player,
                                        victory_type: VictoryType::Joint,
                                        shared_with: players.clone(),
                                    });
                                }
                            },
                            VictoryProposalType::Draw => {
                                commands.spawn(GameDrawEvent {
                                    reason: DrawReason::MutualAgreement,
                                });
                            },
                            // Other types...
                        }
                    } else {
                        // Proposal rejected
                        commands.spawn(VictoryProposalRejectedEvent {
                            proposal_id: proposal.id,
                            reason: "Not all players accepted".to_string(),
                        });
                    }
                    
                    // Clear the active proposal
                    politics.active_victory_proposal = None;
                }
            }
        }
    }
}
```

### Kingmaking and Anti-Leader Coalitions

In Commander multiplayer, players often form temporary alliances against the strongest player:

```rust
fn handle_dynamic_coalitions(
    mut commands: Commands,
    mut politics: ResMut<PoliticsSystem>,
    players: Query<(Entity, &CommanderPlayer)>,
    game_state: Res<CommanderGameState>,
) {
    // Periodically analyze game state to identify potential leaders
    if game_state.turn_number % 3 == 0 { // Every 3 turns
        // Calculate threat assessment for each player
        let mut threat_levels = HashMap::new();
        
        for (player, player_data) in players.iter() {
            // Basic threat metrics
            let mut threat_score = 0;
            
            // Life total relative to starting life
            let life_percentage = player_data.life as f32 / CommanderRules::STARTING_LIFE as f32;
            threat_score += (life_percentage * 10.0) as i32;
            
            // Number of permanents
            let permanent_count = game_state.get_permanent_count(player);
            threat_score += permanent_count as i32;
            
            // Commander on battlefield
            if player_data.commander_on_battlefield {
                threat_score += 5;
                
                // Commander with high power
                if let Some(power) = player_data.commander_power {
                    threat_score += power as i32;
                }
            }
            
            // Hand size
            threat_score += player_data.hand_size as i32 * 2;
            
            // Store threat assessment
            threat_levels.insert(player, threat_score);
        }
        
        // Identify the leader(s)
        if let Some(max_threat) = threat_levels.values().max() {
            let leaders: Vec<Entity> = threat_levels.iter()
                .filter(|(_, &threat)| threat == *max_threat)
                .map(|(&player, _)| player)
                .collect();
                
            // Update politics system
            politics.perceived_leaders = leaders.clone();
            
            // Notify all players about the threat assessment
            commands.spawn(ThreatAssessmentEvent {
                threat_levels: threat_levels.clone(),
                leaders: leaders,
            });
        }
    }
}
```

### Temporary Alliances with Rules Integration

Handle temporary alliances that modify game rules:

```rust
fn apply_alliance_effects(
    mut commands: Commands,
    politics: Res<PoliticsSystem>,
    mut game_rules: ResMut<GameRules>,
) {
    // Apply rule modifications based on active alliances
    for alliance in politics.active_alliances.iter() {
        match &alliance.effect_type {
            AllianceEffectType::SharedDamageRedirection { rate } => {
                // Set up damage redirection system for allied players
                for &player in &alliance.members {
                    game_rules.damage_redirection.insert(
                        player,
                        DamageRedirection {
                            source_type: RedirectionSource::Alliance(alliance.id),
                            share_targets: alliance.members.clone(),
                            share_rate: *rate,
                            duration: alliance.duration,
                        }
                    );
                }
            },
            AllianceEffectType::MutualProtection => {
                // Set up protection effects
                for &player in &alliance.members {
                    for &ally in &alliance.members {
                        if player != ally {
                            game_rules.protection_effects.insert(
                                (player, ally),
                                ProtectionEffect {
                                    protected_player: ally,
                                    protector: player,
                                    source_type: ProtectionSource::Alliance(alliance.id),
                                    duration: alliance.duration,
                                }
                            );
                        }
                    }
                }
            },
            // Other alliance effect types...
        }
    }
}
```

### Edge Case: Simultaneous Decision Making with Timeout

For efficiency in large multiplayer games, implement a timeout system for decisions:

```rust
fn handle_timed_multiplayer_decisions(
    mut commands: Commands,
    mut politics: ResMut<PoliticsSystem>,
    mut decision_events: EventReader<GroupDecisionEvent>,
    mut decision_responses: EventReader<PlayerDecisionResponse>,
    time: Res<Time>,
) {
    // Process new decision events
    for event in decision_events.read() {
        let decision = GroupDecision {
            id: event.id,
            decision_type: event.decision_type.clone(),
            participants: event.participants.clone(),
            responses: HashMap::new(),
            timeout: event.timeout,
            created_at: time.elapsed(),
        };
        
        politics.active_decisions.push(decision);
    }
    
    // Process incoming responses
    for event in decision_responses.read() {
        if let Some(decision_index) = politics.active_decisions.iter()
            .position(|d| d.id == event.decision_id) {
            
            let decision = &mut politics.active_decisions[decision_index];
            decision.responses.insert(event.player, event.response.clone());
            
            // Check if all responses are in
            let all_responded = decision.participants.iter()
                .all(|p| decision.responses.contains_key(p));
                
            if all_responded {
                resolve_group_decision(&mut commands, decision_index, &mut politics);
            }
        }
    }
    
    // Check for timeouts
    let current_time = time.elapsed();
    let mut decisions_to_resolve = Vec::new();
    
    for (i, decision) in politics.active_decisions.iter().enumerate() {
        if let Some(timeout) = decision.timeout {
            if current_time - decision.created_at > timeout {
                // Add defaulting behavior for non-responders
                decisions_to_resolve.push(i);
            }
        }
    }
    
    // Resolve timed-out decisions in reverse order
    for index in decisions_to_resolve.into_iter().rev() {
        resolve_group_decision(&mut commands, index, &mut politics);
    }
}

fn resolve_group_decision(
    commands: &mut Commands,
    decision_index: usize,
    politics: &mut PoliticsSystem,
) {
    // Take ownership of the decision
    let decision = politics.active_decisions.remove(decision_index);
    
    // For any player who didn't respond, apply the default action
    let mut final_responses = decision.responses.clone();
    for &participant in &decision.participants {
        if !final_responses.contains_key(&participant) {
            final_responses.insert(participant, DecisionResponse::Default);
        }
    }
    
    // Process based on decision type
    match decision.decision_type {
        GroupDecisionType::AttackDirection => {
            // Process coordinated attack planning
            commands.spawn(GroupDecisionResolvedEvent {
                decision_id: decision.id,
                decision_type: decision.decision_type,
                responses: final_responses,
            });
        },
        GroupDecisionType::ResourceSharing { resource_type } => {
            // Process resource sharing
            // (e.g., mana, cards, permanent control)
            commands.spawn(ResourceSharingResolvedEvent {
                decision_id: decision.id,
                resource_type,
                shares: final_responses.into_iter()
                    .map(|(player, resp)| {
                        let amount = match resp {
                            DecisionResponse::Number(n) => n,
                            DecisionResponse::Default => 0,
                            _ => 0,
                        };
                        (player, amount)
                    })
                    .collect(),
            });
        },
        // Other decision types...
    }
}
```

### Edge Case: Diplomatic Immunity and Targeting Restrictions

Some political mechanics create targeting restrictions:

```rust
fn apply_diplomatic_immunity(
    mut commands: Commands,
    politics: Res<PoliticsSystem>,
    mut game_state: ResMut<CommanderGameState>,
) {
    // Clear current diplomatic immunities
    game_state.targeting_restrictions.diplomatic_immunity.clear();
    
    // Apply current diplomatic agreements
    for deal in politics.active_deals.iter() {
        match &deal.deal_type {
            DealType::NonAggressionPact { players, duration } => {
                for &player1 in players {
                    for &player2 in players {
                        if player1 != player2 {
                            // Create mutual targeting restriction
                            game_state.targeting_restrictions.diplomatic_immunity
                                .entry(player1)
                                .or_default()
                                .push(TargetingRestriction {
                                    source_player: player2,
                                    restriction_type: TargetingRestrictionType::CannotTargetPlayer,
                                    source: RestrictionSource::Deal(deal.id),
                                });
                        }
                    }
                }
            },
            DealType::ProtectionAgreement { protector, protected, duration } => {
                // One player cannot be targeted by the other
                game_state.targeting_restrictions.diplomatic_immunity
                    .entry(*protected)
                    .or_default()
                    .push(TargetingRestriction {
                        source_player: *protector,
                        restriction_type: TargetingRestrictionType::CannotTargetPlayer,
                        source: RestrictionSource::Deal(deal.id),
                    });
            },
            // Other deal types...
        }
    }
}
```

### Edge Case: Simultaneous Secret Selection

Handle scenarios where players make simultaneous secret choices:

```rust
fn handle_simultaneous_secret_choices(
    mut commands: Commands,
    mut politics: ResMut<PoliticsSystem>,
    mut secret_choice_events: EventReader<SecretChoiceEvent>,
    mut secret_choice_submissions: EventReader<SecretChoiceSubmission>,
    time: Res<Time>,
) {
    // Handle new secret choice scenarios
    for event in secret_choice_events.read() {
        politics.active_secret_choices.push(SecretChoiceScenario {
            id: event.id,
            choice_type: event.choice_type.clone(),
            eligible_players: event.eligible_players.clone(),
            submissions: HashMap::new(),
            timeout: event.timeout,
            created_at: time.elapsed(),
        });
    }
    
    // Process submissions
    for event in secret_choice_submissions.read() {
        if let Some(choice) = politics.active_secret_choices.iter_mut()
            .find(|c| c.id == event.choice_id) {
            
            // Record submission
            choice.submissions.insert(event.player, event.choice.clone());
            
            // Check if all players have submitted
            if choice.submissions.len() == choice.eligible_players.len() {
                resolve_secret_choice(
                    &mut commands,
                    &mut politics.active_secret_choices,
                    choice.id
                );
            }
        }
    }
    
    // Check for timeouts
    let current_time = time.elapsed();
    let choice_ids: Vec<Uuid> = politics.active_secret_choices.iter()
        .filter(|choice| {
            if let Some(timeout) = choice.timeout {
                current_time - choice.created_at > timeout
            } else {
                false
            }
        })
        .map(|choice| choice.id)
        .collect();
    
    // Resolve timed-out choices
    for id in choice_ids {
        resolve_secret_choice(
            &mut commands,
            &mut politics.active_secret_choices,
            id
        );
    }
}

fn resolve_secret_choice(
    commands: &mut Commands,
    choices: &mut Vec<SecretChoiceScenario>,
    choice_id: Uuid,
) {
    // Find and remove the choice
    let choice_index = choices.iter().position(|c| c.id == choice_id).unwrap();
    let choice = choices.remove(choice_index);
    
    // Handle default choices for any players who didn't submit
    let mut final_choices = choice.submissions;
    for &player in &choice.eligible_players {
        if !final_choices.contains_key(&player) {
            match choice.choice_type {
                SecretChoiceType::TargetPlayer => {
                    // Default to no choice
                    final_choices.insert(player, PlayerChoice::NoSelection);
                },
                SecretChoiceType::TargetCard { card_list } => {
                    // Default to first card if possible
                    let default_choice = if card_list.is_empty() {
                        PlayerChoice::NoSelection
                    } else {
                        PlayerChoice::Card(card_list[0])
                    };
                    final_choices.insert(player, default_choice);
                },
                // Other types...
            }
        }
    }
    
    // Reveal all choices simultaneously
    commands.spawn(SecretChoicesRevealedEvent {
        choice_id: choice.id,
        choice_type: choice.choice_type,
        choices: final_choices,
    });
}
```

### Edge Case: Vote Trading and Enforcement

Handle complex voting scenarios where players trade votes:

```rust
fn handle_vote_trading(
    mut commands: Commands,
    mut politics: ResMut<PoliticsSystem>,
    mut vote_trade_events: EventReader<VoteTradeEvent>,
    mut vote_events: EventReader<VoteEvent>,
    mut vote_cast_events: EventReader<VoteCastEvent>,
) {
    // Process vote trade proposals
    for event in vote_trade_events.read() {
        match event.trade_type {
            VoteTradeType::Proposal { from, to, offer, request } => {
                // Record the proposal
                politics.vote_trade_proposals.push(VoteTrade {
                    id: event.id,
                    from,
                    to,
                    offer,
                    request,
                    status: VoteTradeStatus::Pending,
                });
            },
            VoteTradeType::Accept { id } => {
                // Find and update the proposal
                if let Some(trade) = politics.vote_trade_proposals.iter_mut()
                    .find(|t| t.id == id) {
                    
                    trade.status = VoteTradeStatus::Accepted;
                    
                    // Add enforcement tracking
                    politics.vote_trade_enforcement.push(VoteTradeEnforcement {
                        trade_id: id,
                        offerer_voted: false,
                        requester_voted: false,
                        offerer_honored: false,
                        requester_honored: false,
                    });
                }
            },
            VoteTradeType::Reject { id } => {
                // Remove rejected proposals
                politics.vote_trade_proposals.retain(|t| t.id != id);
            },
        }
    }
    
    // Track active votes
    for event in vote_events.read() {
        politics.active_votes.push(event.vote_id);
    }
    
    // Monitor vote casting to enforce trades
    for event in vote_cast_events.read() {
        // Check if this vote is part of a trade
        for enforcement in politics.vote_trade_enforcement.iter_mut() {
            let trade = politics.vote_trade_proposals.iter()
                .find(|t| t.id == enforcement.trade_id)
                .unwrap();
            
            // Check if this is the offerer voting
            if event.player == trade.from {
                enforcement.offerer_voted = true;
                
                // Check if they honored the trade
                if let VoteOffer::SpecificChoice { vote_id, choice } = &trade.offer {
                    if event.vote_id == *vote_id && event.choice == *choice {
                        enforcement.offerer_honored = true;
                    }
                }
            }
            
            // Check if this is the requester voting
            if event.player == trade.to {
                enforcement.requester_voted = true;
                
                // Check if they honored the trade
                if let VoteRequest::SpecificChoice { vote_id, choice } = &trade.request {
                    if event.vote_id == *vote_id && event.choice == *choice {
                        enforcement.requester_honored = true;
                    }
                }
            }
        }
    }
    
    // Process completed votes
    let completed_votes: Vec<Uuid> = politics.active_votes.clone();
    
    for vote_id in completed_votes {
        // For each completed vote, check trade enforcement
        for enforcement in politics.vote_trade_enforcement.iter() {
            let trade = politics.vote_trade_proposals.iter()
                .find(|t| t.id == enforcement.trade_id)
                .unwrap();
            
            // Check if this trade was relevant to the completed vote
            let trade_involves_vote = match &trade.offer {
                VoteOffer::SpecificChoice { vote_id: offer_vote_id, .. } => *offer_vote_id == vote_id,
                _ => false,
            } || match &trade.request {
                VoteRequest::SpecificChoice { vote_id: request_vote_id, .. } => *request_vote_id == vote_id,
                _ => false,
            };
            
            if trade_involves_vote {
                // Check if trade was violated
                let violated = enforcement.offerer_voted && !enforcement.offerer_honored ||
                               enforcement.requester_voted && !enforcement.requester_honored;
                
                if violated {
                    commands.spawn(VoteTradeViolatedEvent {
                        trade_id: enforcement.trade_id,
                        violator: if enforcement.offerer_voted && !enforcement.offerer_honored {
                            trade.from
                        } else {
                            trade.to
                        },
                    });
                }
            }
        }
    }
}
```

### Edge Case: Concession Timing and Inheritance

Handle the complex case of player concession during critical game moments:

```rust
fn handle_player_concession(
    mut commands: Commands,
    mut game_state: ResMut<CommanderGameState>,
    mut concession_events: EventReader<PlayerConcessionEvent>,
    mut politics: ResMut<PoliticsSystem>,
    stack: Res<Stack>,
    combat_state: Option<Res<CombatState>>,
) {
    for event in concession_events.read() {
        let conceding_player = event.player;
        
        // Handle concession during different game states
        let current_phase = game_state.current_phase;
        let stack_is_resolving = stack.currently_resolving.is_some();
        let in_combat = combat_state.is_some() && combat_state.unwrap().in_combat;
        
        // Apply special handling for concessions during critical moments
        if stack_is_resolving || in_combat {
            // Complex concession handling:
            
            // 1. Record all targets controlled by conceding player
            let targets_controlled = game_state.get_permanents_controlled_by(conceding_player);
            
            // 2. For each spell/ability on the stack targeting those permanents
            let affected_stack_items = stack.get_items_targeting_permanents(&targets_controlled);
            
            for stack_item in affected_stack_items {
                // Determine if spell fizzles or needs redirection
                commands.spawn(StackItemConcessionAffectedEvent {
                    stack_item,
                    conceding_player,
                    affected_permanents: targets_controlled.clone(),
                });
            }
            
            // 3. For combat, check if player is being attacked
            if let Some(combat) = combat_state {
                let attacks_against_player = combat
                    .attackers
                    .iter()
                    .filter(|(_, &defender)| defender == conceding_player)
                    .map(|(&attacker, _)| attacker)
                    .collect::<Vec<_>>();
                    
                if !attacks_against_player.is_empty() {
                    commands.spawn(AttacksConcessionAffectedEvent {
                        attackers: attacks_against_player,
                        defender: conceding_player,
                    });
                }
            }
        }
        
        // Handle political effects
        
        // 1. Terminate all deals involving the player
        for deal in politics.active_deals.iter_mut() {
            if deal.involves_player(conceding_player) {
                deal.status = DealStatus::Terminated(TerminationReason::PlayerConceded);
            }
        }
        
        // 2. Resolve all votes immediately if they involve the player
        if politics.active_vote.as_ref().map_or(false, |v| v.involves_player(conceding_player)) {
            // Force immediate vote resolution
            let mut vote = politics.active_vote.take().unwrap();
            vote.eligible_voters.retain(|&p| p != conceding_player);
            vote.votes_cast.remove(&conceding_player);
            
            commands.spawn(VoteResolvedEvent {
                vote_id: vote.id,
                reason: VoteResolutionReason::PlayerConceded(conceding_player),
                vote_results: vote.votes_cast,
            });
        }
        
        // 3. Transfer control of "owned" political effects
        let effects_to_transfer = politics.get_effects_controlled_by(conceding_player);
        
        if !effects_to_transfer.is_empty() {
            commands.spawn(PoliticalEffectsTransferEvent {
                from_player: conceding_player,
                effects: effects_to_transfer,
                transfer_method: TransferMethod::RandomDistribution,
            });
        }
        
        // Finally, eliminate the player
        commands.spawn(PlayerEliminatedEvent {
            player: conceding_player,
            reason: EliminationReason::Concede,
        });
    }
}
```

### Edge Case: Hidden Information Politics

Handle the complex case of shared or revealed hidden information:

```rust
fn handle_hidden_information_sharing(
    mut commands: Commands,
    mut politics: ResMut<PoliticsSystem>,
    mut info_share_events: EventReader<HiddenInfoShareEvent>,
    mut revealed_cards_events: EventReader<RevealedCardsEvent>,
    game_state: Res<CommanderGameState>,
) {
    // Process information sharing events
    for event in info_share_events.read() {
        match event.info_type {
            HiddenInfoType::HandCard { card, owner, shown_to } => {
                // Record that this card has been revealed to specific players
                politics.revealed_information.hand_cards
                    .entry(owner)
                    .or_default()
                    .entry(card)
                    .or_default()
                    .extend(shown_to.iter());
            },
            HiddenInfoType::FaceDownCard { card, controller, shown_to } => {
                // Record face-down card information
                politics.revealed_information.face_down_cards
                    .entry(controller)
                    .or_default()
                    .entry(card)
                    .or_default()
                    .extend(shown_to.iter());
            },
            HiddenInfoType::LibraryTopCard { cards, owner, shown_to } => {
                // Record library top card information
                politics.revealed_information.library_cards
                    .entry(owner)
                    .or_default()
                    .push(LibraryRevealInfo {
                        cards,
                        revealed_to: shown_to.clone(),
                        timestamp: game_state.turn_number,
                    });
            },
            // Other hidden info types...
        }
    }
    
    // Process game-mandated reveals
    for event in revealed_cards_events.read() {
        // Update knowledge tracking based on public reveals
        match event.reveal_type {
            RevealType::ToAll { cards, owner } => {
                // Everyone sees these cards
                let all_players: Vec<Entity> = game_state.get_active_players();
                
                match event.zone {
                    Zone::Hand => {
                        for &card in &cards {
                            politics.revealed_information.hand_cards
                                .entry(owner)
                                .or_default()
                                .entry(card)
                                .or_default()
                                .extend(all_players.iter());
                        }
                    },
                    Zone::Library => {
                        politics.revealed_information.library_cards
                            .entry(owner)
                            .or_default()
                            .push(LibraryRevealInfo {
                                cards,
                                revealed_to: all_players,
                                timestamp: game_state.turn_number,
                            });
                    },
                    // Other zones...
                    _ => {}
                }
            },
            RevealType::ToSpecific { cards, owner, viewers } => {
                // Same logic but for specific viewers
                match event.zone {
                    Zone::Hand => {
                        for &card in &cards {
                            politics.revealed_information.hand_cards
                                .entry(owner)
                                .or_default()
                                .entry(card)
                                .or_default()
                                .extend(viewers.iter());
                        }
                    },
                    // Other zones...
                    _ => {}
                }
            },
        }
    }
    
    // Clean up outdated information
    clean_up_outdated_information(&mut politics, &game_state);
}

fn clean_up_outdated_information(
    politics: &mut PoliticsSystem,
    game_state: &CommanderGameState,
) {
    // Remove information about cards that have moved zones
    
    // 1. Clean hand cards
    for (owner, cards) in politics.revealed_information.hand_cards.iter_mut() {
        let current_hand = game_state.get_hand(*owner);
        cards.retain(|&card, _| current_hand.contains(&card));
    }
    
    // 2. Clean face-down cards
    for (controller, cards) in politics.revealed_information.face_down_cards.iter_mut() {
        let current_face_down = game_state.get_face_down_cards(*controller);
        cards.retain(|&card, _| current_face_down.contains(&card));
    }
    
    // 3. Clean library cards (only keep recent reveals)
    for (owner, reveals) in politics.revealed_information.library_cards.iter_mut() {
        reveals.retain(|info| game_state.turn_number - info.timestamp <= 1);
    }
}
```

## Advanced Multiplayer Politics Testing Framework

Complex political scenarios require thorough testing:

```rust
#[test]
fn test_multiplayer_politics_edge_cases() {
    let mut app = App::new();
    app.add_plugins(CommanderPoliticsTestPlugin);
    
    // Test cases for complex political scenarios
    
    // 1. Multi-player vote trading with enforcement
    // 2. Hidden information sharing affecting game decisions
    // 3. Diplomatic immunity during critical game moments
    // 4. Player concession during stack resolution
    // 5. Kingmaking scenarios and prevention
    // 6. Joint victory proposals with partial acceptance
    // 7. Secret simultaneous choices with conflicting outcomes
    // 8. Dynamic coalition formation and dissolution
    // 9. Deal breaking with consequences
    // 10. Politics during multiplayer mulligans
    
    // Test execution...
}
``` 