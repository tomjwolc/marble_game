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
    pub fn with_sensor_channel(mut self, sensor_channel: SensorChannel) -> Self {
        self.sensor_channel = sensor_channel;

        self
    }
}

impl FromShape for SensorBundle {
    fn from_collider(collider: Collider) -> Self {
        Self { 
            collider, 
            ..Default::default() 
        }
    }

    fn with_transform(mut self, transform: Transform) -> Self {
        self.transform_bundle = TransformBundle::from_transform(transform);

        self
    }

    fn with_pbr_bundle(self, _: PbrBundle) -> Self {
        self
    }
}

impl Default for SensorBundle {
    fn default() -> Self {
        Self {
            collider: Collider::default(),
            transform_bundle: TransformBundle::default(),
            sensor: Sensor,
            sensor_channel: SensorChannel::Respawn,
            sensor_events: SensorEvents { 
                ongoing_events: HashSet::new()
            },
            in_game_entity: InGameEntity
        }
    }
}