use super::*;

#[derive(Bundle)]
pub struct FixedBundle {
    pbr_bundle: PbrBundle,
    collider: Collider,
    friction: Friction,
    restitution: Restitution,
    mass: ColliderMassProperties,
    sensor_channels: SensorChannel,
    object_events: ObjectEvents,
    rigid_body: RigidBody,
    locked_axes: LockedAxes,
    in_game_entity: InGameEntity
}



impl FixedBundle {
    pub fn new(
        mesh_handle: Handle<Mesh>,
        material_handle: Handle<StandardMaterial>,
        transform: Transform,
        collider: Collider,
        mass: ColliderMassProperties,
        material_properties: MaterialProperties
    ) -> Self {
        Self {
            pbr_bundle: PbrBundle {
                mesh: mesh_handle,
                material: material_handle,
                transform,
                ..Default::default()
            },
            collider,
            mass,
            friction: material_properties.friction,
            restitution: material_properties.restitution,
            ..Default::default()
        }
    }
}

impl Default for FixedBundle {
    fn default() -> Self {
        Self {
            pbr_bundle: PbrBundle::default(),
            collider: Collider::default(),
            friction: MOVABLE_FRICTION,
            restitution: MOVABLE_RESTITUTION,
            mass: ColliderMassProperties::Mass(1.0),
            sensor_channels: SensorChannel::CanJump,
            object_events: ObjectEvents::new(),
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::all(),
            in_game_entity: InGameEntity
        }
    }
}