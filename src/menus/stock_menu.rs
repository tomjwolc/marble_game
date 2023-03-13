use std::marker::PhantomData;

pub use super::*;

#[derive(Component)]
pub struct ButtonComponent<T> {
    id: usize,
    __phantom_data: PhantomData<T>
}

#[derive(Debug)]
pub struct StockMenu<T: Component + Clone>{
    title: &'static str,
    buttons: Vec<Button>,
    bg_transparency: f32,
    component: T
}

impl<T: Component + Clone> StockMenu<T> {
    pub fn new(title: &'static str, buttons: Vec<Button>, component: T) -> Self {
        Self { title, buttons, bg_transparency: 1.0, component }
    }

    pub fn new_overlay(title: &'static str, buttons: Vec<Button>, component: T) -> Self {
        Self { 
            title, buttons, 
            bg_transparency: OVERLAY_TRANSPARANCY, 
            component 
        }
    }

    pub fn add_to_app(
        self, 
        menu_scheduler: &mut MenuScheduler, 
        menu_type: MenuType,
        app: &mut App
    ) {
        let button_labels: Vec<&str> = self.buttons
            .iter()
            .filter(|&button| !button.key_press_only)
            .map(|button| button.label)
            .collect();

        add_menu_enter_systems!(
            menu_scheduler:
            menu_type => move |mut commands: Commands, asset_server: Res<AssetServer>| {
                let hover_event = HoverEvent { 
                    color: BUTTON_COLOR, 
                    hover_color: BUTTON_HOVER_COLOR 
                };
            
                let default_button = ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
            
                        padding: BUTTON_PADDING,
                        margin: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    background_color: hover_event.color.into(),
                    ..default()
                };
            
                commands
                    .spawn((NodeBundle {
                        style: Style {
                            size: Size::width(Val::Percent(100.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        background_color: BACKGROUND_COLOR.with_a(self.bg_transparency).into(),
                        ..default()
                    }, self.component.clone()))
                    .with_children(|parent| {
                        // Spawns Title banner
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Auto, Val::Auto),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
            
                                    padding: BUTTON_PADDING,
                                    margin: UiRect::bottom(Val::Px(100.0)),
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    self.title,
                                    TextStyle {
                                        font: asset_server.load(FONT_PATH),
                                        font_size: 100.0,
                                        color: TEXT_COLOR,
                                    },
                                ));
                            });

                        for (i, label) in button_labels.iter().enumerate() {
                            parent
                            .spawn((default_button.clone(), hover_event, ButtonComponent::<T> {
                                id: i,
                                __phantom_data: PhantomData::default()
                            }))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    *label,
                                    TextStyle {
                                        font: asset_server.load(FONT_PATH),
                                        font_size: 40.0,
                                        color: BUTTON_TEXT_COLOR,
                                    },
                                ));
                            });
                        }
                    });
            }
        );

        add_menu_exit_systems!(
            menu_scheduler:
            menu_type => move |entities_query: Query<Entity, With<T>>, mut commands: Commands| {
                for entity in entities_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
            }
        );

        for (i, Button { 
            mut on_click, 
            key_binds, 
            .. 
        }) in self.buttons.into_iter().enumerate() {
            app.add_system(
                // Runs the schedule associated with the button
                (move |world: &mut World| { 
                    if DEBUG_MENUS {
                        println!("{:?} > Button #{} trigged", self.title, i + 1);
                    }
                    on_click.run(world)
                })

                // Checks that we are in the right menu
                .run_if(can_update_menu(menu_type)
                
                // Checks for any of the key presses
                .and_then((move |keys: Res<Input<KeyCode>>| { 
                    key_binds.iter().any(|key| keys.just_pressed(*key)) 
                })
                
                // Checks for button clicks
                .or_else(move |interaction_query: Query<(&Interaction, &ButtonComponent<T>)>,| { 
                    // println!("{:?}", interaction_query);
                    interaction_query.iter().any(|(interaction, ButtonComponent { id, .. })| {
                        // println!("{:?} && {:?} == {:?}", interaction, i, id);
                        *interaction == Interaction::Clicked && i == *id
                    })
                })))
            );
        }
    }
}

pub struct Button {
    label: &'static str,
    key_press_only: bool,
    key_binds: Vec<KeyCode>,
    on_click: Schedule
}

impl std::fmt::Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]: {:?}", self.label, self.key_binds)
    }
}

impl Button {
    pub fn new(label: &'static str, key_binds: Vec<KeyCode>, on_click: Schedule) -> Self {
        Self { label, key_press_only: false, key_binds, on_click }
    }

    pub fn new_key_press_only(key_binds: Vec<KeyCode>, on_click: Schedule) -> Self {
        Self { label: "", key_press_only: true, key_binds, on_click }
    }
}

