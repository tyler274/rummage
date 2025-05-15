pub(super) fn handle_entity_selection(
    mut interaction_query: Query<(&Interaction, &Parent), Changed<Interaction>>,
    parent_query: Query<&CardId>,
    mut select_card_event: EventWriter<SelectCardEvent>,
) {
    for (interaction, parent_entity) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if let Ok(card_id) = parent_query.get(parent_entity.get()) {
                select_card_event.write(SelectCardEvent { card_id: *card_id });
            }
        }
    }
}

pub(super) fn handle_card_selection_event(
    mut events: EventReader<SelectCardEvent>,
    mut card_state_query: Query<(&CardId, &mut CardState)>,
    mut next_card_state: ResMut<NextState<CardSelectionState>>,
    mut target_selection_input_writer: EventWriter<TargetSelectionInput>,
    selected_card_state: Res<State<CardSelectionState>>,
    mut selected_ids: ResMut<SelectedCardIds>,
    card_selection_limit: Res<CardSelectionLimit>,
) {
    for event in events.read() {
        match event.interaction {
            InteractionResult::Continue => {}
            InteractionResult::SelectTarget(target_type) => {
                info!("Select Target: {:?}", target_type);
                selected_ids.ids.push(event.card_id);
                target_selection_input_writer.write(TargetSelectionInput { target_type });
                next_card_state.set(CardSelectionState::Targeting);
            }
        }
    }
}

pub(super) fn handle_target_selection_input(
    mut target_selection_input_reader: EventReader<TargetSelectionInput>,
    mut target_selection_output_writer: EventWriter<TargetSelectionOutput>,
) {
    for event in target_selection_input_reader.read() {
        // TODO: Implement target selection logic
        // For now, just echo the input to the output
        target_selection_output_writer.write(TargetSelectionOutput {
            target: event.target_type,
        });
    }
}

pub(super) fn handle_target_selection_output(
    mut target_selection_output_reader: EventReader<TargetSelectionOutput>,
    mut selected_ids: ResMut<SelectedCardIds>,
    mut card_play_event_writer: EventWriter<CardPlayEvent>,
    mut next_card_state: ResMut<NextState<CardSelectionState>>,
) {
    for event in target_selection_output_reader.read() {
        // TODO: Handle target selection output
        // For now, assume the first selected card is played with the selected target
        if let Some(card_id) = selected_ids.ids.first() {
            card_play_event_writer.write(CardPlayEvent {
                card_id: *card_id,
                target: Some(event.target),
            });
        }
        selected_ids.ids.clear();
        next_card_state.set(CardSelectionState::Idle);
    }
}

pub(super) fn handle_card_play_event(
    mut events: EventReader<CardPlayEvent>,
    mut card_state_query: Query<(&CardId, &mut CardState)>,
    mut add_to_stack_writer: EventWriter<AddToStackEvent>,
    script_map: Res<CardScriptMap>,
) {
    for event in events.read() {
        for (card_id, mut card_state) in card_state_query.iter_mut() {
            if card_id == &event.card_id {
                card_state.play();
                if let Some(script) = script_map.map.get(&card_id.archetype_id) {
                    (script.on_play)(card_id, event.target);
                    add_to_stack_writer.write(AddToStackEvent {
                        card_id: *card_id,
                        script: *script,
                        target: event.target,
                    });
                }
            }
        }
    }
}

pub(super) fn handle_counter_spell_input(
    mut events: EventReader<CounterSpellInputEvent>,
    mut card_state_query: Query<(&CardId, &mut CardState)>,
    mut counter_spell_output_event_writer: EventWriter<CounterSpellOutputEvent>,
) {
    for event in events.read() {
        for (card_id, mut card_state) in card_state_query.iter_mut() {
            if card_id == &event.card_id {
                if card_state.can_counter() {
                    card_state.counter();
                    counter_spell_output_event_writer.write(CounterSpellOutputEvent {
                        card_id: *card_id,
                        countered: true,
                    });
                } else {
                    counter_spell_output_event_writer.write(CounterSpellOutputEvent {
                        card_id: *card_id,
                        countered: false,
                    });
                }
            }
        }
    }
}
