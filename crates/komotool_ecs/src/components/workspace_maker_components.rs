use crate::resources::ExtendedMarkerMap;
use bevy_app::App;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::system::Commands;
use bevy_mod_scripting::core::bindings::DynamicComponent;
use bevy_reflect::Reflect;

#[derive(Component, Reflect)]
pub struct Workspace1;

#[derive(Component, Reflect)]
pub struct Workspace2;

#[derive(Component, Reflect)]
pub struct Workspace3;

#[derive(Component, Reflect)]
pub struct Workspace4;

#[derive(Component, Reflect)]
pub struct Workspace5;

#[derive(Component, Reflect)]
pub struct Workspace6;

#[derive(Component, Reflect)]
pub struct Workspace7;

#[derive(Component, Reflect)]
pub struct Workspace8;

#[derive(Component, Reflect)]
pub struct Workspace9;

#[derive(Component, Reflect)]
pub struct Workspace10;

#[derive(Component, Reflect)]
pub struct Workspace11;

#[derive(Component, Reflect)]
pub struct Workspace12;

#[derive(Component, Reflect)]
pub struct Workspace13;

#[derive(Component, Reflect)]
pub struct Workspace14;

#[derive(Component, Reflect)]
pub struct Workspace15;

#[derive(Component, Reflect)]
pub struct Workspace16;

#[derive(Component, Reflect)]
pub struct Workspace17;

#[derive(Component, Reflect)]
pub struct Workspace18;

#[derive(Component, Reflect)]
pub struct Workspace19;

#[derive(Component, Reflect)]
pub struct Workspace20;

#[derive(Component, Reflect)]
pub struct Workspace21;

#[derive(Component, Reflect)]
pub struct Workspace22;

#[derive(Component, Reflect)]
pub struct Workspace23;

#[derive(Component, Reflect)]
pub struct Workspace24;

#[derive(Component, Reflect)]
pub struct Workspace25;

#[derive(Component, Reflect)]
pub struct Workspace26;

#[derive(Component, Reflect)]
pub struct Workspace27;

#[derive(Component, Reflect)]
pub struct Workspace28;

#[derive(Component, Reflect)]
pub struct Workspace29;

#[derive(Component, Reflect)]
pub struct Workspace30;

#[derive(Component, Reflect)]
pub struct Workspace31;

#[derive(Component, Reflect)]
pub struct Workspace32;

#[derive(Component, Reflect)]
pub struct Workspace33;

#[derive(Component, Reflect)]
pub struct Workspace34;

#[derive(Component, Reflect)]
pub struct Workspace35;

#[derive(Component, Reflect)]
pub struct Workspace36;

#[derive(Component, Reflect)]
pub struct Workspace37;

#[derive(Component, Reflect)]
pub struct Workspace38;

#[derive(Component, Reflect)]
pub struct Workspace39;

#[derive(Component, Reflect)]
pub struct Workspace40;

#[derive(Component, Reflect)]
pub struct Workspace41;

#[derive(Component, Reflect)]
pub struct Workspace42;

#[derive(Component, Reflect)]
pub struct Workspace43;

#[derive(Component, Reflect)]
pub struct Workspace44;

#[derive(Component, Reflect)]
pub struct Workspace45;

#[derive(Component, Reflect)]
pub struct Workspace46;

#[derive(Component, Reflect)]
pub struct Workspace47;

#[derive(Component, Reflect)]
pub struct Workspace48;

#[derive(Component, Reflect)]
pub struct Workspace49;

#[derive(Component, Reflect)]
pub struct Workspace50;

#[derive(Component, Reflect)]
pub struct Workspace51;

#[derive(Component, Reflect)]
pub struct Workspace52;

#[derive(Component, Reflect)]
pub struct Workspace53;

#[derive(Component, Reflect)]
pub struct Workspace54;

#[derive(Component, Reflect)]
pub struct Workspace55;

#[derive(Component, Reflect)]
pub struct Workspace56;

#[derive(Component, Reflect)]
pub struct Workspace57;

#[derive(Component, Reflect)]
pub struct Workspace58;

#[derive(Component, Reflect)]
pub struct Workspace59;

#[derive(Component, Reflect)]
pub struct Workspace60;

#[derive(Component, Reflect)]
pub struct Workspace61;

#[derive(Component, Reflect)]
pub struct Workspace62;

#[derive(Component, Reflect)]
pub struct Workspace63;

#[derive(Component, Reflect)]
pub struct Workspace64;

pub fn register_workspace_types(app: &mut App) {
    app.register_type::<Workspace1>()
        .register_type::<Workspace2>()
        .register_type::<Workspace3>()
        .register_type::<Workspace4>()
        .register_type::<Workspace5>()
        .register_type::<Workspace6>()
        .register_type::<Workspace7>()
        .register_type::<Workspace8>()
        .register_type::<Workspace9>()
        .register_type::<Workspace10>()
        .register_type::<Workspace11>()
        .register_type::<Workspace12>()
        .register_type::<Workspace13>()
        .register_type::<Workspace14>()
        .register_type::<Workspace15>()
        .register_type::<Workspace16>()
        .register_type::<Workspace17>()
        .register_type::<Workspace18>()
        .register_type::<Workspace19>()
        .register_type::<Workspace20>()
        .register_type::<Workspace21>()
        .register_type::<Workspace22>()
        .register_type::<Workspace23>()
        .register_type::<Workspace24>()
        .register_type::<Workspace25>()
        .register_type::<Workspace26>()
        .register_type::<Workspace27>()
        .register_type::<Workspace28>()
        .register_type::<Workspace29>()
        .register_type::<Workspace30>()
        .register_type::<Workspace31>()
        .register_type::<Workspace32>()
        .register_type::<Workspace33>()
        .register_type::<Workspace34>()
        .register_type::<Workspace35>()
        .register_type::<Workspace36>()
        .register_type::<Workspace37>()
        .register_type::<Workspace38>()
        .register_type::<Workspace39>()
        .register_type::<Workspace40>()
        .register_type::<Workspace41>()
        .register_type::<Workspace42>()
        .register_type::<Workspace43>()
        .register_type::<Workspace44>()
        .register_type::<Workspace45>()
        .register_type::<Workspace46>()
        .register_type::<Workspace47>()
        .register_type::<Workspace48>()
        .register_type::<Workspace49>()
        .register_type::<Workspace50>()
        .register_type::<Workspace51>()
        .register_type::<Workspace52>()
        .register_type::<Workspace53>()
        .register_type::<Workspace54>()
        .register_type::<Workspace55>()
        .register_type::<Workspace56>()
        .register_type::<Workspace57>()
        .register_type::<Workspace58>()
        .register_type::<Workspace59>()
        .register_type::<Workspace60>()
        .register_type::<Workspace61>()
        .register_type::<Workspace62>()
        .register_type::<Workspace63>()
        .register_type::<Workspace64>();
}

pub fn insert_workspace_marker_component(
    index: usize,
    entity: Entity,
    mut commands: Commands,
    extended_marker_map: &ExtendedMarkerMap,
) {
    match index {
        1 => {
            commands.entity(entity).insert(Workspace1);
        }
        2 => {
            commands.entity(entity).insert(Workspace2);
        }
        3 => {
            commands.entity(entity).insert(Workspace3);
        }
        4 => {
            commands.entity(entity).insert(Workspace4);
        }
        5 => {
            commands.entity(entity).insert(Workspace5);
        }
        6 => {
            commands.entity(entity).insert(Workspace6);
        }
        7 => {
            commands.entity(entity).insert(Workspace7);
        }
        8 => {
            commands.entity(entity).insert(Workspace8);
        }
        9 => {
            commands.entity(entity).insert(Workspace9);
        }
        10 => {
            commands.entity(entity).insert(Workspace10);
        }
        11 => {
            commands.entity(entity).insert(Workspace11);
        }
        12 => {
            commands.entity(entity).insert(Workspace12);
        }
        13 => {
            commands.entity(entity).insert(Workspace13);
        }
        14 => {
            commands.entity(entity).insert(Workspace14);
        }
        15 => {
            commands.entity(entity).insert(Workspace15);
        }
        16 => {
            commands.entity(entity).insert(Workspace16);
        }
        17 => {
            commands.entity(entity).insert(Workspace17);
        }
        18 => {
            commands.entity(entity).insert(Workspace18);
        }
        19 => {
            commands.entity(entity).insert(Workspace19);
        }
        20 => {
            commands.entity(entity).insert(Workspace20);
        }
        21 => {
            commands.entity(entity).insert(Workspace21);
        }
        22 => {
            commands.entity(entity).insert(Workspace22);
        }
        23 => {
            commands.entity(entity).insert(Workspace23);
        }
        24 => {
            commands.entity(entity).insert(Workspace24);
        }
        25 => {
            commands.entity(entity).insert(Workspace25);
        }
        26 => {
            commands.entity(entity).insert(Workspace26);
        }
        27 => {
            commands.entity(entity).insert(Workspace27);
        }
        28 => {
            commands.entity(entity).insert(Workspace28);
        }
        29 => {
            commands.entity(entity).insert(Workspace29);
        }
        30 => {
            commands.entity(entity).insert(Workspace30);
        }
        31 => {
            commands.entity(entity).insert(Workspace31);
        }
        32 => {
            commands.entity(entity).insert(Workspace32);
        }
        33 => {
            commands.entity(entity).insert(Workspace33);
        }
        34 => {
            commands.entity(entity).insert(Workspace34);
        }
        35 => {
            commands.entity(entity).insert(Workspace35);
        }
        36 => {
            commands.entity(entity).insert(Workspace36);
        }
        37 => {
            commands.entity(entity).insert(Workspace37);
        }
        38 => {
            commands.entity(entity).insert(Workspace38);
        }
        39 => {
            commands.entity(entity).insert(Workspace39);
        }
        40 => {
            commands.entity(entity).insert(Workspace40);
        }
        41 => {
            commands.entity(entity).insert(Workspace41);
        }
        42 => {
            commands.entity(entity).insert(Workspace42);
        }
        43 => {
            commands.entity(entity).insert(Workspace43);
        }
        44 => {
            commands.entity(entity).insert(Workspace44);
        }
        45 => {
            commands.entity(entity).insert(Workspace45);
        }
        46 => {
            commands.entity(entity).insert(Workspace46);
        }
        47 => {
            commands.entity(entity).insert(Workspace47);
        }
        48 => {
            commands.entity(entity).insert(Workspace48);
        }
        49 => {
            commands.entity(entity).insert(Workspace49);
        }
        50 => {
            commands.entity(entity).insert(Workspace50);
        }
        51 => {
            commands.entity(entity).insert(Workspace51);
        }
        52 => {
            commands.entity(entity).insert(Workspace52);
        }
        53 => {
            commands.entity(entity).insert(Workspace53);
        }
        54 => {
            commands.entity(entity).insert(Workspace54);
        }
        55 => {
            commands.entity(entity).insert(Workspace55);
        }
        56 => {
            commands.entity(entity).insert(Workspace56);
        }
        57 => {
            commands.entity(entity).insert(Workspace57);
        }
        58 => {
            commands.entity(entity).insert(Workspace58);
        }
        59 => {
            commands.entity(entity).insert(Workspace59);
        }
        60 => {
            commands.entity(entity).insert(Workspace60);
        }
        61 => {
            commands.entity(entity).insert(Workspace61);
        }
        62 => {
            commands.entity(entity).insert(Workspace62);
        }
        63 => {
            commands.entity(entity).insert(Workspace63);
        }
        64 => {
            commands.entity(entity).insert(Workspace64);
        }
        n if n > 64 => unsafe {
            if let Some(component_id) = extended_marker_map.makers.get(&n) {
                commands
                    .entity(entity)
                    .insert_by_id(*component_id, DynamicComponent::default());
            }
        },
        _ => {}
    };
}

pub fn despawn_workspace_marker_component(
    index: usize,
    entity: Entity,
    mut commands: Commands,
    extended_marker_map: &ExtendedMarkerMap,
) {
    match index {
        1 => {
            commands.entity(entity).remove::<Workspace1>();
        }
        2 => {
            commands.entity(entity).remove::<Workspace2>();
        }
        3 => {
            commands.entity(entity).remove::<Workspace3>();
        }
        4 => {
            commands.entity(entity).remove::<Workspace4>();
        }
        5 => {
            commands.entity(entity).remove::<Workspace5>();
        }
        6 => {
            commands.entity(entity).remove::<Workspace6>();
        }
        7 => {
            commands.entity(entity).remove::<Workspace7>();
        }
        8 => {
            commands.entity(entity).remove::<Workspace8>();
        }
        9 => {
            commands.entity(entity).remove::<Workspace9>();
        }
        10 => {
            commands.entity(entity).remove::<Workspace10>();
        }
        11 => {
            commands.entity(entity).remove::<Workspace11>();
        }
        12 => {
            commands.entity(entity).remove::<Workspace12>();
        }
        13 => {
            commands.entity(entity).remove::<Workspace13>();
        }
        14 => {
            commands.entity(entity).remove::<Workspace14>();
        }
        15 => {
            commands.entity(entity).remove::<Workspace15>();
        }
        16 => {
            commands.entity(entity).remove::<Workspace16>();
        }
        17 => {
            commands.entity(entity).remove::<Workspace17>();
        }
        18 => {
            commands.entity(entity).remove::<Workspace18>();
        }
        19 => {
            commands.entity(entity).remove::<Workspace19>();
        }
        20 => {
            commands.entity(entity).remove::<Workspace20>();
        }
        21 => {
            commands.entity(entity).remove::<Workspace21>();
        }
        22 => {
            commands.entity(entity).remove::<Workspace22>();
        }
        23 => {
            commands.entity(entity).remove::<Workspace23>();
        }
        24 => {
            commands.entity(entity).remove::<Workspace24>();
        }
        25 => {
            commands.entity(entity).remove::<Workspace25>();
        }
        26 => {
            commands.entity(entity).remove::<Workspace26>();
        }
        27 => {
            commands.entity(entity).remove::<Workspace27>();
        }
        28 => {
            commands.entity(entity).remove::<Workspace28>();
        }
        29 => {
            commands.entity(entity).remove::<Workspace29>();
        }
        30 => {
            commands.entity(entity).remove::<Workspace30>();
        }
        31 => {
            commands.entity(entity).remove::<Workspace31>();
        }
        32 => {
            commands.entity(entity).remove::<Workspace32>();
        }
        33 => {
            commands.entity(entity).remove::<Workspace33>();
        }
        34 => {
            commands.entity(entity).remove::<Workspace34>();
        }
        35 => {
            commands.entity(entity).remove::<Workspace35>();
        }
        36 => {
            commands.entity(entity).remove::<Workspace36>();
        }
        37 => {
            commands.entity(entity).remove::<Workspace37>();
        }
        38 => {
            commands.entity(entity).remove::<Workspace38>();
        }
        39 => {
            commands.entity(entity).remove::<Workspace39>();
        }
        40 => {
            commands.entity(entity).remove::<Workspace40>();
        }
        41 => {
            commands.entity(entity).remove::<Workspace41>();
        }
        42 => {
            commands.entity(entity).remove::<Workspace42>();
        }
        43 => {
            commands.entity(entity).remove::<Workspace43>();
        }
        44 => {
            commands.entity(entity).remove::<Workspace44>();
        }
        45 => {
            commands.entity(entity).remove::<Workspace45>();
        }
        46 => {
            commands.entity(entity).remove::<Workspace46>();
        }
        47 => {
            commands.entity(entity).remove::<Workspace47>();
        }
        48 => {
            commands.entity(entity).remove::<Workspace48>();
        }
        49 => {
            commands.entity(entity).remove::<Workspace49>();
        }
        50 => {
            commands.entity(entity).remove::<Workspace50>();
        }
        51 => {
            commands.entity(entity).remove::<Workspace51>();
        }
        52 => {
            commands.entity(entity).remove::<Workspace52>();
        }
        53 => {
            commands.entity(entity).remove::<Workspace53>();
        }
        54 => {
            commands.entity(entity).remove::<Workspace54>();
        }
        55 => {
            commands.entity(entity).remove::<Workspace55>();
        }
        56 => {
            commands.entity(entity).remove::<Workspace56>();
        }
        57 => {
            commands.entity(entity).remove::<Workspace57>();
        }
        58 => {
            commands.entity(entity).remove::<Workspace58>();
        }
        59 => {
            commands.entity(entity).remove::<Workspace59>();
        }
        60 => {
            commands.entity(entity).remove::<Workspace60>();
        }
        61 => {
            commands.entity(entity).remove::<Workspace61>();
        }
        62 => {
            commands.entity(entity).remove::<Workspace62>();
        }
        63 => {
            commands.entity(entity).remove::<Workspace63>();
        }
        64 => {
            commands.entity(entity).remove::<Workspace64>();
        }
        n if n > 64 => {
            if let Some(component_id) = extended_marker_map.makers.get(&n) {
                commands.entity(entity).remove_by_id(*component_id);
            } else {
                // Optional: Log warning if needed for index {n}
            }
        }
        _ => {}
    };
}
