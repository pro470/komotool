use crate::resources::ExtendedMarkerMap;
use bevy_app::App;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::system::Commands;
use bevy_mod_scripting::core::bindings::DynamicComponent;
use bevy_reflect::Reflect;

#[derive(Component, Reflect)]
pub struct Monitor1;

#[derive(Component, Reflect)]
pub struct Monitor2;

#[derive(Component, Reflect)]
pub struct Monitor3;

#[derive(Component, Reflect)]
pub struct Monitor4;

#[derive(Component, Reflect)]
pub struct Monitor5;

#[derive(Component, Reflect)]
pub struct Monitor6;

#[derive(Component, Reflect)]
pub struct Monitor7;

#[derive(Component, Reflect)]
pub struct Monitor8;

#[derive(Component, Reflect)]
pub struct Monitor9;

#[derive(Component, Reflect)]
pub struct Monitor10;

#[derive(Component, Reflect)]
pub struct Monitor11;

#[derive(Component, Reflect)]
pub struct Monitor12;

#[derive(Component, Reflect)]
pub struct Monitor13;

#[derive(Component, Reflect)]
pub struct Monitor14;

#[derive(Component, Reflect)]
pub struct Monitor15;

#[derive(Component, Reflect)]
pub struct Monitor16;

#[derive(Component, Reflect)]
pub struct Monitor17;

#[derive(Component, Reflect)]
pub struct Monitor18;

#[derive(Component, Reflect)]
pub struct Monitor19;

#[derive(Component, Reflect)]
pub struct Monitor20;

#[derive(Component, Reflect)]
pub struct Monitor21;

#[derive(Component, Reflect)]
pub struct Monitor22;

#[derive(Component, Reflect)]
pub struct Monitor23;

#[derive(Component, Reflect)]
pub struct Monitor24;

#[derive(Component, Reflect)]
pub struct Monitor25;

#[derive(Component, Reflect)]
pub struct Monitor26;

#[derive(Component, Reflect)]
pub struct Monitor27;

#[derive(Component, Reflect)]
pub struct Monitor28;

#[derive(Component, Reflect)]
pub struct Monitor29;

#[derive(Component, Reflect)]
pub struct Monitor30;

#[derive(Component, Reflect)]
pub struct Monitor31;

#[derive(Component, Reflect)]
pub struct Monitor32;

pub fn register_monitor_types(app: &mut App) {
    app.register_type::<Monitor1>()
        .register_type::<Monitor2>()
        .register_type::<Monitor3>()
        .register_type::<Monitor4>()
        .register_type::<Monitor5>()
        .register_type::<Monitor6>()
        .register_type::<Monitor7>()
        .register_type::<Monitor8>()
        .register_type::<Monitor9>()
        .register_type::<Monitor10>()
        .register_type::<Monitor11>()
        .register_type::<Monitor12>()
        .register_type::<Monitor13>()
        .register_type::<Monitor14>()
        .register_type::<Monitor15>()
        .register_type::<Monitor16>()
        .register_type::<Monitor17>()
        .register_type::<Monitor18>()
        .register_type::<Monitor19>()
        .register_type::<Monitor20>()
        .register_type::<Monitor21>()
        .register_type::<Monitor22>()
        .register_type::<Monitor23>()
        .register_type::<Monitor24>()
        .register_type::<Monitor25>()
        .register_type::<Monitor26>()
        .register_type::<Monitor27>()
        .register_type::<Monitor28>()
        .register_type::<Monitor29>()
        .register_type::<Monitor30>()
        .register_type::<Monitor31>()
        .register_type::<Monitor32>();
}

pub fn insert_monitor_marker_component(
    index: usize,
    entity: Entity,
    mut commands: Commands,
    extended_marker_map: &ExtendedMarkerMap,
) {
    match index {
        1 => {
            commands.entity(entity).insert(Monitor1);
        }
        2 => {
            commands.entity(entity).insert(Monitor2);
        }
        3 => {
            commands.entity(entity).insert(Monitor3);
        }
        4 => {
            commands.entity(entity).insert(Monitor4);
        }
        5 => {
            commands.entity(entity).insert(Monitor5);
        }
        6 => {
            commands.entity(entity).insert(Monitor6);
        }
        7 => {
            commands.entity(entity).insert(Monitor7);
        }
        8 => {
            commands.entity(entity).insert(Monitor8);
        }
        9 => {
            commands.entity(entity).insert(Monitor9);
        }
        10 => {
            commands.entity(entity).insert(Monitor10);
        }
        11 => {
            commands.entity(entity).insert(Monitor11);
        }
        12 => {
            commands.entity(entity).insert(Monitor12);
        }
        13 => {
            commands.entity(entity).insert(Monitor13);
        }
        14 => {
            commands.entity(entity).insert(Monitor14);
        }
        15 => {
            commands.entity(entity).insert(Monitor15);
        }
        16 => {
            commands.entity(entity).insert(Monitor16);
        }
        17 => {
            commands.entity(entity).insert(Monitor17);
        }
        18 => {
            commands.entity(entity).insert(Monitor18);
        }
        19 => {
            commands.entity(entity).insert(Monitor19);
        }
        20 => {
            commands.entity(entity).insert(Monitor20);
        }
        21 => {
            commands.entity(entity).insert(Monitor21);
        }
        22 => {
            commands.entity(entity).insert(Monitor22);
        }
        23 => {
            commands.entity(entity).insert(Monitor23);
        }
        24 => {
            commands.entity(entity).insert(Monitor24);
        }
        25 => {
            commands.entity(entity).insert(Monitor25);
        }
        26 => {
            commands.entity(entity).insert(Monitor26);
        }
        27 => {
            commands.entity(entity).insert(Monitor27);
        }
        28 => {
            commands.entity(entity).insert(Monitor28);
        }
        29 => {
            commands.entity(entity).insert(Monitor29);
        }
        30 => {
            commands.entity(entity).insert(Monitor30);
        }
        31 => {
            commands.entity(entity).insert(Monitor31);
        }
        32 => {
            commands.entity(entity).insert(Monitor32);
        }
        n if n > 32 => unsafe {
            if let Some(component_id) = extended_marker_map.makers.get(&n) {
                commands
                    .entity(entity)
                    .insert_by_id(*component_id, DynamicComponent::default());
            }
        },
        _ => {}
    };
}

pub fn despawn_monitor_marker_component(
    index: usize,
    entity: Entity,
    mut commands: Commands,
    extended_marker_map: &ExtendedMarkerMap,
) {
    match index {
        1 => {
            commands.entity(entity).remove::<Monitor1>();
        }
        2 => {
            commands.entity(entity).remove::<Monitor2>();
        }
        3 => {
            commands.entity(entity).remove::<Monitor3>();
        }
        4 => {
            commands.entity(entity).remove::<Monitor4>();
        }
        5 => {
            commands.entity(entity).remove::<Monitor5>();
        }
        6 => {
            commands.entity(entity).remove::<Monitor6>();
        }
        7 => {
            commands.entity(entity).remove::<Monitor7>();
        }
        8 => {
            commands.entity(entity).remove::<Monitor8>();
        }
        9 => {
            commands.entity(entity).remove::<Monitor9>();
        }
        10 => {
            commands.entity(entity).remove::<Monitor10>();
        }
        11 => {
            commands.entity(entity).remove::<Monitor11>();
        }
        12 => {
            commands.entity(entity).remove::<Monitor12>();
        }
        13 => {
            commands.entity(entity).remove::<Monitor13>();
        }
        14 => {
            commands.entity(entity).remove::<Monitor14>();
        }
        15 => {
            commands.entity(entity).remove::<Monitor15>();
        }
        16 => {
            commands.entity(entity).remove::<Monitor16>();
        }
        17 => {
            commands.entity(entity).remove::<Monitor17>();
        }
        18 => {
            commands.entity(entity).remove::<Monitor18>();
        }
        19 => {
            commands.entity(entity).remove::<Monitor19>();
        }
        20 => {
            commands.entity(entity).remove::<Monitor20>();
        }
        21 => {
            commands.entity(entity).remove::<Monitor21>();
        }
        22 => {
            commands.entity(entity).remove::<Monitor22>();
        }
        23 => {
            commands.entity(entity).remove::<Monitor23>();
        }
        24 => {
            commands.entity(entity).remove::<Monitor24>();
        }
        25 => {
            commands.entity(entity).remove::<Monitor25>();
        }
        26 => {
            commands.entity(entity).remove::<Monitor26>();
        }
        27 => {
            commands.entity(entity).remove::<Monitor27>();
        }
        28 => {
            commands.entity(entity).remove::<Monitor28>();
        }
        29 => {
            commands.entity(entity).remove::<Monitor29>();
        }
        30 => {
            commands.entity(entity).remove::<Monitor30>();
        }
        31 => {
            commands.entity(entity).remove::<Monitor31>();
        }
        32 => {
            commands.entity(entity).remove::<Monitor32>();
        }
        n if n > 32 => {
            if let Some(component_id) = extended_marker_map.makers.get(&n) {
                commands.entity(entity).remove_by_id(*component_id);
            } else {
                // Optional: Log warning if needed for index {n}
            }
        }
        _ => {}
    };
}
