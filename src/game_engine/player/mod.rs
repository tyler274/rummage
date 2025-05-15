fn draw_card(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Player, &Children), With<Player>>,
    mut hand_query: Query<&mut HandZone>,
    mut library_query: Query<&mut LibraryZone>,
    mut draw_card_events: EventWriter<DrawCardEvent>,
) {
    for (player_entity, mut player, children) in query.iter_mut() {
        if player.cards_to_draw > 0 {
            let mut library_zone_entity = None;
            let mut hand_zone_entity = None;

            for child in children.iter() {
                if library_query.get_mut(*child).is_ok() {
                    library_zone_entity = Some(*child);
                }
                if hand_query.get_mut(*child).is_ok() {
                    hand_zone_entity = Some(*child);
                }
            }

            if let (Some(library_zone_entity), Some(hand_zone_entity)) =
                (library_zone_entity, hand_zone_entity)
            {
                if let Ok(mut library_zone) = library_query.get_mut(library_zone_entity) {
                    if let Some(card_id) = library_zone.draw_card() {
                        if let Ok(mut hand_zone) = hand_query.get_mut(hand_zone_entity) {
                            hand_zone.add_card(card_id);
                            player.cards_to_draw -= 1;
                            draw_card_events.write(DrawCardEvent {
                                player_id: player_entity,
                                card_id,
                            });
                        }
                    }
                }
            }
        }
    }
}
