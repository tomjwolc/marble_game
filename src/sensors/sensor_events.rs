use super::*;

pub fn check_sensor_events(
    rapier_context: Res<RapierContext>,
    sensor_entity_query: Query<(Entity, &SensorChannel, Option<&WarpTo>), With<Sensor>>,
    object_entity_query: Query<(Entity, &SensorChannel), Without<Sensor>>,
    mut respawn_events: EventWriter<RespawnEvent>,
    mut warp_events: EventWriter<WarpEvent>,
) {
    let object_entities: Vec<(Entity, &SensorChannel)> = object_entity_query.iter().collect();

    for (
        sensor_entity, 
        sensor_channel,
        warp_to_option
    ) in sensor_entity_query.iter() {
        for (
            collider1, 
            collider2, 
            is_intersecting
        ) in rapier_context.intersections_with(sensor_entity) {
            //     If the colliders are intersecting, one of the colliders is a 
            // object_entity and the colliders share a sensor channel
            if is_intersecting {
                if let Some( &(object_entity, _) ) = object_entities
                    .iter().find(|(entity, object_channel)| {
                        (*entity == collider1 || *entity == collider2) && 
                        object_channel.contains(*sensor_channel)
                    }) 
                {
                    println!("channel: {:?}", sensor_channel);

                    match *sensor_channel {
                        SensorChannel::Respawn => respawn_events.send(RespawnEvent),
                        SensorChannel::Warp => if let Some(warp_to) = warp_to_option {
                            warp_events.send(WarpEvent { 
                                warp_to: warp_to.clone(), 
                                object_entity 
                            });
                        },
                        _ => {}
                    }
                };
            }
        }
    }
}

pub fn respawn_events(
    mut respawn_events: EventReader<RespawnEvent>,
    mut menu_scheduler: ResMut<MenuScheduler>,
    mut state: ResMut<NextState<AppState>>
) {
    if respawn_events.len() > 0 {
        menu_scheduler.set_menu_type(MenuType::DeathScreen);
        state.set(AppState::OverlayMenu);

        respawn_events.clear();
    }
}

pub fn warp_events(
    mut warp_events: EventReader<WarpEvent>,
    mut level_stack: ResMut<LevelStack>,
    mut menu_scheduler: ResMut<MenuScheduler>,
    mut state: ResMut<NextState<AppState>>,
    player_entity_query: Query<Entity, With<Player>>
) {
    let Ok(player_entity) = player_entity_query.get_single() else { return };

    for warp_event in warp_events.iter() {
        println!("event: {:?}\nplayer_entity: {:?}", warp_event, player_entity);

        if player_entity == warp_event.object_entity {
            level_stack.warp(&warp_event.warp_to);

            menu_scheduler.set_menu_type(MenuType::Loading);
            state.set(AppState::MenuScreen);
        }
    }
}