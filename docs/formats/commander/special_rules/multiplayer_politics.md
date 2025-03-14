# Multiplayer Politics

## Overview

The Multiplayer Politics module handles the social and strategic elements unique to multiplayer Commander games. This includes deal-making, alliance formation, temporary agreements, and other player interactions not codified in the standard Magic rules.

## Core Features

The system includes:

- **Deal Making System**: Framework for players to propose, accept, and track in-game deals
- **Threat Assessment**: Tools to analyze and display relative threat levels of players
- **Alliance Tracking**: Temporary cooperative arrangements between players
- **Table Talk Integration**: Support for in-game communication with policy enforcement

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

## Deal Making

The deal making system allows players to:

- Propose deals with specific terms and duration
- Accept or reject deals from other players
- Set automatic deal conditions and consequences
- Track deal fulfillment and violations

Deals are non-binding at the rules level but provide framework for player agreements.

## Threat Assessment

The threat assessment system helps players evaluate relative threats by:

- Displaying board state power metrics
- Tracking win proximity indicators
- Highlighting potential combo pieces
- Providing history of player actions and tendencies

## AI Integration

For games with AI opponents, the politics system:

- Models AI political decision making based on configured personalities
- Evaluates deal proposals based on game state and risk assessment
- Tracks human player tendencies for future political decisions
- Simulates realistic political behavior for different AI difficulty levels

## UI Components

The multiplayer politics UI provides:

- Deal proposal interface with customizable terms
- Alliance status indicators
- Threat assessment visualization
- Communication tools with appropriate filters
- Deal history and player reputation tracking

## Constraints and Limitations

The politics system operates within these constraints:

- No rules enforcement of political agreements (maintaining game integrity)
- Appropriate limits on information sharing for hidden information
- Configurable table talk policies to match playgroup preferences
- Balance between automation and player agency in political decisions 