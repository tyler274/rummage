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