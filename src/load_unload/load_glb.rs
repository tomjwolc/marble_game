// use bevy::scene::SceneInstance;

use bevy::gltf::{GltfMesh, GltfNode};
use serde::{Serialize, Deserialize};
use colored::*;

use super::*;

pub fn load_glb_asset(
    asset_server: Res<AssetServer>,
    mut level_stack: ResMut<LevelStack>
) {
    if level_stack.get_current_level().handle.is_none() {
        level_stack.get_current_level_mut().handle = Some(asset_server.load(
            format!("levels/{}.glb", level_stack.get_current_level().file_name).as_str()
        ));
    }
}

pub fn try_load_glb_data(
    mut loaded_glb_data: ResMut<LoadedGlbData>,
    level_stack: Res<LevelStack>,
    gltf_assets: Res<Assets<Gltf>>,
    gltf_node_assets: Res<Assets<GltfNode>>,
    gltf_mesh_assets: Res<Assets<GltfMesh>>,
    mesh_assets: Res<Assets<Mesh>>,
    default_material: Res<DefaultMaterial>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut next_state: ResMut<NextState<AppState>>
) {
    if let Some(gltf) = gltf_assets.get(&level_stack.get_current_level().handle.as_ref().unwrap()) {
        let num_threads = 10;

        if DEBUG_MENUS { println!("Collider loading started"); }

        if DEBUG_GLTF_LOAD { println!("{}", "\nScene:                                  ".underline().bold()) };

        let nodes: Vec<(&String, &Handle<GltfNode>)> = gltf.named_nodes.iter().collect();

        std::thread::scope(|thread_spawner| {
            let threads = nodes
                .chunks(num_threads)
                .map(|node_batch| { thread_spawner.spawn(|| {
                    let mut loaded_glb_objects = Vec::new();

                    for (name, node_handle) in node_batch.into_iter() {
                        /* Continues if the node asset does not exist, the node has no mesh, 
                        or the mesh asset does not exist */
                        extract!(continue; 
                            Some( node ) = gltf_node_assets.get(node_handle);
                            Some( gltf_mesh_handle ) = &node.mesh;
                            Some( gltf_mesh ) = gltf_mesh_assets.get(gltf_mesh_handle)
                        );
            
                        let extras_data: ExtrasData = serde_json::from_str(
                            if let Some(extras) = &node.extras { extras.value.as_str() } else { "{}" }
                        ).unwrap();
            
                        // Most objects only have 1 primitive
                        let mesh = gltf_mesh.primitives[0].mesh.clone();
                        let material = if let Some( 
                            material
                        ) = gltf_mesh.primitives[0].material.clone() {
                            material
                        } else {
                            default_material.0.clone()
                        };

                        let collider = match (
                            &extras_data.shape.as_ref().map(|string| string.as_str()), 
                            &extras_data.collider_dimensions
                        ) {
                            (Some("ignore"), _) => Collider::default(),
                            (Some("cuboid" | "box"), Some(sizes)) => Collider::cuboid(
                                sizes[0] / 2.0, 
                                sizes[1] / 2.0, 
                                sizes[2] / 2.0
                            ),
                            (Some("ball" | "sphere"), Some(sizes)) => Collider::ball(
                                sizes[0] / 2.0
                            ),
                            (Some("cylinder"), Some(sizes)) => Collider::cylinder(
                                sizes[1] / 2.0, 
                                sizes[0] / 2.0
                            ),
                            _ => Collider::from_bevy_mesh(
                                mesh_assets.get(&mesh).unwrap(), 
                                &ComputedColliderShape::ConvexDecomposition(VHACDParameters { 
                                    resolution: 2,
                                    ..Default::default() 
                                })
                            ).unwrap(),
                        };
            
                        let object_type = GltfObjectType::from(extras_data);
                        let mut transform = node.transform;
                        transform.scale *= SCALE;
                        transform.translation *= SCALE;
            
                        if DEBUG_GLTF_LOAD { println!("  {}: {:?} w/ {:?}", name.underline(), object_type, transform); }
                        
                        loaded_glb_objects.push(LoadedGlbObject {
                            object_type,
                            collider,
                            transform,
                            mesh,
                            material
                        });
                    }

                    loaded_glb_objects
                }) }).collect::<Vec<std::thread::ScopedJoinHandle<Vec<LoadedGlbObject>>>>();

            loaded_glb_data.0 = Vec::new();

            for thread in threads.into_iter() {
                loaded_glb_data.0.append(&mut thread.join().expect("Could not join thread"));
            }

            if DEBUG_MENUS { println!("Loading Complete!") }

            menu_state.set(MenuState::None);
            next_state.set(AppState::None);
        });
    }
}

pub fn load_glb(
    mut commands: Commands,
    loaded_glb_data: Res<LoadedGlbData>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut activation_table: ResMut<ActivationTable>
) {
    for LoadedGlbObject { 
        object_type, collider, 
        transform, mesh, material
    } in loaded_glb_data.0.clone().into_iter() {
        println!("{:?}", transform);

        match object_type {
            GltfObjectType::Warp(
                warp_to, 
                activatable
            ) => {
                commands.spawn(WarpBundle {
                    pbr_bundle: PbrBundle {
                        mesh: mesh.clone(),
                        material,
                        transform,
                        ..Default::default()
                    },
                    collider,
                    warp_to: warp_to.clone(),
                    activatable: activatable.clone(),
                    ..Default::default()
                });

                commands.spawn((SensorBundle::new(
                    Collider::cylinder(WARP_SENSOR_HEIGHT / 2.0, 1.0),
                    transform.with_translation(transform.translation + 
                        transform.rotation.mul_vec3(Vec3::Y * SCALE * WARP_SENSOR_HEIGHT / 2.0)), 
                    SensorChannel::Warp
                ), activatable, warp_to));
            },

            GltfObjectType::Activator(
                Activator(id)
            ) => {
                activation_table.0.insert(id, false);

                commands.spawn(PhysicsButtonBundle::new(
                    transform, material, &mut mesh_assets, id
                ));
                
                commands.spawn((SensorBundle::new(
                    Collider::cylinder(BUTTON_SENSOR_HEIGHT / 2.0, BUTTON_RADIUS),
                    transform, SensorChannel::Activator
                ), Activator(id)));
            },

            GltfObjectType::Sensor(
                sensor_channel, 
                gravity_direction_option
            ) => {
                let mut entity_commands = commands.spawn(SensorBundle::new(
                    collider, transform, sensor_channel
                ));

                if let Some(gravity_direction) = gravity_direction_option {
                    entity_commands.insert(GravitySensorDirection(SCALE * Vec3::from_array(gravity_direction)));
                }
            },

            GltfObjectType::Movable(
                mass_properties,
                material_properties
            ) => {
                commands.spawn(MovableBundle::new(
                    mesh, material, transform, collider, 
                    SensorChannel::Respawn.not(),  mass_properties, material_properties
                ));
            },
            
            GltfObjectType::Fixed(
                mass_properties,
                material_properties
            ) => {
                commands.spawn(FixedBundle::new(
                    mesh, material, transform,
                    collider, mass_properties, material_properties
                ));
            },

            _ => {
                commands.spawn(FixedBundle::new(
                    mesh, material, transform,
                    collider, ColliderMassProperties::Mass(1.0), DEFAULT_MATERIAL_PROPERTIES
                ));
            }
        };
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ExtrasData {
    // warp info
        file_name: Option<String>,
        activation_requirements: Option<Vec<usize>>,

    // activator info
        activator_type: Option<String>,
        activator_id: Option<usize>,

    // collider spawner info
        shape: Option<String>,
        collider_dimensions: Option<Vec<f32>>,

    // sensor info
        sensor_channel: Option<String>,
    // optional sensor info
        gravity_direction: Option<[f32; 3]>,

    // object info
        // blender doesn't support bool's as custom properties >:(
        is_dynamic: Option<i8>,
        // Either mass or density with be Some(..)
        mass: Option<f32>,
        density: Option<f32>,
        // default vs ice vs ..
        material_type: Option<String>
}

#[derive(Clone)]
pub enum GltfObjectType {
    Warp(WarpTo, Activatable),
    Activator(Activator),
    Sensor(SensorChannel, Option<[f32; 3]>),
    Movable(ColliderMassProperties, MaterialProperties),
    Fixed(ColliderMassProperties, MaterialProperties),
    Object
}

impl From<ExtrasData> for GltfObjectType {
    fn from(extras_data: ExtrasData) -> Self {
        match extras_data {

            // Warp
            ExtrasData { 
                file_name: Some(file_name), 
                activation_requirements,
                ..
            } => {
                // If activation_requirements is none, then there are no activation requirements
                let requirements = if let Some(requirements) = activation_requirements 
                    { requirements } else { Vec::new() };
                let warp_to = if file_name.len() == 0 { WarpTo::Out } else { WarpTo::File(file_name) };

                // Only starts out active if there are no requirements
                let is_active = requirements.len() == 0;

                GltfObjectType::Warp(
                    warp_to, 
                    Activatable {
                        requirements,
                        is_active
                    }
                )
            }, 
            
            // Activator
            ExtrasData { 
                // activator_type: Some(activator_type),
                activator_id: Some(id),
                .. 
            } => {
                GltfObjectType::Activator(Activator(id))
            },
        
            // Sensor
            ExtrasData {
                sensor_channel: Some(sensor_channel),
                gravity_direction: gravity_direction_option,
                ..
            } => GltfObjectType::Sensor(
                match sensor_channel.to_lowercase().as_str() {
                    "warp" => SensorChannel::Warp,
                    "respawn" => SensorChannel::Respawn,
                    "activator" => SensorChannel::Activator,
                    "gravity" => SensorChannel::Gravity,
                    _ => SensorChannel::none()
                }, 
                gravity_direction_option
            ), 

            // Movable and Fixed
            ExtrasData { 
                is_dynamic: Some(is_dynamic),
                material_type,
                mass,
                density,
                .. 
            } => {
                let material_properties = 
                    MaterialProperties::from(material_type, is_dynamic != 0);

                let mass_properties = if let Some(mass) = mass {
                    ColliderMassProperties::Mass(mass * SCALE)
                } else if let Some(density) = density {
                    ColliderMassProperties::Density(density)
                } else {
                    ColliderMassProperties::Mass(1.0)
                };

                if is_dynamic != 0 {
                    GltfObjectType::Movable(mass_properties, material_properties)
                } else {
                    GltfObjectType::Fixed(mass_properties, material_properties)
                }
            },

            // Regular object
            _ => GltfObjectType::Object
        }
    }
}

impl std::fmt::Debug for GltfObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            GltfObjectType::Warp(warp_to, activatable) => 
                pretty_debug!(GltfObjectType::Warp(warp_to, activatable)),
            GltfObjectType::Activator(activator) => 
                pretty_debug!(GltfObjectType::Activator(activator)),
            GltfObjectType::Sensor( sizes, gravity_direction_option ) => 
                pretty_debug!(GltfObjectType::Sensor( sizes, gravity_direction_option )),
            GltfObjectType::Movable(mass_properties, material_properties) => 
                pretty_debug!(GltfObjectType::Movable(mass_properties, material_properties)),
            GltfObjectType::Fixed(mass_properties, material_properties) => 
                pretty_debug!(GltfObjectType::Fixed(mass_properties, material_properties)),
            GltfObjectType::Object => String::from("Object")
        })
    }
}