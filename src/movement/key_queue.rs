use super::*;

#[derive(Resource, Clone, Debug)]
pub struct KeyQueue(pub HashMap<KeyCode, usize>);

const QUEUED_KEYS: [KeyCode; 1] = [
    KeyCode::Space
];

pub fn update_key_queue(
    keys: Res<Input<KeyCode>>,
    mut key_queue: ResMut<KeyQueue>
) {
    for queued_key in QUEUED_KEYS.iter() {
        if keys.just_pressed(*queued_key) {
            key_queue.0.insert(*queued_key, 0);
        }

        if let Some(num_updates_since_press) = key_queue.0.get_mut(queued_key) {
            if *num_updates_since_press >= KEY_QUEUE_LIFESPAN {
                key_queue.0.remove(queued_key);
            } else {
                *num_updates_since_press += 1;
            }
        }
    }
}