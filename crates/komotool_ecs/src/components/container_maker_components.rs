use crate::resources::ContainerExtendedMarkerMap;
use bevy_app::App;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::system::Commands;
use bevy_mod_scripting::core::bindings::DynamicComponent;
use bevy_reflect::Reflect;

#[derive(Component, Reflect)]
pub struct Container1;

#[derive(Component, Reflect)]
pub struct Container2;

#[derive(Component, Reflect)]
pub struct Container3;

#[derive(Component, Reflect)]
pub struct Container4;

#[derive(Component, Reflect)]
pub struct Container5;

#[derive(Component, Reflect)]
pub struct Container6;

#[derive(Component, Reflect)]
pub struct Container7;

#[derive(Component, Reflect)]
pub struct Container8;

#[derive(Component, Reflect)]
pub struct Container9;

#[derive(Component, Reflect)]
pub struct Container10;

#[derive(Component, Reflect)]
pub struct Container11;

#[derive(Component, Reflect)]
pub struct Container12;

#[derive(Component, Reflect)]
pub struct Container13;

#[derive(Component, Reflect)]
pub struct Container14;

#[derive(Component, Reflect)]
pub struct Container15;

#[derive(Component, Reflect)]
pub struct Container16;

#[derive(Component, Reflect)]
pub struct Container17;

#[derive(Component, Reflect)]
pub struct Container18;

#[derive(Component, Reflect)]
pub struct Container19;

#[derive(Component, Reflect)]
pub struct Container20;

#[derive(Component, Reflect)]
pub struct Container21;

#[derive(Component, Reflect)]
pub struct Container22;

#[derive(Component, Reflect)]
pub struct Container23;

#[derive(Component, Reflect)]
pub struct Container24;

#[derive(Component, Reflect)]
pub struct Container25;

#[derive(Component, Reflect)]
pub struct Container26;

#[derive(Component, Reflect)]
pub struct Container27;

#[derive(Component, Reflect)]
pub struct Container28;

#[derive(Component, Reflect)]
pub struct Container29;

#[derive(Component, Reflect)]
pub struct Container30;

#[derive(Component, Reflect)]
pub struct Container31;

#[derive(Component, Reflect)]
pub struct Container32;

#[derive(Component, Reflect)]
pub struct Container33;

#[derive(Component, Reflect)]
pub struct Container34;

#[derive(Component, Reflect)]
pub struct Container35;

#[derive(Component, Reflect)]
pub struct Container36;

#[derive(Component, Reflect)]
pub struct Container37;

#[derive(Component, Reflect)]
pub struct Container38;

#[derive(Component, Reflect)]
pub struct Container39;

#[derive(Component, Reflect)]
pub struct Container40;

#[derive(Component, Reflect)]
pub struct Container41;

#[derive(Component, Reflect)]
pub struct Container42;

#[derive(Component, Reflect)]
pub struct Container43;

#[derive(Component, Reflect)]
pub struct Container44;

#[derive(Component, Reflect)]
pub struct Container45;

#[derive(Component, Reflect)]
pub struct Container46;

#[derive(Component, Reflect)]
pub struct Container47;

#[derive(Component, Reflect)]
pub struct Container48;

#[derive(Component, Reflect)]
pub struct Container49;

#[derive(Component, Reflect)]
pub struct Container50;

#[derive(Component, Reflect)]
pub struct Container51;

#[derive(Component, Reflect)]
pub struct Container52;

#[derive(Component, Reflect)]
pub struct Container53;

#[derive(Component, Reflect)]
pub struct Container54;

#[derive(Component, Reflect)]
pub struct Container55;

#[derive(Component, Reflect)]
pub struct Container56;

#[derive(Component, Reflect)]
pub struct Container57;

#[derive(Component, Reflect)]
pub struct Container58;

#[derive(Component, Reflect)]
pub struct Container59;

#[derive(Component, Reflect)]
pub struct Container60;

#[derive(Component, Reflect)]
pub struct Container61;

#[derive(Component, Reflect)]
pub struct Container62;

#[derive(Component, Reflect)]
pub struct Container63;

#[derive(Component, Reflect)]
pub struct Container64;

#[derive(Component, Reflect)]
pub struct Container65;

#[derive(Component, Reflect)]
pub struct Container66;

#[derive(Component, Reflect)]
pub struct Container67;

#[derive(Component, Reflect)]
pub struct Container68;

#[derive(Component, Reflect)]
pub struct Container69;

#[derive(Component, Reflect)]
pub struct Container70;

#[derive(Component, Reflect)]
pub struct Container71;

#[derive(Component, Reflect)]
pub struct Container72;

#[derive(Component, Reflect)]
pub struct Container73;

#[derive(Component, Reflect)]
pub struct Container74;

#[derive(Component, Reflect)]
pub struct Container75;

#[derive(Component, Reflect)]
pub struct Container76;

#[derive(Component, Reflect)]
pub struct Container77;

#[derive(Component, Reflect)]
pub struct Container78;

#[derive(Component, Reflect)]
pub struct Container79;

#[derive(Component, Reflect)]
pub struct Container80;

#[derive(Component, Reflect)]
pub struct Container81;

#[derive(Component, Reflect)]
pub struct Container82;

#[derive(Component, Reflect)]
pub struct Container83;

#[derive(Component, Reflect)]
pub struct Container84;

#[derive(Component, Reflect)]
pub struct Container85;

#[derive(Component, Reflect)]
pub struct Container86;

#[derive(Component, Reflect)]
pub struct Container87;

#[derive(Component, Reflect)]
pub struct Container88;

#[derive(Component, Reflect)]
pub struct Container89;

#[derive(Component, Reflect)]
pub struct Container90;

#[derive(Component, Reflect)]
pub struct Container91;

#[derive(Component, Reflect)]
pub struct Container92;

#[derive(Component, Reflect)]
pub struct Container93;

#[derive(Component, Reflect)]
pub struct Container94;

#[derive(Component, Reflect)]
pub struct Container95;

#[derive(Component, Reflect)]
pub struct Container96;

#[derive(Component, Reflect)]
pub struct Container97;

#[derive(Component, Reflect)]
pub struct Container98;

#[derive(Component, Reflect)]
pub struct Container99;

#[derive(Component, Reflect)]
pub struct Container100;

#[derive(Component, Reflect)]
pub struct Container101;

#[derive(Component, Reflect)]
pub struct Container102;

#[derive(Component, Reflect)]
pub struct Container103;

#[derive(Component, Reflect)]
pub struct Container104;

#[derive(Component, Reflect)]
pub struct Container105;

#[derive(Component, Reflect)]
pub struct Container106;

#[derive(Component, Reflect)]
pub struct Container107;

#[derive(Component, Reflect)]
pub struct Container108;

#[derive(Component, Reflect)]
pub struct Container109;

#[derive(Component, Reflect)]
pub struct Container110;

#[derive(Component, Reflect)]
pub struct Container111;

#[derive(Component, Reflect)]
pub struct Container112;

#[derive(Component, Reflect)]
pub struct Container113;

#[derive(Component, Reflect)]
pub struct Container114;

#[derive(Component, Reflect)]
pub struct Container115;

#[derive(Component, Reflect)]
pub struct Container116;

#[derive(Component, Reflect)]
pub struct Container117;

#[derive(Component, Reflect)]
pub struct Container118;

#[derive(Component, Reflect)]
pub struct Container119;

#[derive(Component, Reflect)]
pub struct Container120;

#[derive(Component, Reflect)]
pub struct Container121;

#[derive(Component, Reflect)]
pub struct Container122;

#[derive(Component, Reflect)]
pub struct Container123;

#[derive(Component, Reflect)]
pub struct Container124;

#[derive(Component, Reflect)]
pub struct Container125;

#[derive(Component, Reflect)]
pub struct Container126;

#[derive(Component, Reflect)]
pub struct Container127;

#[derive(Component, Reflect)]
pub struct Container128;

pub fn register_container_types(app: &mut App) {
    app.register_type::<Container1>()
        .register_type::<Container2>()
        .register_type::<Container3>()
        .register_type::<Container4>()
        .register_type::<Container5>()
        .register_type::<Container6>()
        .register_type::<Container7>()
        .register_type::<Container8>()
        .register_type::<Container9>()
        .register_type::<Container10>()
        .register_type::<Container11>()
        .register_type::<Container12>()
        .register_type::<Container13>()
        .register_type::<Container14>()
        .register_type::<Container15>()
        .register_type::<Container16>()
        .register_type::<Container17>()
        .register_type::<Container18>()
        .register_type::<Container19>()
        .register_type::<Container20>()
        .register_type::<Container21>()
        .register_type::<Container22>()
        .register_type::<Container23>()
        .register_type::<Container24>()
        .register_type::<Container25>()
        .register_type::<Container26>()
        .register_type::<Container27>()
        .register_type::<Container28>()
        .register_type::<Container29>()
        .register_type::<Container30>()
        .register_type::<Container31>()
        .register_type::<Container32>()
        .register_type::<Container33>()
        .register_type::<Container34>()
        .register_type::<Container35>()
        .register_type::<Container36>()
        .register_type::<Container37>()
        .register_type::<Container38>()
        .register_type::<Container39>()
        .register_type::<Container40>()
        .register_type::<Container41>()
        .register_type::<Container42>()
        .register_type::<Container43>()
        .register_type::<Container44>()
        .register_type::<Container45>()
        .register_type::<Container46>()
        .register_type::<Container47>()
        .register_type::<Container48>()
        .register_type::<Container49>()
        .register_type::<Container50>()
        .register_type::<Container51>()
        .register_type::<Container52>()
        .register_type::<Container53>()
        .register_type::<Container54>()
        .register_type::<Container55>()
        .register_type::<Container56>()
        .register_type::<Container57>()
        .register_type::<Container58>()
        .register_type::<Container59>()
        .register_type::<Container60>()
        .register_type::<Container61>()
        .register_type::<Container62>()
        .register_type::<Container63>()
        .register_type::<Container64>()
        .register_type::<Container65>()
        .register_type::<Container66>()
        .register_type::<Container67>()
        .register_type::<Container68>()
        .register_type::<Container69>()
        .register_type::<Container70>()
        .register_type::<Container71>()
        .register_type::<Container72>()
        .register_type::<Container73>()
        .register_type::<Container74>()
        .register_type::<Container75>()
        .register_type::<Container76>()
        .register_type::<Container77>()
        .register_type::<Container78>()
        .register_type::<Container79>()
        .register_type::<Container80>()
        .register_type::<Container81>()
        .register_type::<Container82>()
        .register_type::<Container83>()
        .register_type::<Container84>()
        .register_type::<Container85>()
        .register_type::<Container86>()
        .register_type::<Container87>()
        .register_type::<Container88>()
        .register_type::<Container89>()
        .register_type::<Container90>()
        .register_type::<Container91>()
        .register_type::<Container92>()
        .register_type::<Container93>()
        .register_type::<Container94>()
        .register_type::<Container95>()
        .register_type::<Container96>()
        .register_type::<Container97>()
        .register_type::<Container98>()
        .register_type::<Container99>()
        .register_type::<Container100>()
        .register_type::<Container101>()
        .register_type::<Container102>()
        .register_type::<Container103>()
        .register_type::<Container104>()
        .register_type::<Container105>()
        .register_type::<Container106>()
        .register_type::<Container107>()
        .register_type::<Container108>()
        .register_type::<Container109>()
        .register_type::<Container110>()
        .register_type::<Container111>()
        .register_type::<Container112>()
        .register_type::<Container113>()
        .register_type::<Container114>()
        .register_type::<Container115>()
        .register_type::<Container116>()
        .register_type::<Container117>()
        .register_type::<Container118>()
        .register_type::<Container119>()
        .register_type::<Container120>()
        .register_type::<Container121>()
        .register_type::<Container122>()
        .register_type::<Container123>()
        .register_type::<Container124>()
        .register_type::<Container125>()
        .register_type::<Container126>()
        .register_type::<Container127>()
        .register_type::<Container128>();
}

pub fn insert_container_marker_component(
    index: usize,
    entity: Entity,
    mut commands: Commands,
    extended_marker_map: &ContainerExtendedMarkerMap,
) {
    match index {
        1 => {
            commands.entity(entity).insert(Container1);
        }
        2 => {
            commands.entity(entity).insert(Container2);
        }
        3 => {
            commands.entity(entity).insert(Container3);
        }
        4 => {
            commands.entity(entity).insert(Container4);
        }
        5 => {
            commands.entity(entity).insert(Container5);
        }
        6 => {
            commands.entity(entity).insert(Container6);
        }
        7 => {
            commands.entity(entity).insert(Container7);
        }
        8 => {
            commands.entity(entity).insert(Container8);
        }
        9 => {
            commands.entity(entity).insert(Container9);
        }
        10 => {
            commands.entity(entity).insert(Container10);
        }
        11 => {
            commands.entity(entity).insert(Container11);
        }
        12 => {
            commands.entity(entity).insert(Container12);
        }
        13 => {
            commands.entity(entity).insert(Container13);
        }
        14 => {
            commands.entity(entity).insert(Container14);
        }
        15 => {
            commands.entity(entity).insert(Container15);
        }
        16 => {
            commands.entity(entity).insert(Container16);
        }
        17 => {
            commands.entity(entity).insert(Container17);
        }
        18 => {
            commands.entity(entity).insert(Container18);
        }
        19 => {
            commands.entity(entity).insert(Container19);
        }
        20 => {
            commands.entity(entity).insert(Container20);
        }
        21 => {
            commands.entity(entity).insert(Container21);
        }
        22 => {
            commands.entity(entity).insert(Container22);
        }
        23 => {
            commands.entity(entity).insert(Container23);
        }
        24 => {
            commands.entity(entity).insert(Container24);
        }
        25 => {
            commands.entity(entity).insert(Container25);
        }
        26 => {
            commands.entity(entity).insert(Container26);
        }
        27 => {
            commands.entity(entity).insert(Container27);
        }
        28 => {
            commands.entity(entity).insert(Container28);
        }
        29 => {
            commands.entity(entity).insert(Container29);
        }
        30 => {
            commands.entity(entity).insert(Container30);
        }
        31 => {
            commands.entity(entity).insert(Container31);
        }
        32 => {
            commands.entity(entity).insert(Container32);
        }
        33 => {
            commands.entity(entity).insert(Container33);
        }
        34 => {
            commands.entity(entity).insert(Container34);
        }
        35 => {
            commands.entity(entity).insert(Container35);
        }
        36 => {
            commands.entity(entity).insert(Container36);
        }
        37 => {
            commands.entity(entity).insert(Container37);
        }
        38 => {
            commands.entity(entity).insert(Container38);
        }
        39 => {
            commands.entity(entity).insert(Container39);
        }
        40 => {
            commands.entity(entity).insert(Container40);
        }
        41 => {
            commands.entity(entity).insert(Container41);
        }
        42 => {
            commands.entity(entity).insert(Container42);
        }
        43 => {
            commands.entity(entity).insert(Container43);
        }
        44 => {
            commands.entity(entity).insert(Container44);
        }
        45 => {
            commands.entity(entity).insert(Container45);
        }
        46 => {
            commands.entity(entity).insert(Container46);
        }
        47 => {
            commands.entity(entity).insert(Container47);
        }
        48 => {
            commands.entity(entity).insert(Container48);
        }
        49 => {
            commands.entity(entity).insert(Container49);
        }
        50 => {
            commands.entity(entity).insert(Container50);
        }
        51 => {
            commands.entity(entity).insert(Container51);
        }
        52 => {
            commands.entity(entity).insert(Container52);
        }
        53 => {
            commands.entity(entity).insert(Container53);
        }
        54 => {
            commands.entity(entity).insert(Container54);
        }
        55 => {
            commands.entity(entity).insert(Container55);
        }
        56 => {
            commands.entity(entity).insert(Container56);
        }
        57 => {
            commands.entity(entity).insert(Container57);
        }
        58 => {
            commands.entity(entity).insert(Container58);
        }
        59 => {
            commands.entity(entity).insert(Container59);
        }
        60 => {
            commands.entity(entity).insert(Container60);
        }
        61 => {
            commands.entity(entity).insert(Container61);
        }
        62 => {
            commands.entity(entity).insert(Container62);
        }
        63 => {
            commands.entity(entity).insert(Container63);
        }
        64 => {
            commands.entity(entity).insert(Container64);
        }
        65 => {
            commands.entity(entity).insert(Container65);
        }
        66 => {
            commands.entity(entity).insert(Container66);
        }
        67 => {
            commands.entity(entity).insert(Container67);
        }
        68 => {
            commands.entity(entity).insert(Container68);
        }
        69 => {
            commands.entity(entity).insert(Container69);
        }
        70 => {
            commands.entity(entity).insert(Container70);
        }
        71 => {
            commands.entity(entity).insert(Container71);
        }
        72 => {
            commands.entity(entity).insert(Container72);
        }
        73 => {
            commands.entity(entity).insert(Container73);
        }
        74 => {
            commands.entity(entity).insert(Container74);
        }
        75 => {
            commands.entity(entity).insert(Container75);
        }
        76 => {
            commands.entity(entity).insert(Container76);
        }
        77 => {
            commands.entity(entity).insert(Container77);
        }
        78 => {
            commands.entity(entity).insert(Container78);
        }
        79 => {
            commands.entity(entity).insert(Container79);
        }
        80 => {
            commands.entity(entity).insert(Container80);
        }
        81 => {
            commands.entity(entity).insert(Container81);
        }
        82 => {
            commands.entity(entity).insert(Container82);
        }
        83 => {
            commands.entity(entity).insert(Container83);
        }
        84 => {
            commands.entity(entity).insert(Container84);
        }
        85 => {
            commands.entity(entity).insert(Container85);
        }
        86 => {
            commands.entity(entity).insert(Container86);
        }
        87 => {
            commands.entity(entity).insert(Container87);
        }
        88 => {
            commands.entity(entity).insert(Container88);
        }
        89 => {
            commands.entity(entity).insert(Container89);
        }
        90 => {
            commands.entity(entity).insert(Container90);
        }
        91 => {
            commands.entity(entity).insert(Container91);
        }
        92 => {
            commands.entity(entity).insert(Container92);
        }
        93 => {
            commands.entity(entity).insert(Container93);
        }
        94 => {
            commands.entity(entity).insert(Container94);
        }
        95 => {
            commands.entity(entity).insert(Container95);
        }
        96 => {
            commands.entity(entity).insert(Container96);
        }
        97 => {
            commands.entity(entity).insert(Container97);
        }
        98 => {
            commands.entity(entity).insert(Container98);
        }
        99 => {
            commands.entity(entity).insert(Container99);
        }
        100 => {
            commands.entity(entity).insert(Container100);
        }
        101 => {
            commands.entity(entity).insert(Container101);
        }
        102 => {
            commands.entity(entity).insert(Container102);
        }
        103 => {
            commands.entity(entity).insert(Container103);
        }
        104 => {
            commands.entity(entity).insert(Container104);
        }
        105 => {
            commands.entity(entity).insert(Container105);
        }
        106 => {
            commands.entity(entity).insert(Container106);
        }
        107 => {
            commands.entity(entity).insert(Container107);
        }
        108 => {
            commands.entity(entity).insert(Container108);
        }
        109 => {
            commands.entity(entity).insert(Container109);
        }
        110 => {
            commands.entity(entity).insert(Container110);
        }
        111 => {
            commands.entity(entity).insert(Container111);
        }
        112 => {
            commands.entity(entity).insert(Container112);
        }
        113 => {
            commands.entity(entity).insert(Container113);
        }
        114 => {
            commands.entity(entity).insert(Container114);
        }
        115 => {
            commands.entity(entity).insert(Container115);
        }
        116 => {
            commands.entity(entity).insert(Container116);
        }
        117 => {
            commands.entity(entity).insert(Container117);
        }
        118 => {
            commands.entity(entity).insert(Container118);
        }
        119 => {
            commands.entity(entity).insert(Container119);
        }
        120 => {
            commands.entity(entity).insert(Container120);
        }
        121 => {
            commands.entity(entity).insert(Container121);
        }
        122 => {
            commands.entity(entity).insert(Container122);
        }
        123 => {
            commands.entity(entity).insert(Container123);
        }
        124 => {
            commands.entity(entity).insert(Container124);
        }
        125 => {
            commands.entity(entity).insert(Container125);
        }
        126 => {
            commands.entity(entity).insert(Container126);
        }
        127 => {
            commands.entity(entity).insert(Container127);
        }
        128 => {
            commands.entity(entity).insert(Container128);
        }
        n if n > 128 => unsafe {
            if let Some(component_id) = extended_marker_map.makers.get(&n) {
                commands
                    .entity(entity)
                    .insert_by_id(*component_id, DynamicComponent::default());
            }
        },
        _ => {}
    };
}

pub fn despawn_container_marker_component(
    index: usize,
    entity: Entity,
    mut commands: Commands,
    extended_marker_map: &ContainerExtendedMarkerMap,
) {
    match index {
        1 => {
            commands.entity(entity).remove::<Container1>();
        }
        2 => {
            commands.entity(entity).remove::<Container2>();
        }
        3 => {
            commands.entity(entity).remove::<Container3>();
        }
        4 => {
            commands.entity(entity).remove::<Container4>();
        }
        5 => {
            commands.entity(entity).remove::<Container5>();
        }
        6 => {
            commands.entity(entity).remove::<Container6>();
        }
        7 => {
            commands.entity(entity).remove::<Container7>();
        }
        8 => {
            commands.entity(entity).remove::<Container8>();
        }
        9 => {
            commands.entity(entity).remove::<Container9>();
        }
        10 => {
            commands.entity(entity).remove::<Container10>();
        }
        11 => {
            commands.entity(entity).remove::<Container11>();
        }
        12 => {
            commands.entity(entity).remove::<Container12>();
        }
        13 => {
            commands.entity(entity).remove::<Container13>();
        }
        14 => {
            commands.entity(entity).remove::<Container14>();
        }
        15 => {
            commands.entity(entity).remove::<Container15>();
        }
        16 => {
            commands.entity(entity).remove::<Container16>();
        }
        17 => {
            commands.entity(entity).remove::<Container17>();
        }
        18 => {
            commands.entity(entity).remove::<Container18>();
        }
        19 => {
            commands.entity(entity).remove::<Container19>();
        }
        20 => {
            commands.entity(entity).remove::<Container20>();
        }
        21 => {
            commands.entity(entity).remove::<Container21>();
        }
        22 => {
            commands.entity(entity).remove::<Container22>();
        }
        23 => {
            commands.entity(entity).remove::<Container23>();
        }
        24 => {
            commands.entity(entity).remove::<Container24>();
        }
        25 => {
            commands.entity(entity).remove::<Container25>();
        }
        26 => {
            commands.entity(entity).remove::<Container26>();
        }
        27 => {
            commands.entity(entity).remove::<Container27>();
        }
        28 => {
            commands.entity(entity).remove::<Container28>();
        }
        29 => {
            commands.entity(entity).remove::<Container29>();
        }
        30 => {
            commands.entity(entity).remove::<Container30>();
        }
        31 => {
            commands.entity(entity).remove::<Container31>();
        }
        32 => {
            commands.entity(entity).remove::<Container32>();
        }
        33 => {
            commands.entity(entity).remove::<Container33>();
        }
        34 => {
            commands.entity(entity).remove::<Container34>();
        }
        35 => {
            commands.entity(entity).remove::<Container35>();
        }
        36 => {
            commands.entity(entity).remove::<Container36>();
        }
        37 => {
            commands.entity(entity).remove::<Container37>();
        }
        38 => {
            commands.entity(entity).remove::<Container38>();
        }
        39 => {
            commands.entity(entity).remove::<Container39>();
        }
        40 => {
            commands.entity(entity).remove::<Container40>();
        }
        41 => {
            commands.entity(entity).remove::<Container41>();
        }
        42 => {
            commands.entity(entity).remove::<Container42>();
        }
        43 => {
            commands.entity(entity).remove::<Container43>();
        }
        44 => {
            commands.entity(entity).remove::<Container44>();
        }
        45 => {
            commands.entity(entity).remove::<Container45>();
        }
        46 => {
            commands.entity(entity).remove::<Container46>();
        }
        47 => {
            commands.entity(entity).remove::<Container47>();
        }
        48 => {
            commands.entity(entity).remove::<Container48>();
        }
        49 => {
            commands.entity(entity).remove::<Container49>();
        }
        50 => {
            commands.entity(entity).remove::<Container50>();
        }
        51 => {
            commands.entity(entity).remove::<Container51>();
        }
        52 => {
            commands.entity(entity).remove::<Container52>();
        }
        53 => {
            commands.entity(entity).remove::<Container53>();
        }
        54 => {
            commands.entity(entity).remove::<Container54>();
        }
        55 => {
            commands.entity(entity).remove::<Container55>();
        }
        56 => {
            commands.entity(entity).remove::<Container56>();
        }
        57 => {
            commands.entity(entity).remove::<Container57>();
        }
        58 => {
            commands.entity(entity).remove::<Container58>();
        }
        59 => {
            commands.entity(entity).remove::<Container59>();
        }
        60 => {
            commands.entity(entity).remove::<Container60>();
        }
        61 => {
            commands.entity(entity).remove::<Container61>();
        }
        62 => {
            commands.entity(entity).remove::<Container62>();
        }
        63 => {
            commands.entity(entity).remove::<Container63>();
        }
        64 => {
            commands.entity(entity).remove::<Container64>();
        }
        65 => {
            commands.entity(entity).remove::<Container65>();
        }
        66 => {
            commands.entity(entity).remove::<Container66>();
        }
        67 => {
            commands.entity(entity).remove::<Container67>();
        }
        68 => {
            commands.entity(entity).remove::<Container68>();
        }
        69 => {
            commands.entity(entity).remove::<Container69>();
        }
        70 => {
            commands.entity(entity).remove::<Container70>();
        }
        71 => {
            commands.entity(entity).remove::<Container71>();
        }
        72 => {
            commands.entity(entity).remove::<Container72>();
        }
        73 => {
            commands.entity(entity).remove::<Container73>();
        }
        74 => {
            commands.entity(entity).remove::<Container74>();
        }
        75 => {
            commands.entity(entity).remove::<Container75>();
        }
        76 => {
            commands.entity(entity).remove::<Container76>();
        }
        77 => {
            commands.entity(entity).remove::<Container77>();
        }
        78 => {
            commands.entity(entity).remove::<Container78>();
        }
        79 => {
            commands.entity(entity).remove::<Container79>();
        }
        80 => {
            commands.entity(entity).remove::<Container80>();
        }
        81 => {
            commands.entity(entity).remove::<Container81>();
        }
        82 => {
            commands.entity(entity).remove::<Container82>();
        }
        83 => {
            commands.entity(entity).remove::<Container83>();
        }
        84 => {
            commands.entity(entity).remove::<Container84>();
        }
        85 => {
            commands.entity(entity).remove::<Container85>();
        }
        86 => {
            commands.entity(entity).remove::<Container86>();
        }
        87 => {
            commands.entity(entity).remove::<Container87>();
        }
        88 => {
            commands.entity(entity).remove::<Container88>();
        }
        89 => {
            commands.entity(entity).remove::<Container89>();
        }
        90 => {
            commands.entity(entity).remove::<Container90>();
        }
        91 => {
            commands.entity(entity).remove::<Container91>();
        }
        92 => {
            commands.entity(entity).remove::<Container92>();
        }
        93 => {
            commands.entity(entity).remove::<Container93>();
        }
        94 => {
            commands.entity(entity).remove::<Container94>();
        }
        95 => {
            commands.entity(entity).remove::<Container95>();
        }
        96 => {
            commands.entity(entity).remove::<Container96>();
        }
        97 => {
            commands.entity(entity).remove::<Container97>();
        }
        98 => {
            commands.entity(entity).remove::<Container98>();
        }
        99 => {
            commands.entity(entity).remove::<Container99>();
        }
        100 => {
            commands.entity(entity).remove::<Container100>();
        }
        101 => {
            commands.entity(entity).remove::<Container101>();
        }
        102 => {
            commands.entity(entity).remove::<Container102>();
        }
        103 => {
            commands.entity(entity).remove::<Container103>();
        }
        104 => {
            commands.entity(entity).remove::<Container104>();
        }
        105 => {
            commands.entity(entity).remove::<Container105>();
        }
        106 => {
            commands.entity(entity).remove::<Container106>();
        }
        107 => {
            commands.entity(entity).remove::<Container107>();
        }
        108 => {
            commands.entity(entity).remove::<Container108>();
        }
        109 => {
            commands.entity(entity).remove::<Container109>();
        }
        110 => {
            commands.entity(entity).remove::<Container110>();
        }
        111 => {
            commands.entity(entity).remove::<Container111>();
        }
        112 => {
            commands.entity(entity).remove::<Container112>();
        }
        113 => {
            commands.entity(entity).remove::<Container113>();
        }
        114 => {
            commands.entity(entity).remove::<Container114>();
        }
        115 => {
            commands.entity(entity).remove::<Container115>();
        }
        116 => {
            commands.entity(entity).remove::<Container116>();
        }
        117 => {
            commands.entity(entity).remove::<Container117>();
        }
        118 => {
            commands.entity(entity).remove::<Container118>();
        }
        119 => {
            commands.entity(entity).remove::<Container119>();
        }
        120 => {
            commands.entity(entity).remove::<Container120>();
        }
        121 => {
            commands.entity(entity).remove::<Container121>();
        }
        122 => {
            commands.entity(entity).remove::<Container122>();
        }
        123 => {
            commands.entity(entity).remove::<Container123>();
        }
        124 => {
            commands.entity(entity).remove::<Container124>();
        }
        125 => {
            commands.entity(entity).remove::<Container125>();
        }
        126 => {
            commands.entity(entity).remove::<Container126>();
        }
        127 => {
            commands.entity(entity).remove::<Container127>();
        }
        128 => {
            commands.entity(entity).remove::<Container128>();
        }
        n if n > 128 => {
            if let Some(component_id) = extended_marker_map.makers.get(&n) {
                commands.entity(entity).remove_by_id(*component_id);
            } else {
                // Optional: Log warning if needed for index {n}
            }
        }
        _ => {}
    };
}
