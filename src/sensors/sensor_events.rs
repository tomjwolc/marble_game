use super::*;

pub fn check_sensor_events(
    rapier_context: Res<RapierContext>,
    sensor_entity_query: Query<(Entity, &SensorChannel), With<Sensor>>,
    object_entity_query: Query<(Entity, &SensorChannel), Without<Sensor>>,
    mut sensor_events: EventWriter<SensorEvent>
) {
    let object_entities: Vec<(Entity, &SensorChannel)> = object_entity_query.iter().collect();

    for (sensor_entity, sensor_channel) in sensor_entity_query.iter() {
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
                        object_channel.in_channel(&sensor_channel)
                    }) 
                {sensor_events.send(SensorEvent { 
                    sensor_channel: sensor_channel.clone(), 
                    sensor_entity, 
                    object_entity
                })};
            }
        }
    }
}

pub fn respawn_sensor(
    mut sensor_events: EventReader<SensorEvent>,
    mut states: ResMut<NextState<AppState>>,
    mut menu_scheduler: ResMut<MenuScheduler>
) {
    // This may clobber other sensor events
    for SensorEvent { sensor_channel, .. } in sensor_events.into_iter() {
        if *sensor_channel == SensorChannel::Respawn {
            menu_scheduler.set_menu_type(MenuType::DeathScreen);
            states.set(AppState::OverlayMenu);
        }
    }
}