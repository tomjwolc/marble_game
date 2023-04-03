use super::*;

use colored::*;

pub fn check_sensor_events(
    rapier_context: Res<RapierContext>,
    mut sensor_entity_query: Query<(Entity, &SensorChannel, &mut SensorEvents), With<Sensor>>,
    mut object_entity_query: Query<(Entity, &SensorChannel, &mut ObjectEvents), Without<Sensor>>,
) {
    for (
        _, _,
        mut sensor_events,
    ) in sensor_entity_query.iter_mut() {
        // sets sensor_events to SensorEvents(HashSet::new(), sensor_events.0)
        sensor_events.1 = std::mem::replace(&mut sensor_events.0, HashSet::new());
    }

    for (
        object_entity, 
        object_sensor_channels, 
        mut object_events
    ) in object_entity_query.iter_mut() {
        object_events.1 = std::mem::replace(&mut object_events.0, vec![HashSet::new(); NUM_SENSOR_CHANNELS]);

        for (
            sensor_entity, 
            sensor_channel,
            mut sensor_events,
        ) in sensor_entity_query.iter_mut() {
            let is_intersecting = object_sensor_channels.contains(*sensor_channel) &&
                rapier_context.intersection_pair(sensor_entity, object_entity) == Some(true);

            if is_intersecting {
                object_events.get_mut(*sensor_channel).insert((object_entity, sensor_entity));
                sensor_events.0.insert((object_entity, sensor_entity));
            }
        }

        if DEBUG_SENSORS {
            if object_events.0 != object_events.1 {
                println!(
                    "{} {:?}", 
                    format!(
                        "[ {: <5} {: >12} ]:", 
                        format!("{:?}", object_entity).bold(), 
                        format!("{}", object_sensor_channels).bold()
                    ).yellow(),
                    object_events.0
                );
            }
        }
    }

    if DEBUG_SENSORS {
        for (
            sensor_entity,
            sensor_channel,
            sensor_events,
        ) in sensor_entity_query.iter() {
            if sensor_events.0.len() != sensor_events.1.len() {
                println!(
                    "{} {:?}", 
                    format!(
                        "[ {: <5} {: >12} ]:", 
                        format!("{:?}", sensor_entity).bold(), 
                        format!("{}", sensor_channel).bold()
                    ).purple(),
                    sensor_events.0
                );
            }
        }
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
    player_events_query: Query<&ObjectEvents, With<Player>>,
    warp_sensors: Query<(&WarpTo, &SensorEvents), With<Sensor>>,
    mut level_stack: ResMut<LevelStack>,
    mut menu_scheduler: ResMut<MenuScheduler>,
    mut state: ResMut<NextState<AppState>>,
) {
    let Ok(object_events) = player_events_query.get_single() else { return };

    for (warp_to, SensorEvents(sensor_events, _)) in warp_sensors.iter() {
        if !object_events.get(SensorChannel::Warp).is_disjoint(sensor_events) {
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

    for (Activator(id), SensorEvents(ongoing_events, _)) in activator_query.iter() {
        activation_table.0[*id] = ongoing_events.len() > 0;
    }
}

pub fn gravity_sensor_events(
    gravity_sensor_events_query: Query<(&GravitySensorDirection, &SensorEvents)>,
    mut object_events_query: Query<(&mut Gravity, &ObjectEvents), Without<Sensor>>
) {
    for (mut gravity, object_events) in object_events_query.iter_mut() {
        let object_event_id_set = object_events.get(SensorChannel::Gravity);

        if object_event_id_set.len() > 0 {
            *gravity = Gravity(
                gravity_sensor_events_query.iter().find(|(_, sensor_events)| {
                    !sensor_events.0.is_disjoint(object_event_id_set)
                }).expect("Could not find the sensor with the same event id in gravity_sensor_events").0.0,
                GravityType::Constant
            );
        } else {
            gravity.1 = GravityType::Planets;
        };
    }
}

pub fn can_jump_sensor_events(
    can_jump_sensor_query: Query<&SensorEvents, With<CanJumpSensor>>,
    mut can_jump: ResMut<CanJump>
) {
    let Ok(sensor_events) = can_jump_sensor_query.get_single() else { return };
    can_jump.0 = sensor_events.0.len() > 0;
}