use super::*;

use colored::*;

pub fn check_sensor_events(
    rapier_context: Res<RapierContext>,
    mut sensor_entity_query: Query<(Entity, &SensorChannel, &mut SensorEvents), (With<Sensor>, With<InGameEntity>)>,
    mut object_entity_query: Query<(Entity, &SensorChannel, &mut ObjectEvents), (With<InGameEntity>, Without<Sensor>)>,
) {
    for (
        _, _,
        mut sensor_events
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
            mut sensor_events
        ) in sensor_entity_query.iter_mut() {
            let is_intersecting = *sensor_channel != SensorChannel::None &&
                object_sensor_channels.contains(*sensor_channel) &&
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
            sensor_events
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
    player_events_query: Query<&ObjectEvents, (With<Player>, With<InGameEntity>)>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut state: ResMut<NextState<AppState>>
) {
    if let Ok(object_events) = player_events_query.get_single() {
        if object_events.get(SensorChannel::Respawn).len() > 0 {
            menu_state.set(MenuState::DeathScreen);
            state.set(AppState::OverlayMenu);
        }
    }
}

pub fn warp_events(
    player_events_query: Query<&ObjectEvents, (With<Player>, With<InGameEntity>)>,
    warp_sensors: Query<(Entity, &WarpTo, &SensorEvents), (With<Sensor>, With<InGameEntity>, Without<Timed>)>,
    mut level_stack: ResMut<LevelStack>,
    mut unload_type: ResMut<UnloadType>,
    mut load_type: ResMut<LoadType>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut state: ResMut<NextState<AppState>>,
    soft_unloaded_query: Query<With<SoftUnloadedData>>,
    mut commands: Commands,
    mut pkv_store: ResMut<PkvStore>
) {
    let Ok(object_events) = player_events_query.get_single() else { return };

    for (entity, warp_to, SensorEvents(sensor_events, _)) in warp_sensors.iter() {
        if !object_events.get(SensorChannel::Warp).is_disjoint(sensor_events) {
            (*unload_type, *load_type) = if let WarpTo::File(_) = warp_to { 
                (UnloadType::Soft, LoadType::Fresh) // Soft unloads current level and loads the next fresh
            } else {
                // Marks the current level as completed
                let mut completed_levels = pkv_store
                    .get::<CompletedLevels>(COMPLETED_LEVELS)
                    .unwrap_or(CompletedLevels(HashSet::new()));

                completed_levels.0.insert(level_stack.get_current_level().file_name.clone());

                let _ = pkv_store.set::<CompletedLevels>(COMPLETED_LEVELS, &completed_levels);

                // Hard unloads the current level and reloads the last if it exists
                (UnloadType::Hard, if soft_unloaded_query.is_empty() { LoadType::Fresh } else { LoadType::Reload })
            };

            commands.entity(entity).insert(AddTimerForReload);

            level_stack.warp(warp_to);

            menu_state.set(MenuState::WinScreen);
            state.set(AppState::OverlayMenu);

            return;
        }   
    }
}

pub fn activator_events(
    mut activator_query: Query<(&mut Activator, &SensorEvents), With<InGameEntity>>
) {
    for (
        mut activator, 
        SensorEvents(ongoing_events, _)
    ) in activator_query.iter_mut() {
        activator.is_active = ongoing_events.len() > 0;
    }
}

pub fn gravity_sensor_events(
    gravity_sensor_events_query: Query<(&GravitySensorDirection, &SensorEvents), With<InGameEntity>>,
    mut object_events_query: Query<(&mut Gravity, &ObjectEvents), (With<InGameEntity>, Without<Sensor>)>
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
    can_jump_sensor_query: Query<&SensorEvents, (With<CanJumpSensor>, With<InGameEntity>)>,
    mut can_jump: ResMut<CanJump>
) {
    let Ok(sensor_events) = can_jump_sensor_query.get_single() else { return };
    can_jump.0 = sensor_events.0.len() > 0;
}