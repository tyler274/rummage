# Multiplayer Commander Networking

This document focuses on the specific networking considerations for supporting multiplayer Commander games with 4+ players. Commander games have unique requirements that affect networking design and implementation.

## Table of Contents

1. [Multiplayer Commander Format](#multiplayer-commander-format)
2. [Scaling Considerations](#scaling-considerations)
3. [Player Interactions](#player-interactions)
4. [Politics System](#politics-system)
5. [Zone Visibility](#zone-visibility)
6. [Event Broadcasting](#event-broadcasting)
7. [State Management](#state-management)

## Multiplayer Commander Format

Commander is a multiplayer-focused format with specific rules that affect networking:

- Games typically involve 3-6 players (with 4 being the standard)
- Each player has a designated legendary creature as their commander
- Players start at 40 life instead of 20
- Commander damage tracking (21+ damage from a single commander eliminates a player)
- Free-for-all political gameplay with temporary alliances
- Complex board states with many permanents
- Games can last multiple hours

These characteristics require special networking considerations beyond those of typical 1v1 Magic games.

## Scaling Considerations

### Player Count Scaling

```rust
/// Configuration for multiplayer Commander games
#[derive(Resource)]
pub struct CommanderMultiplayerConfig {
    /// Minimum number of players required to start
    pub min_players: usize,
    /// Maximum number of players allowed
    pub max_players: usize,
    /// Whether to allow spectators
    pub allow_spectators: bool,
    /// Maximum number of spectators
    pub max_spectators: Option<usize>,
    /// Timeout for player actions (in seconds)
    pub player_timeout: Option<u32>,
    /// Whether to enable turn timer
    pub use_turn_timer: bool,
    /// Turn time limit (in seconds)
    pub turn_time_limit: Option<u32>,
}

impl Default for CommanderMultiplayerConfig {
    fn default() -> Self {
        Self {
            min_players: 2, // Support even 1v1 Commander
            max_players: 6, // Default to classic pod size
            allow_spectators: true,
            max_spectators: Some(10),
            player_timeout: Some(60), // 1 minute default timeout
            use_turn_timer: false,
            turn_time_limit: Some(120), // 2 minutes per turn
        }
    }
}
```

### Data Volume Optimization

With 4+ players, the amount of state data increases significantly. Optimizing data transmission is crucial:

```rust
/// System to optimize network traffic based on player count
pub fn optimize_data_transmission(
    player_count: Res<PlayerCount>,
    mut server_config: ResMut<ServerReplicationConfig>,
) {
    // Adjust replication frequency based on player count
    match player_count.active {
        1..=2 => {
            // For 1-2 players, use higher frequency updates
            server_config.replication_frequency = 30; // ~30Hz
        }
        3..=4 => {
            // For 3-4 players, use moderate frequency
            server_config.replication_frequency = 20; // ~20Hz
        }
        _ => {
            // For 5+ players, reduce update frequency
            server_config.replication_frequency = 10; // ~10Hz
        }
    }
    
    // Adjust component replication priorities
    if player_count.active > 4 {
        // Prioritize critical components in high-player-count games
        server_config.priority_components = vec![
            ComponentPriority { type_id: TypeId::of::<Player>(), priority: 10 },
            ComponentPriority { type_id: TypeId::of::<Phase>(), priority: 9 },
            ComponentPriority { type_id: TypeId::of::<Commander>(), priority: 8 },
            // Less critical components get lower priority
            ComponentPriority { type_id: TypeId::of::<CardArt>(), priority: 1 },
        ];
    }
}
```

## Player Interactions

Commander games feature unique player-to-player interactions that must be properly networked:

### Table Politics

```rust
#[derive(Event, Serialize, Deserialize, Clone, Debug)]
pub struct PoliticalDealProposal {
    /// Player proposing the deal
    pub proposer: Entity,
    /// Player receiving the proposal
    pub target: Entity,
    /// The proposed deal terms
    pub terms: Vec<DealTerm>,
    /// How long the deal should last
    pub duration: DealDuration,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DealTerm {
    /// Promise not to attack
    NonAggression,
    /// Promise to attack another player
    AttackPlayer(Entity),
    /// Promise not to counter spells
    NonCountering,
    /// Promise to help with a specific threat
    RemoveThreat(Entity),
    /// One-time favor (free attack, no blocks, etc.)
    OneTimeFavor(FavorType),
    /// Custom text agreement
    Custom(String),
}

/// System to handle political deal proposals and responses
pub fn handle_political_deals(
    mut server: ResMut<RepliconServer>,
    mut proposals: EventReader<FromClient<PoliticalDealProposal>>,
    mut responses: EventReader<FromClient<PoliticalDealResponse>>,
    mut active_deals: ResMut<ActiveDeals>,
    connected_clients: Res<ConnectedClients>,
) {
    // Process new deal proposals
    for FromClient { client_id, event } in proposals.read() {
        // Validate the proposal
        if validate_deal_proposal(&event, &connected_clients) {
            // Forward the proposal to the target player
            if let Some(target_client_id) = get_client_id_for_player(event.target) {
                server.send(
                    target_client_id, 
                    RepliconChannel::UnreliableOrdered,
                    bincode::serialize(&event).unwrap()
                );
            }
        }
    }
    
    // Process deal responses
    for FromClient { client_id, event } in responses.read() {
        if event.accepted {
            // Add to active deals if accepted
            active_deals.add(event.proposal_id.clone(), event.proposal.clone());
            
            // Notify both parties
            let notification = DealAcceptedNotification {
                deal_id: event.proposal_id.clone(),
            };
            
            notify_deal_participants(&mut server, &event.proposal, &notification);
        } else {
            // Notify of rejection
            let notification = DealRejectedNotification {
                deal_id: event.proposal_id.clone(),
            };
            
            notify_deal_participants(&mut server, &event.proposal, &notification);
        }
    }
}
```

### Voting Mechanism

Commander often involves voting effects from cards:

```rust
#[derive(Event, Serialize, Deserialize, Clone, Debug)]
pub struct VoteProposal {
    /// The card or effect that initiated the vote
    pub source: Entity,
    /// The player who controls the voting effect
    pub controller: Entity,
    /// The available choices
    pub choices: Vec<VoteChoice>,
    /// Time limit for voting (in seconds)
    pub time_limit: Option<u32>,
}

#[derive(Event, Serialize, Deserialize, Clone, Debug)]
pub struct PlayerVote {
    /// The vote proposal ID
    pub proposal_id: Uuid,
    /// The player casting the vote
    pub player: Entity,
    /// The chosen option
    pub choice: usize,
}

/// System to synchronize voting across all players
pub fn handle_voting(
    mut server: ResMut<RepliconServer>,
    mut vote_proposals: EventReader<FromClient<VoteProposal>>,
    mut player_votes: EventReader<FromClient<PlayerVote>>,
    mut active_votes: ResMut<ActiveVotes>,
    connected_clients: Res<ConnectedClients>,
) {
    // Process new vote proposals from effects
    for FromClient { client_id, event } in vote_proposals.read() {
        // Validate that the client is authorized to initiate this vote
        if validate_vote_proposal(&event, client_id, &connected_clients) {
            // Create and store the vote
            let vote_id = Uuid::new_v4();
            active_votes.add(vote_id, event.clone());
            
            // Broadcast to all eligible voters
            for player_entity in get_eligible_voters(&event) {
                if let Some(player_client_id) = get_client_id_for_player(player_entity) {
                    let vote_notification = VoteStartedNotification {
                        vote_id,
                        proposal: event.clone(),
                    };
                    
                    server.send(
                        player_client_id,
                        RepliconChannel::ReliableOrdered,
                        bincode::serialize(&vote_notification).unwrap()
                    );
                }
            }
        }
    }
    
    // Process incoming votes
    for FromClient { client_id, event } in player_votes.read() {
        if let Some(vote) = active_votes.get_mut(&event.proposal_id) {
            // Record the vote
            vote.record_vote(event.player, event.choice);
            
            // Check if voting is complete
            if vote.is_complete() {
                // Determine the result
                let result = vote.tally_result();
                
                // Broadcast the result to all participants
                let result_notification = VoteCompletedNotification {
                    vote_id: event.proposal_id,
                    result,
                };
                
                for participant in vote.participants() {
                    if let Some(participant_client_id) = get_client_id_for_player(participant) {
                        server.send(
                            participant_client_id,
                            RepliconChannel::ReliableOrdered,
                            bincode::serialize(&result_notification).unwrap()
                        );
                    }
                }
            }
        }
    }
}
```

## Politics System

Commander games feature political elements that affect gameplay:

```rust
/// Architecture for politics system in Commander
pub struct PoliticsPlugin;

impl Plugin for PoliticsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<ActiveDeals>()
            .init_resource::<ActiveVotes>()
            .init_resource::<Alliances>()
            .init_resource::<CombatRestrictions>()
            .init_resource::<MonarchState>()
            
            // Components
            .register_type::<PoliticalDeal>()
            .register_type::<Alliance>()
            .register_type::<VoteWeight>()
            .register_type::<Monarch>()
            .register_type::<Goad>()
            .register_type::<CombatRestriction>()
            
            // Events
            .add_event::<PoliticalDealProposal>()
            .add_event::<PoliticalDealResponse>()
            .add_event::<VoteProposal>()
            .add_event::<PlayerVote>()
            .add_event::<MonarchChangeEvent>()
            .add_event::<GoadEvent>()
            
            // Systems
            .add_systems(Update, (
                handle_political_deals,
                handle_voting,
                track_monarch,
                enforce_goad,
                update_combat_restrictions,
                check_deal_expirations,
            ))
            
            // Replicate components and events
            .replicate::<PoliticalDeal>()
            .replicate::<Alliance>()
            .replicate::<VoteWeight>()
            .replicate::<Monarch>()
            .replicate::<Goad>()
            .replicate_event::<MonarchChangeEvent>()
            .replicate_event::<VoteCompletedNotification>()
            .replicate_event::<DealAcceptedNotification>()
            .replicate_event::<DealRejectedNotification>();
    }
}

/// System to enforce goad effects, which are political combat-forcing effects
pub fn enforce_goad(
    mut commands: Commands,
    goaded_creatures: Query<(Entity, &Goad)>,
    mut attack_restrictions: ResMut<CombatRestrictions>,
    time: Res<Time>,
) {
    // Process all goaded creatures
    for (entity, goad) in &goaded_creatures {
        // Check if the goad effect has expired
        if let Some(expiration) = goad.expires_at {
            if time.elapsed_seconds() > expiration {
                // Remove expired goad effect
                commands.entity(entity).remove::<Goad>();
                attack_restrictions.remove_restriction(entity, RestrictionType::MustAttack);
            } else {
                // Ensure the creature has attack restrictions
                if !attack_restrictions.has_restriction(entity, RestrictionType::MustAttack) {
                    attack_restrictions.add_restriction(
                        entity,
                        RestrictionType::MustAttack(goad.goaded_by),
                    );
                }
            }
        }
    }
}
```

## Zone Visibility

Commander requires special handling for zone visibility:

```rust
/// System to manage zone visibility in multiplayer Commander
pub fn manage_zone_visibility(
    mut commands: Commands,
    cards: Query<(Entity, &Card, &Zone, Option<&ClientVisibility>)>,
    players: Query<(Entity, &ReplicatedClient)>,
    reveal_events: EventReader<CardRevealEvent>,
) {
    // Special case: Command Zone
    // Command zone is public, so commanders are visible to all players
    for (entity, card, zone, _) in cards.iter().filter(|(_, _, z, _)| z.zone_type == ZoneType::Command) {
        // Ensure commanders are visible to all
        commands.entity(entity).remove::<ClientVisibility>();
    }
    
    // Handle hidden zones (hand, library)
    for (entity, card, zone, existing_visibility) in cards.iter().filter(|(_, _, z, _)| 
        z.zone_type == ZoneType::Hand || z.zone_type == ZoneType::Library
    ) {
        match zone.zone_type {
            ZoneType::Hand => {
                // Only the owner can see cards in hand
                if let Some(owner_client_id) = get_client_id_for_player(card.owner, &players) {
                    let blacklist: Vec<ClientId> = players
                        .iter()
                        .filter_map(|(_, client)| {
                            if client.client_id != owner_client_id {
                                Some(client.client_id)
                            } else {
                                None
                            }
                        })
                        .collect();
                    
                    commands.entity(entity).insert(ClientVisibility {
                        policy: VisibilityPolicy::Blacklist,
                        client_ids: blacklist,
                    });
                }
            },
            ZoneType::Library => {
                // Libraries are hidden from all players
                let all_clients: Vec<ClientId> = players
                    .iter()
                    .map(|(_, client)| client.client_id)
                    .collect();
                
                commands.entity(entity).insert(ClientVisibility {
                    policy: VisibilityPolicy::Blacklist,
                    client_ids: all_clients,
                });
            },
            _ => {},
        }
    }
    
    // Process reveal events (when cards are revealed to specific players)
    for reveal_event in reveal_events.read() {
        if let Ok((entity, card, zone, _)) = cards.get(reveal_event.card) {
            // Create a whitelist of players who can see this revealed card
            let whitelist: Vec<ClientId> = reveal_event.visible_to
                .iter()
                .filter_map(|player| get_client_id_for_player(*player, &players))
                .collect();
            
            if !whitelist.is_empty() {
                commands.entity(entity).insert(ClientVisibility {
                    policy: VisibilityPolicy::Whitelist,
                    client_ids: whitelist,
                });
                
                // Schedule automatic un-reveal after the specified duration
                if let Some(duration) = reveal_event.duration {
                    commands.entity(entity).insert(TemporaryReveal {
                        revert_at: time.elapsed_seconds() + duration,
                        original_zone: zone.zone_type,
                    });
                }
            }
        }
    }
}
```

## Event Broadcasting

Commander games generate many events that need to be broadcast to players:

```rust
/// System to efficiently broadcast game events to players
pub fn broadcast_game_events(
    mut server: ResMut<RepliconServer>,
    mut turn_events: EventReader<TurnEvent>,
    mut combat_events: EventReader<CombatEvent>,
    mut commander_events: EventReader<CommanderEvent>,
    mut state_events: EventReader<GameStateEvent>,
    connected_clients: Res<ConnectedClients>,
) {
    // Create a batch of events to send to each client
    let mut client_event_batches: HashMap<ClientId, Vec<GameEvent>> = HashMap::new();
    
    // Process turn events (turn start, phase changes, etc.)
    for event in turn_events.read() {
        // Turn events are public and sent to all players
        for client_id in connected_clients.iter() {
            client_event_batches
                .entry(*client_id)
                .or_default()
                .push(GameEvent::Turn(event.clone()));
        }
    }
    
    // Process combat events
    for event in combat_events.read() {
        // Combat events are public and sent to all players
        for client_id in connected_clients.iter() {
            client_event_batches
                .entry(*client_id)
                .or_default()
                .push(GameEvent::Combat(event.clone()));
        }
    }
    
    // Process commander-specific events
    for event in commander_events.read() {
        match event {
            CommanderEvent::CommanderCast { player, commander } => {
                // Public event, send to all
                for client_id in connected_clients.iter() {
                    client_event_batches
                        .entry(*client_id)
                        .or_default()
                        .push(GameEvent::Commander(event.clone()));
                }
            },
            CommanderEvent::CommanderDamage { source, target, amount } => {
                // Public event, send to all
                for client_id in connected_clients.iter() {
                    client_event_batches
                        .entry(*client_id)
                        .or_default()
                        .push(GameEvent::Commander(event.clone()));
                }
            },
            CommanderEvent::PlayerEliminated { player, reason } => {
                // Public event, send to all
                for client_id in connected_clients.iter() {
                    client_event_batches
                        .entry(*client_id)
                        .or_default()
                        .push(GameEvent::Commander(event.clone()));
                }
            },
        }
    }
    
    // Process game state events
    for event in state_events.read() {
        // Game state events might contain hidden information
        // Filter based on event type and player visibility
        match event {
            GameStateEvent::CardDrawn { player, card } => {
                // Only notify the player who drew the card
                if let Some(client_id) = get_client_id_for_player(*player) {
                    client_event_batches
                        .entry(client_id)
                        .or_default()
                        .push(GameEvent::State(event.clone()));
                }
                
                // Notify others of a card drawn but hide the card identity
                let public_event = GameStateEvent::CardDrawn {
                    player: *player,
                    card: Entity::PLACEHOLDER, // Hide the actual card
                };
                
                for client_id in connected_clients.iter() {
                    if Some(*client_id) != get_client_id_for_player(*player) {
                        client_event_batches
                            .entry(*client_id)
                            .or_default()
                            .push(GameEvent::State(public_event.clone()));
                    }
                }
            },
            GameStateEvent::LibrarySearched { player, cards_revealed } => {
                // Only the searching player sees the cards
                if let Some(client_id) = get_client_id_for_player(*player) {
                    client_event_batches
                        .entry(client_id)
                        .or_default()
                        .push(GameEvent::State(event.clone()));
                }
                
                // Others just know a search happened
                let public_event = GameStateEvent::LibrarySearched {
                    player: *player,
                    cards_revealed: Vec::new(), // Hide the actual cards
                };
                
                for client_id in connected_clients.iter() {
                    if Some(*client_id) != get_client_id_for_player(*player) {
                        client_event_batches
                            .entry(*client_id)
                            .or_default()
                            .push(GameEvent::State(public_event.clone()));
                    }
                }
            },
            // Public events are sent to everyone
            GameStateEvent::LifeTotalChanged { player, new_total, change } |
            GameStateEvent::ManaAdded { player, mana } |
            GameStateEvent::PermanentEntered { permanent, controller } => {
                for client_id in connected_clients.iter() {
                    client_event_batches
                        .entry(*client_id)
                        .or_default()
                        .push(GameEvent::State(event.clone()));
                }
            },
        }
    }
    
    // Send batched events to clients
    for (client_id, events) in client_event_batches {
        if !events.is_empty() {
            let event_batch = EventBatch {
                events,
                timestamp: time.elapsed_seconds(),
            };
            
            server.send(
                client_id,
                RepliconChannel::ReliableOrdered,
                bincode::serialize(&event_batch).unwrap(),
            );
        }
    }
}
```

## State Management

For large Commander games, state management needs special attention:

```rust
/// Enhanced state sync system for multiplayer Commander
pub fn sync_commander_game_state(
    mut commands: Commands,
    mut server: ResMut<RepliconServer>,
    game_state: Res<GameState>,
    turn_manager: Res<TurnManager>,
    zone_manager: Res<ZoneManager>,
    player_query: Query<(Entity, &Player, &CommanderState)>,
    connected_clients: Res<ConnectedClients>,
    mut client_sync_status: ResMut<ClientSyncStatus>,
    time: Res<Time>,
) {
    // Determine which clients need full or delta syncs
    for client_id in connected_clients.iter() {
        let client_status = client_sync_status.entry(*client_id).or_insert(SyncStatus {
            last_full_sync: 0.0,
            last_delta_sync: 0.0,
            sync_generation: 0,
        });
        
        let current_time = time.elapsed_seconds();
        
        // Client needs a full sync if:
        // 1. Never received a full sync before
        // 2. Haven't received a full sync in a long time
        // 3. Just connected
        let needs_full_sync = client_status.last_full_sync == 0.0 || 
                             current_time - client_status.last_full_sync > 30.0 ||
                             client_status.sync_generation == 0;
        
        // Regular delta sync interval
        let needs_delta_sync = current_time - client_status.last_delta_sync > 0.25; // 4 Hz
        
        if needs_full_sync {
            // Send full game state to client
            send_full_game_state(&mut server, *client_id, &game_state, &turn_manager, &zone_manager, &player_query);
            
            // Update sync status
            client_status.last_full_sync = current_time;
            client_status.last_delta_sync = current_time;
            client_status.sync_generation += 1;
        } else if needs_delta_sync {
            // Send delta update
            send_delta_update(&mut server, *client_id, &game_state, &turn_manager, &player_query);
            
            // Update sync status
            client_status.last_delta_sync = current_time;
        }
    }
}

/// Optimized serialization with compression for large game states
fn send_full_game_state(
    server: &mut ResMut<RepliconServer>,
    client_id: ClientId,
    game_state: &Res<GameState>,
    turn_manager: &Res<TurnManager>,
    zone_manager: &Res<ZoneManager>,
    player_query: &Query<(Entity, &Player, &CommanderState)>,
) {
    // Create a full game state snapshot
    let mut snapshot = GameStateSnapshot {
        is_full_sync: true,
        sync_generation: time.frame_count(),
        turn: turn_manager.turn_number,
        phase: turn_manager.current_phase.clone(),
        active_player: turn_manager.active_player,
        priority_player: turn_manager.priority_player,
        players: Vec::new(),
        zones: Vec::new(),
        permanents: Vec::new(),
        stack: Vec::new(),
    };
    
    // Add player data (filtering based on client visibility)
    for (entity, player, commander_state) in player_query.iter() {
        snapshot.players.push(PlayerSnapshot {
            entity,
            name: player.name.clone(),
            life: player.life,
            mana_pool: player.mana_pool.clone(),
            commander: commander_state.commander,
            commander_casts: commander_state.cast_count,
            commander_damage_received: commander_state.commander_damage_received.clone(),
        });
    }
    
    // Add zone data
    for (zone_entity, zone) in zone_manager.zones.iter() {
        // Filter cards based on visibility to this client
        let visible_cards = filter_visible_cards(zone, client_id);
        
        snapshot.zones.push(ZoneSnapshot {
            entity: *zone_entity,
            zone_type: zone.zone_type,
            owner: zone.owner,
            cards: visible_cards,
        });
    }
    
    // Add permanents on battlefield
    for permanent in game_state.battlefield.iter() {
        if let Ok(permanent_data) = get_permanent_data(*permanent) {
            snapshot.permanents.push(permanent_data);
        }
    }
    
    // Add stack items
    for stack_item in game_state.stack.items() {
        snapshot.stack.push(StackItemSnapshot {
            entity: stack_item.entity,
            source: stack_item.source,
            controller: stack_item.controller,
            targets: stack_item.targets.clone(),
        });
    }
    
    // Compress the snapshot to reduce bandwidth usage
    let serialized = bincode::serialize(&snapshot).unwrap();
    let compressed = compress_data(&serialized);
    
    // Send the compressed snapshot
    server.send(
        client_id,
        RepliconChannel::ReliableOrdered,
        Bytes::from(compressed),
    );
}
```

This document provides detailed guidance on the specific networking considerations for multiplayer Commander games, focusing on scaling challenges, political elements, and the complexity of managing large game states across many players. 