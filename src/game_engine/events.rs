fn log_card_events(mut events: EventReader<CardEvent>, mut log_event: EventWriter<LogEvent>) {
    for event in events.read() {
        log_event.write(LogEvent {
            message: format!("CardEvent: {:?}", event),
        });
    }
}

fn log_player_events(mut events: EventReader<PlayerEvent>, mut log_event: EventWriter<LogEvent>) {
    for event in events.read() {
        log_event.write(LogEvent {
            message: format!("PlayerEvent: {:?}", event),
        });
    }
}

fn log_zone_events(mut events: EventReader<ZoneEvent>, mut log_event: EventWriter<LogEvent>) {
    for event in events.read() {
        log_event.write(LogEvent {
            message: format!("ZoneEvent: {:?}", event),
        });
    }
}

fn log_turn_events(mut events: EventReader<TurnEvent>, mut log_event: EventWriter<LogEvent>) {
    for event in events.read() {
        log_event.write(LogEvent {
            message: format!("TurnEvent: {:?}", event),
        });
    }
}

fn log_stack_events(mut events: EventReader<StackEvent>, mut log_event: EventWriter<LogEvent>) {
    for event in events.read() {
        log_event.write(LogEvent {
            message: format!("StackEvent: {:?}", event),
        });
    }
}

fn log_game_state_events(
    mut events: EventReader<GameStateEvent>,
    mut log_event: EventWriter<LogEvent>,
) {
    for event in events.read() {
        log_event.write(LogEvent {
            message: format!("GameStateEvent: {:?}", event),
        });
    }
}
