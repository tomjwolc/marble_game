use super::*;

pub fn check_detection(
    rapier_context: Res<RapierContext>,
    respawn_sensor_entity_query: Query<Entity, With<RespawnSensor>>,
    player_entity_query: Query<Entity, (With<Player>, Without<RespawnSensor>)>,
    mut states: ResMut<NextState<AppState>>,
    mut menu_scheduler: ResMut<MenuScheduler>
) {
    ignore_extract!(
        Ok(sensor_entity) = respawn_sensor_entity_query.get_single();
        Ok(player_entity) = player_entity_query.get_single()
    );
    
    for (collider1, collider2, is_intersecting) in rapier_context.intersections_with(sensor_entity) {
        if is_intersecting && (player_entity == collider1 || player_entity == collider2) {
            menu_scheduler.set_menu_type(MenuType::DeathScreen);
            states.set(AppState::OverlayMenu);
        }
    }
}