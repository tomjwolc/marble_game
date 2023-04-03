use super::*;

#[derive(Bundle)]
pub struct SensorBundle {
    collider: Collider,
    transform_bundle: TransformBundle,
    sensor: Sensor,
    sensor_channel: SensorChannel,
    sensor_events: SensorEvents,
    in_game_entity: InGameEntity
}

impl SensorBundle {
    pub fn new(
        collider: Collider,
        transform: Transform,
        sensor_channel: SensorChannel
    ) -> Self {
        Self {
            collider,
            transform_bundle: TransformBundle::from_transform(transform),
            sensor_channel,
            ..Default::default()
        }
    }
}

impl Default for SensorBundle {
    fn default() -> Self {
        Self {
            collider: Collider::default(),
            transform_bundle: TransformBundle::default(),
            sensor: Sensor,
            sensor_channel: SensorChannel::Respawn,
            sensor_events: SensorEvents::new(),
            in_game_entity: InGameEntity
        }
    }
}