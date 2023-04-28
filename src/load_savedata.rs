use serde::{Serialize, Deserialize};

use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletedLevels(pub HashSet<String>);

pub fn load_savedata(
    mut commands: Commands
) {
    let mut pkv_store = PkvStore::new("Twol Games", "The Amazing Marble Game");

    if pkv_store.get::<CompletedLevels>(COMPLETED_LEVELS).is_err() {
        let _ = pkv_store.set::<CompletedLevels>(
            COMPLETED_LEVELS, 
            &CompletedLevels(HashSet::new())
        );
    }

    commands.insert_resource(pkv_store);
}