use super::*;

pub fn check_sensor_events(
    rapier_context: Res<RapierContext>,
    mut sensor_entity_query: Query<(Entity, &SensorChannel, &mut SensorEvents), With<Sensor>>,
    mut object_entity_query: Query<(Entity, &SensorChannel, &mut ObjectEvents), Without<Sensor>>,
) {
    let mut event_id = 0;

    for (
        object_entity, 
        object_sensor_channels, 
        mut object_events
    ) in object_entity_query.iter_mut() {
        *object_events = ObjectEvents::new();

        for (
            sensor_entity, 
            sensor_channel,
            mut sensor_events,
        ) in sensor_entity_query.iter_mut() {
            let is_intersecting = object_sensor_channels.contains(*sensor_channel) &&
                rapier_context.intersection_pair(sensor_entity, object_entity) == Some(true);

            if is_intersecting {
                object_events.get_mut(*sensor_channel).insert(event_id);
                sensor_events.ongoing_events.insert(event_id);

                event_id += 1;
            }
        }

        // if DEBUG_SENSORS && object_sensor_trigger.is_active && !object_sensor_trigger.was_prev_active {
        //     println!("{:?} has entered {:?}", object_entity, object_sensor_trigger);
        // } else if DEBUG_SENSORS && !object_sensor_trigger.is_active && object_sensor_trigger.was_prev_active {
        //     println!("{:?}  has exited {:?}", object_entity, object_sensor_trigger);
        // }
    }
}

pub fn respawn_events(
    player_events_query: Query<&ObjectEvents, With<Player>>,
    mut menu_scheduler: ResMut<MenuScheduler>,
    mut state: ResMut<NextState<AppState>>
) {
    if let Ok(object_events) = player_events_query.get_single() {
        if object_events.get(SensorChannel::Respawn).len() > 0 {
            menu_scheduler.set_menu_type(MenuType::DeathScreen);
            state.set(AppState::OverlayMenu);
        }
    }
}

pub fn warp_events(
    warp_sensors: Query<(&WarpTo, &SensorEvents), With<Sensor>>,
    mut level_stack: ResMut<LevelStack>,
    mut menu_scheduler: ResMut<MenuScheduler>,
    mut state: ResMut<NextState<AppState>>,
) {
    for (warp_to, SensorEvents { ongoing_events, .. }) in warp_sensors.iter() {
        if ongoing_events.len() > 0 {
            level_stack.warp(warp_to);

            menu_scheduler.set_menu_type(MenuType::WinScreen);
            state.set(AppState::OverlayMenu);
        }   
    }
}

pub fn activator_events(
    activator_query: Query<(&Activator, &SensorEvents)>,
    mut activation_table: ResMut<ActivationTable>
) {
    activation_table.0.iter_mut().for_each(|b| *b = false);

    for (Activator(id), SensorEvents { ongoing_events, .. }) in activator_query.iter() {
        activation_table.0[*id] = ongoing_events.len() > 0;
    }
}

pub fn gravity_sensor_events(
    gravity_sensor_events_query: Query<(&GravitySensorDirection, &SensorEvents)>,
    mut object_events_query: Query<(&mut Gravity, &ObjectEvents), Without<Sensor>>
) {
    for (mut gravity, object_events) in object_events_query.iter_mut() {
        let object_event_id_set = object_events.get(SensorChannel::Gravity);

        *gravity = if object_event_id_set.len() > 0 {
            Gravity(
                gravity_sensor_events_query.iter().find(|(_, sensor_events)| {
                    !sensor_events.ongoing_events.is_disjoint(object_event_id_set)
                }).expect("Could not find the sensor with the same event id in gravity_sensor_events").0.0,
                GravityType::Constant
            )
        } else {
            Gravity(-Vec3::Y, GravityType::Planets)
        };
    }
}