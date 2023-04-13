use bevy::gltf::Gltf;

use super::*;

#[derive(Resource, Clone)]
pub struct DefaultMaterial(pub Handle<StandardMaterial>);

#[derive(Resource)]
pub struct CanJump(pub bool);

#[derive(Resource)]
pub struct ActivationTable(pub Vec<bool>);

#[derive(Resource)]
pub struct PrevAppState(pub AppState);

/* This should be set whenever the in game elements are about to be
 unloaded */
#[derive(Resource, PartialEq, Eq)]
pub enum UnloadType {
    Soft,
    Hard,
    Complete
}

#[derive(Resource, PartialEq, Eq)]
pub enum LoadType {
    Reload,
    Fresh
}

#[derive(Resource)]
pub struct LoadedGlbData(pub Vec<LoadedGlbObject>);

#[derive(Clone)]
pub struct LoadedGlbObject {
    pub object_type: GltfObjectType,
    pub collider: Collider,
    pub transform: Transform,
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>
}

#[derive(Resource)]
pub struct LevelStack(Vec<Level>);

pub struct Level {
    pub handle: Option<Handle<Gltf>>,
    pub file_name: String
}

// It is assumed that LevelStack will always have at least one element in it
impl LevelStack {
    /* If you want to have a meta world before the overworld you will need to 
     make sure that LoadType can be set differently on WarpTo::Out so meta-world
     can load from asset instead of expecting soft-unloaded entities*/
    pub fn initial_stack() -> Self {
        let mut level_stack = LevelStack::from_level("meta_level");
        level_stack.warp(&WarpTo::File(String::from("test_level")));

        level_stack
    }

    pub fn pop(&mut self) -> Level {
        self.0.pop().unwrap()
    }

    pub fn push(&mut self, level: Level) {
        self.0.push(level);
    }

    pub fn from_level(file_name: &'static str) -> Self {
        Self(vec![Level { handle: None, file_name: file_name.to_owned() }])
    }

    pub fn warp(&mut self, warp_to: &WarpTo) {
        match warp_to {
            WarpTo::File(file_name) => self.push(Level { 
                handle: None, 
                file_name: file_name.clone()
            }), 
            WarpTo::Out => {
                self.pop();
            }
        }
    }

    pub fn get_current_level(&self) -> &Level {
        self.0.last().unwrap()
    }

    pub fn get_current_level_mut(&mut self) -> &mut Level {
        self.0.last_mut().unwrap()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}