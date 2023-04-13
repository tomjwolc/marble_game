use super::*;

pub fn reload(
    mut soft_unloaded_entities_query: Query<(Entity, Option<&mut Visibility>, Option<&Transform>, &SoftUnloadedData)>,
    level_stack: Res<LevelStack>,
    mut commands: Commands,
    time: Res<Time>
) {
    if DEBUG_MENUS || DEBUG_LOAD { println!("Reloading previous level") }

    for (
        entity, 
        visibility_option,
        transform_option,
        SoftUnloadedData { 
            id, 
            collider_option, 
            rigid_body_option,
            give_lifetime
        }
    ) in soft_unloaded_entities_query.iter_mut() {
        if *id != level_stack.len() { continue };
        if DEBUG_LOAD { println!("{:?} -- {:?}", transform_option, collider_option); }

        let entity_commands = &mut commands.entity(entity);

        entity_commands.remove::<SoftUnloadedData>();
        entity_commands.insert(InGameEntity);

        if let Some(mut visibility) = visibility_option {
            *visibility = Visibility::Inherited;
        }
        
        if let Some(collider) = collider_option {
            entity_commands.insert(collider.clone());
        }
        
        if let Some(rigid_body) = rigid_body_option {
            entity_commands.insert(rigid_body.clone());
        }

        if let Some(lifetime) = give_lifetime {
            entity_commands.insert(Timed { 
                lifetime: *lifetime, 
                spawn_time: time.elapsed() 
            });
        }
    }
}

pub fn immediate_exit_loading(
    mut menu_state: ResMut<NextState<MenuState>>,
    mut next_state: ResMut<NextState<AppState>>
) {
    menu_state.set(MenuState::None);
    next_state.set(AppState::None);
}