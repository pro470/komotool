use crate::resources::WindowExtendedMarkerMap;
use bevy_app::App;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::system::Commands;
use bevy_mod_scripting::core::bindings::DynamicComponent;
use bevy_reflect::Reflect;

#[derive(Component, Reflect)]
pub struct Window1;

#[derive(Component, Reflect)]
pub struct Window2;

#[derive(Component, Reflect)]
pub struct Window3;

#[derive(Component, Reflect)]
pub struct Window4;

#[derive(Component, Reflect)]
pub struct Window5;

#[derive(Component, Reflect)]
pub struct Window6;

#[derive(Component, Reflect)]
pub struct Window7;

#[derive(Component, Reflect)]
pub struct Window8;

#[derive(Component, Reflect)]
pub struct Window9;

#[derive(Component, Reflect)]
pub struct Window10;

#[derive(Component, Reflect)]
pub struct Window11;

#[derive(Component, Reflect)]
pub struct Window12;

#[derive(Component, Reflect)]
pub struct Window13;

#[derive(Component, Reflect)]
pub struct Window14;

#[derive(Component, Reflect)]
pub struct Window15;

#[derive(Component, Reflect)]
pub struct Window16;

#[derive(Component, Reflect)]
pub struct Window17;

#[derive(Component, Reflect)]
pub struct Window18;

#[derive(Component, Reflect)]
pub struct Window19;

#[derive(Component, Reflect)]
pub struct Window20;

#[derive(Component, Reflect)]
pub struct Window21;

#[derive(Component, Reflect)]
pub struct Window22;

#[derive(Component, Reflect)]
pub struct Window23;

#[derive(Component, Reflect)]
pub struct Window24;

#[derive(Component, Reflect)]
pub struct Window25;

#[derive(Component, Reflect)]
pub struct Window26;

#[derive(Component, Reflect)]
pub struct Window27;

#[derive(Component, Reflect)]
pub struct Window28;

#[derive(Component, Reflect)]
pub struct Window29;

#[derive(Component, Reflect)]
pub struct Window30;

#[derive(Component, Reflect)]
pub struct Window31;

#[derive(Component, Reflect)]
pub struct Window32;

#[derive(Component, Reflect)]
pub struct Window33;

#[derive(Component, Reflect)]
pub struct Window34;

#[derive(Component, Reflect)]
pub struct Window35;

#[derive(Component, Reflect)]
pub struct Window36;

#[derive(Component, Reflect)]
pub struct Window37;

#[derive(Component, Reflect)]
pub struct Window38;

#[derive(Component, Reflect)]
pub struct Window39;

#[derive(Component, Reflect)]
pub struct Window40;

#[derive(Component, Reflect)]
pub struct Window41;

#[derive(Component, Reflect)]
pub struct Window42;

#[derive(Component, Reflect)]
pub struct Window43;

#[derive(Component, Reflect)]
pub struct Window44;

#[derive(Component, Reflect)]
pub struct Window45;

#[derive(Component, Reflect)]
pub struct Window46;

#[derive(Component, Reflect)]
pub struct Window47;

#[derive(Component, Reflect)]
pub struct Window48;

#[derive(Component, Reflect)]
pub struct Window49;

#[derive(Component, Reflect)]
pub struct Window50;

#[derive(Component, Reflect)]
pub struct Window51;

#[derive(Component, Reflect)]
pub struct Window52;

#[derive(Component, Reflect)]
pub struct Window53;

#[derive(Component, Reflect)]
pub struct Window54;

#[derive(Component, Reflect)]
pub struct Window55;

#[derive(Component, Reflect)]
pub struct Window56;

#[derive(Component, Reflect)]
pub struct Window57;

#[derive(Component, Reflect)]
pub struct Window58;

#[derive(Component, Reflect)]
pub struct Window59;

#[derive(Component, Reflect)]
pub struct Window60;

#[derive(Component, Reflect)]
pub struct Window61;

#[derive(Component, Reflect)]
pub struct Window62;

#[derive(Component, Reflect)]
pub struct Window63;

#[derive(Component, Reflect)]
pub struct Window64;

#[derive(Component, Reflect)]
pub struct Window65;

#[derive(Component, Reflect)]
pub struct Window66;

#[derive(Component, Reflect)]
pub struct Window67;

#[derive(Component, Reflect)]
pub struct Window68;

#[derive(Component, Reflect)]
pub struct Window69;

#[derive(Component, Reflect)]
pub struct Window70;

#[derive(Component, Reflect)]
pub struct Window71;

#[derive(Component, Reflect)]
pub struct Window72;

#[derive(Component, Reflect)]
pub struct Window73;

#[derive(Component, Reflect)]
pub struct Window74;

#[derive(Component, Reflect)]
pub struct Window75;

#[derive(Component, Reflect)]
pub struct Window76;

#[derive(Component, Reflect)]
pub struct Window77;

#[derive(Component, Reflect)]
pub struct Window78;

#[derive(Component, Reflect)]
pub struct Window79;

#[derive(Component, Reflect)]
pub struct Window80;

#[derive(Component, Reflect)]
pub struct Window81;

#[derive(Component, Reflect)]
pub struct Window82;

#[derive(Component, Reflect)]
pub struct Window83;

#[derive(Component, Reflect)]
pub struct Window84;

#[derive(Component, Reflect)]
pub struct Window85;

#[derive(Component, Reflect)]
pub struct Window86;

#[derive(Component, Reflect)]
pub struct Window87;

#[derive(Component, Reflect)]
pub struct Window88;

#[derive(Component, Reflect)]
pub struct Window89;

#[derive(Component, Reflect)]
pub struct Window90;

#[derive(Component, Reflect)]
pub struct Window91;

#[derive(Component, Reflect)]
pub struct Window92;

#[derive(Component, Reflect)]
pub struct Window93;

#[derive(Component, Reflect)]
pub struct Window94;

#[derive(Component, Reflect)]
pub struct Window95;

#[derive(Component, Reflect)]
pub struct Window96;

#[derive(Component, Reflect)]
pub struct Window97;

#[derive(Component, Reflect)]
pub struct Window98;

#[derive(Component, Reflect)]
pub struct Window99;

#[derive(Component, Reflect)]
pub struct Window100;

#[derive(Component, Reflect)]
pub struct Window101;

#[derive(Component, Reflect)]
pub struct Window102;

#[derive(Component, Reflect)]
pub struct Window103;

#[derive(Component, Reflect)]
pub struct Window104;

#[derive(Component, Reflect)]
pub struct Window105;

#[derive(Component, Reflect)]
pub struct Window106;

#[derive(Component, Reflect)]
pub struct Window107;

#[derive(Component, Reflect)]
pub struct Window108;

#[derive(Component, Reflect)]
pub struct Window109;

#[derive(Component, Reflect)]
pub struct Window110;

#[derive(Component, Reflect)]
pub struct Window111;

#[derive(Component, Reflect)]
pub struct Window112;

#[derive(Component, Reflect)]
pub struct Window113;

#[derive(Component, Reflect)]
pub struct Window114;

#[derive(Component, Reflect)]
pub struct Window115;

#[derive(Component, Reflect)]
pub struct Window116;

#[derive(Component, Reflect)]
pub struct Window117;

#[derive(Component, Reflect)]
pub struct Window118;

#[derive(Component, Reflect)]
pub struct Window119;

#[derive(Component, Reflect)]
pub struct Window120;

#[derive(Component, Reflect)]
pub struct Window121;

#[derive(Component, Reflect)]
pub struct Window122;

#[derive(Component, Reflect)]
pub struct Window123;

#[derive(Component, Reflect)]
pub struct Window124;

#[derive(Component, Reflect)]
pub struct Window125;

#[derive(Component, Reflect)]
pub struct Window126;

#[derive(Component, Reflect)]
pub struct Window127;

#[derive(Component, Reflect)]
pub struct Window128;

pub fn register_window_types(app: &mut App) {
    app.register_type::<Window1>()
        .register_type::<Window2>()
        .register_type::<Window3>()
        .register_type::<Window4>()
        .register_type::<Window5>()
        .register_type::<Window6>()
        .register_type::<Window7>()
        .register_type::<Window8>()
        .register_type::<Window9>()
        .register_type::<Window10>()
        .register_type::<Window11>()
        .register_type::<Window12>()
        .register_type::<Window13>()
        .register_type::<Window14>()
        .register_type::<Window15>()
        .register_type::<Window16>()
        .register_type::<Window17>()
        .register_type::<Window18>()
        .register_type::<Window19>()
        .register_type::<Window20>()
        .register_type::<Window21>()
        .register_type::<Window22>()
        .register_type::<Window23>()
        .register_type::<Window24>()
        .register_type::<Window25>()
        .register_type::<Window26>()
        .register_type::<Window27>()
        .register_type::<Window28>()
        .register_type::<Window29>()
        .register_type::<Window30>()
        .register_type::<Window31>()
        .register_type::<Window32>()
        .register_type::<Window33>()
        .register_type::<Window34>()
        .register_type::<Window35>()
        .register_type::<Window36>()
        .register_type::<Window37>()
        .register_type::<Window38>()
        .register_type::<Window39>()
        .register_type::<Window40>()
        .register_type::<Window41>()
        .register_type::<Window42>()
        .register_type::<Window43>()
        .register_type::<Window44>()
        .register_type::<Window45>()
        .register_type::<Window46>()
        .register_type::<Window47>()
        .register_type::<Window48>()
        .register_type::<Window49>()
        .register_type::<Window50>()
        .register_type::<Window51>()
        .register_type::<Window52>()
        .register_type::<Window53>()
        .register_type::<Window54>()
        .register_type::<Window55>()
        .register_type::<Window56>()
        .register_type::<Window57>()
        .register_type::<Window58>()
        .register_type::<Window59>()
        .register_type::<Window60>()
        .register_type::<Window61>()
        .register_type::<Window62>()
        .register_type::<Window63>()
        .register_type::<Window64>()
        .register_type::<Window65>()
        .register_type::<Window66>()
        .register_type::<Window67>()
        .register_type::<Window68>()
        .register_type::<Window69>()
        .register_type::<Window70>()
        .register_type::<Window71>()
        .register_type::<Window72>()
        .register_type::<Window73>()
        .register_type::<Window74>()
        .register_type::<Window75>()
        .register_type::<Window76>()
        .register_type::<Window77>()
        .register_type::<Window78>()
        .register_type::<Window79>()
        .register_type::<Window80>()
        .register_type::<Window81>()
        .register_type::<Window82>()
        .register_type::<Window83>()
        .register_type::<Window84>()
        .register_type::<Window85>()
        .register_type::<Window86>()
        .register_type::<Window87>()
        .register_type::<Window88>()
        .register_type::<Window89>()
        .register_type::<Window90>()
        .register_type::<Window91>()
        .register_type::<Window92>()
        .register_type::<Window93>()
        .register_type::<Window94>()
        .register_type::<Window95>()
        .register_type::<Window96>()
        .register_type::<Window97>()
        .register_type::<Window98>()
        .register_type::<Window99>()
        .register_type::<Window100>()
        .register_type::<Window101>()
        .register_type::<Window102>()
        .register_type::<Window103>()
        .register_type::<Window104>()
        .register_type::<Window105>()
        .register_type::<Window106>()
        .register_type::<Window107>()
        .register_type::<Window108>()
        .register_type::<Window109>()
        .register_type::<Window110>()
        .register_type::<Window111>()
        .register_type::<Window112>()
        .register_type::<Window113>()
        .register_type::<Window114>()
        .register_type::<Window115>()
        .register_type::<Window116>()
        .register_type::<Window117>()
        .register_type::<Window118>()
        .register_type::<Window119>()
        .register_type::<Window120>()
        .register_type::<Window121>()
        .register_type::<Window122>()
        .register_type::<Window123>()
        .register_type::<Window124>()
        .register_type::<Window125>()
        .register_type::<Window126>()
        .register_type::<Window127>()
        .register_type::<Window128>();
}

pub fn insert_window_marker_component(
    index: usize,
    entity: Entity,
    mut commands: Commands,
    extended_marker_map: &WindowExtendedMarkerMap,
) {
    match index {
        1 => {
            commands.entity(entity).insert(Window1);
        }
        2 => {
            commands.entity(entity).insert(Window2);
        }
        3 => {
            commands.entity(entity).insert(Window3);
        }
        4 => {
            commands.entity(entity).insert(Window4);
        }
        5 => {
            commands.entity(entity).insert(Window5);
        }
        6 => {
            commands.entity(entity).insert(Window6);
        }
        7 => {
            commands.entity(entity).insert(Window7);
        }
        8 => {
            commands.entity(entity).insert(Window8);
        }
        9 => {
            commands.entity(entity).insert(Window9);
        }
        10 => {
            commands.entity(entity).insert(Window10);
        }
        11 => {
            commands.entity(entity).insert(Window11);
        }
        12 => {
            commands.entity(entity).insert(Window12);
        }
        13 => {
            commands.entity(entity).insert(Window13);
        }
        14 => {
            commands.entity(entity).insert(Window14);
        }
        15 => {
            commands.entity(entity).insert(Window15);
        }
        16 => {
            commands.entity(entity).insert(Window16);
        }
        17 => {
            commands.entity(entity).insert(Window17);
        }
        18 => {
            commands.entity(entity).insert(Window18);
        }
        19 => {
            commands.entity(entity).insert(Window19);
        }
        20 => {
            commands.entity(entity).insert(Window20);
        }
        21 => {
            commands.entity(entity).insert(Window21);
        }
        22 => {
            commands.entity(entity).insert(Window22);
        }
        23 => {
            commands.entity(entity).insert(Window23);
        }
        24 => {
            commands.entity(entity).insert(Window24);
        }
        25 => {
            commands.entity(entity).insert(Window25);
        }
        26 => {
            commands.entity(entity).insert(Window26);
        }
        27 => {
            commands.entity(entity).insert(Window27);
        }
        28 => {
            commands.entity(entity).insert(Window28);
        }
        29 => {
            commands.entity(entity).insert(Window29);
        }
        30 => {
            commands.entity(entity).insert(Window30);
        }
        31 => {
            commands.entity(entity).insert(Window31);
        }
        32 => {
            commands.entity(entity).insert(Window32);
        }
        33 => {
            commands.entity(entity).insert(Window33);
        }
        34 => {
            commands.entity(entity).insert(Window34);
        }
        35 => {
            commands.entity(entity).insert(Window35);
        }
        36 => {
            commands.entity(entity).insert(Window36);
        }
        37 => {
            commands.entity(entity).insert(Window37);
        }
        38 => {
            commands.entity(entity).insert(Window38);
        }
        39 => {
            commands.entity(entity).insert(Window39);
        }
        40 => {
            commands.entity(entity).insert(Window40);
        }
        41 => {
            commands.entity(entity).insert(Window41);
        }
        42 => {
            commands.entity(entity).insert(Window42);
        }
        43 => {
            commands.entity(entity).insert(Window43);
        }
        44 => {
            commands.entity(entity).insert(Window44);
        }
        45 => {
            commands.entity(entity).insert(Window45);
        }
        46 => {
            commands.entity(entity).insert(Window46);
        }
        47 => {
            commands.entity(entity).insert(Window47);
        }
        48 => {
            commands.entity(entity).insert(Window48);
        }
        49 => {
            commands.entity(entity).insert(Window49);
        }
        50 => {
            commands.entity(entity).insert(Window50);
        }
        51 => {
            commands.entity(entity).insert(Window51);
        }
        52 => {
            commands.entity(entity).insert(Window52);
        }
        53 => {
            commands.entity(entity).insert(Window53);
        }
        54 => {
            commands.entity(entity).insert(Window54);
        }
        55 => {
            commands.entity(entity).insert(Window55);
        }
        56 => {
            commands.entity(entity).insert(Window56);
        }
        57 => {
            commands.entity(entity).insert(Window57);
        }
        58 => {
            commands.entity(entity).insert(Window58);
        }
        59 => {
            commands.entity(entity).insert(Window59);
        }
        60 => {
            commands.entity(entity).insert(Window60);
        }
        61 => {
            commands.entity(entity).insert(Window61);
        }
        62 => {
            commands.entity(entity).insert(Window62);
        }
        63 => {
            commands.entity(entity).insert(Window63);
        }
        64 => {
            commands.entity(entity).insert(Window64);
        }
        65 => {
            commands.entity(entity).insert(Window65);
        }
        66 => {
            commands.entity(entity).insert(Window66);
        }
        67 => {
            commands.entity(entity).insert(Window67);
        }
        68 => {
            commands.entity(entity).insert(Window68);
        }
        69 => {
            commands.entity(entity).insert(Window69);
        }
        70 => {
            commands.entity(entity).insert(Window70);
        }
        71 => {
            commands.entity(entity).insert(Window71);
        }
        72 => {
            commands.entity(entity).insert(Window72);
        }
        73 => {
            commands.entity(entity).insert(Window73);
        }
        74 => {
            commands.entity(entity).insert(Window74);
        }
        75 => {
            commands.entity(entity).insert(Window75);
        }
        76 => {
            commands.entity(entity).insert(Window76);
        }
        77 => {
            commands.entity(entity).insert(Window77);
        }
        78 => {
            commands.entity(entity).insert(Window78);
        }
        79 => {
            commands.entity(entity).insert(Window79);
        }
        80 => {
            commands.entity(entity).insert(Window80);
        }
        81 => {
            commands.entity(entity).insert(Window81);
        }
        82 => {
            commands.entity(entity).insert(Window82);
        }
        83 => {
            commands.entity(entity).insert(Window83);
        }
        84 => {
            commands.entity(entity).insert(Window84);
        }
        85 => {
            commands.entity(entity).insert(Window85);
        }
        86 => {
            commands.entity(entity).insert(Window86);
        }
        87 => {
            commands.entity(entity).insert(Window87);
        }
        88 => {
            commands.entity(entity).insert(Window88);
        }
        89 => {
            commands.entity(entity).insert(Window89);
        }
        90 => {
            commands.entity(entity).insert(Window90);
        }
        91 => {
            commands.entity(entity).insert(Window91);
        }
        92 => {
            commands.entity(entity).insert(Window92);
        }
        93 => {
            commands.entity(entity).insert(Window93);
        }
        94 => {
            commands.entity(entity).insert(Window94);
        }
        95 => {
            commands.entity(entity).insert(Window95);
        }
        96 => {
            commands.entity(entity).insert(Window96);
        }
        97 => {
            commands.entity(entity).insert(Window97);
        }
        98 => {
            commands.entity(entity).insert(Window98);
        }
        99 => {
            commands.entity(entity).insert(Window99);
        }
        100 => {
            commands.entity(entity).insert(Window100);
        }
        101 => {
            commands.entity(entity).insert(Window101);
        }
        102 => {
            commands.entity(entity).insert(Window102);
        }
        103 => {
            commands.entity(entity).insert(Window103);
        }
        104 => {
            commands.entity(entity).insert(Window104);
        }
        105 => {
            commands.entity(entity).insert(Window105);
        }
        106 => {
            commands.entity(entity).insert(Window106);
        }
        107 => {
            commands.entity(entity).insert(Window107);
        }
        108 => {
            commands.entity(entity).insert(Window108);
        }
        109 => {
            commands.entity(entity).insert(Window109);
        }
        110 => {
            commands.entity(entity).insert(Window110);
        }
        111 => {
            commands.entity(entity).insert(Window111);
        }
        112 => {
            commands.entity(entity).insert(Window112);
        }
        113 => {
            commands.entity(entity).insert(Window113);
        }
        114 => {
            commands.entity(entity).insert(Window114);
        }
        115 => {
            commands.entity(entity).insert(Window115);
        }
        116 => {
            commands.entity(entity).insert(Window116);
        }
        117 => {
            commands.entity(entity).insert(Window117);
        }
        118 => {
            commands.entity(entity).insert(Window118);
        }
        119 => {
            commands.entity(entity).insert(Window119);
        }
        120 => {
            commands.entity(entity).insert(Window120);
        }
        121 => {
            commands.entity(entity).insert(Window121);
        }
        122 => {
            commands.entity(entity).insert(Window122);
        }
        123 => {
            commands.entity(entity).insert(Window123);
        }
        124 => {
            commands.entity(entity).insert(Window124);
        }
        125 => {
            commands.entity(entity).insert(Window125);
        }
        126 => {
            commands.entity(entity).insert(Window126);
        }
        127 => {
            commands.entity(entity).insert(Window127);
        }
        128 => {
            commands.entity(entity).insert(Window128);
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

pub fn despawn_window_marker_component(
    index: usize,
    entity: Entity,
    mut commands: Commands,
    extended_marker_map: &WindowExtendedMarkerMap,
) {
    match index {
        1 => {
            commands.entity(entity).remove::<Window1>();
        }
        2 => {
            commands.entity(entity).remove::<Window2>();
        }
        3 => {
            commands.entity(entity).remove::<Window3>();
        }
        4 => {
            commands.entity(entity).remove::<Window4>();
        }
        5 => {
            commands.entity(entity).remove::<Window5>();
        }
        6 => {
            commands.entity(entity).remove::<Window6>();
        }
        7 => {
            commands.entity(entity).remove::<Window7>();
        }
        8 => {
            commands.entity(entity).remove::<Window8>();
        }
        9 => {
            commands.entity(entity).remove::<Window9>();
        }
        10 => {
            commands.entity(entity).remove::<Window10>();
        }
        11 => {
            commands.entity(entity).remove::<Window11>();
        }
        12 => {
            commands.entity(entity).remove::<Window12>();
        }
        13 => {
            commands.entity(entity).remove::<Window13>();
        }
        14 => {
            commands.entity(entity).remove::<Window14>();
        }
        15 => {
            commands.entity(entity).remove::<Window15>();
        }
        16 => {
            commands.entity(entity).remove::<Window16>();
        }
        17 => {
            commands.entity(entity).remove::<Window17>();
        }
        18 => {
            commands.entity(entity).remove::<Window18>();
        }
        19 => {
            commands.entity(entity).remove::<Window19>();
        }
        20 => {
            commands.entity(entity).remove::<Window20>();
        }
        21 => {
            commands.entity(entity).remove::<Window21>();
        }
        22 => {
            commands.entity(entity).remove::<Window22>();
        }
        23 => {
            commands.entity(entity).remove::<Window23>();
        }
        24 => {
            commands.entity(entity).remove::<Window24>();
        }
        25 => {
            commands.entity(entity).remove::<Window25>();
        }
        26 => {
            commands.entity(entity).remove::<Window26>();
        }
        27 => {
            commands.entity(entity).remove::<Window27>();
        }
        28 => {
            commands.entity(entity).remove::<Window28>();
        }
        29 => {
            commands.entity(entity).remove::<Window29>();
        }
        30 => {
            commands.entity(entity).remove::<Window30>();
        }
        31 => {
            commands.entity(entity).remove::<Window31>();
        }
        32 => {
            commands.entity(entity).remove::<Window32>();
        }
        33 => {
            commands.entity(entity).remove::<Window33>();
        }
        34 => {
            commands.entity(entity).remove::<Window34>();
        }
        35 => {
            commands.entity(entity).remove::<Window35>();
        }
        36 => {
            commands.entity(entity).remove::<Window36>();
        }
        37 => {
            commands.entity(entity).remove::<Window37>();
        }
        38 => {
            commands.entity(entity).remove::<Window38>();
        }
        39 => {
            commands.entity(entity).remove::<Window39>();
        }
        40 => {
            commands.entity(entity).remove::<Window40>();
        }
        41 => {
            commands.entity(entity).remove::<Window41>();
        }
        42 => {
            commands.entity(entity).remove::<Window42>();
        }
        43 => {
            commands.entity(entity).remove::<Window43>();
        }
        44 => {
            commands.entity(entity).remove::<Window44>();
        }
        45 => {
            commands.entity(entity).remove::<Window45>();
        }
        46 => {
            commands.entity(entity).remove::<Window46>();
        }
        47 => {
            commands.entity(entity).remove::<Window47>();
        }
        48 => {
            commands.entity(entity).remove::<Window48>();
        }
        49 => {
            commands.entity(entity).remove::<Window49>();
        }
        50 => {
            commands.entity(entity).remove::<Window50>();
        }
        51 => {
            commands.entity(entity).remove::<Window51>();
        }
        52 => {
            commands.entity(entity).remove::<Window52>();
        }
        53 => {
            commands.entity(entity).remove::<Window53>();
        }
        54 => {
            commands.entity(entity).remove::<Window54>();
        }
        55 => {
            commands.entity(entity).remove::<Window55>();
        }
        56 => {
            commands.entity(entity).remove::<Window56>();
        }
        57 => {
            commands.entity(entity).remove::<Window57>();
        }
        58 => {
            commands.entity(entity).remove::<Window58>();
        }
        59 => {
            commands.entity(entity).remove::<Window59>();
        }
        60 => {
            commands.entity(entity).remove::<Window60>();
        }
        61 => {
            commands.entity(entity).remove::<Window61>();
        }
        62 => {
            commands.entity(entity).remove::<Window62>();
        }
        63 => {
            commands.entity(entity).remove::<Window63>();
        }
        64 => {
            commands.entity(entity).remove::<Window64>();
        }
        65 => {
            commands.entity(entity).remove::<Window65>();
        }
        66 => {
            commands.entity(entity).remove::<Window66>();
        }
        67 => {
            commands.entity(entity).remove::<Window67>();
        }
        68 => {
            commands.entity(entity).remove::<Window68>();
        }
        69 => {
            commands.entity(entity).remove::<Window69>();
        }
        70 => {
            commands.entity(entity).remove::<Window70>();
        }
        71 => {
            commands.entity(entity).remove::<Window71>();
        }
        72 => {
            commands.entity(entity).remove::<Window72>();
        }
        73 => {
            commands.entity(entity).remove::<Window73>();
        }
        74 => {
            commands.entity(entity).remove::<Window74>();
        }
        75 => {
            commands.entity(entity).remove::<Window75>();
        }
        76 => {
            commands.entity(entity).remove::<Window76>();
        }
        77 => {
            commands.entity(entity).remove::<Window77>();
        }
        78 => {
            commands.entity(entity).remove::<Window78>();
        }
        79 => {
            commands.entity(entity).remove::<Window79>();
        }
        80 => {
            commands.entity(entity).remove::<Window80>();
        }
        81 => {
            commands.entity(entity).remove::<Window81>();
        }
        82 => {
            commands.entity(entity).remove::<Window82>();
        }
        83 => {
            commands.entity(entity).remove::<Window83>();
        }
        84 => {
            commands.entity(entity).remove::<Window84>();
        }
        85 => {
            commands.entity(entity).remove::<Window85>();
        }
        86 => {
            commands.entity(entity).remove::<Window86>();
        }
        87 => {
            commands.entity(entity).remove::<Window87>();
        }
        88 => {
            commands.entity(entity).remove::<Window88>();
        }
        89 => {
            commands.entity(entity).remove::<Window89>();
        }
        90 => {
            commands.entity(entity).remove::<Window90>();
        }
        91 => {
            commands.entity(entity).remove::<Window91>();
        }
        92 => {
            commands.entity(entity).remove::<Window92>();
        }
        93 => {
            commands.entity(entity).remove::<Window93>();
        }
        94 => {
            commands.entity(entity).remove::<Window94>();
        }
        95 => {
            commands.entity(entity).remove::<Window95>();
        }
        96 => {
            commands.entity(entity).remove::<Window96>();
        }
        97 => {
            commands.entity(entity).remove::<Window97>();
        }
        98 => {
            commands.entity(entity).remove::<Window98>();
        }
        99 => {
            commands.entity(entity).remove::<Window99>();
        }
        100 => {
            commands.entity(entity).remove::<Window100>();
        }
        101 => {
            commands.entity(entity).remove::<Window101>();
        }
        102 => {
            commands.entity(entity).remove::<Window102>();
        }
        103 => {
            commands.entity(entity).remove::<Window103>();
        }
        104 => {
            commands.entity(entity).remove::<Window104>();
        }
        105 => {
            commands.entity(entity).remove::<Window105>();
        }
        106 => {
            commands.entity(entity).remove::<Window106>();
        }
        107 => {
            commands.entity(entity).remove::<Window107>();
        }
        108 => {
            commands.entity(entity).remove::<Window108>();
        }
        109 => {
            commands.entity(entity).remove::<Window109>();
        }
        110 => {
            commands.entity(entity).remove::<Window110>();
        }
        111 => {
            commands.entity(entity).remove::<Window111>();
        }
        112 => {
            commands.entity(entity).remove::<Window112>();
        }
        113 => {
            commands.entity(entity).remove::<Window113>();
        }
        114 => {
            commands.entity(entity).remove::<Window114>();
        }
        115 => {
            commands.entity(entity).remove::<Window115>();
        }
        116 => {
            commands.entity(entity).remove::<Window116>();
        }
        117 => {
            commands.entity(entity).remove::<Window117>();
        }
        118 => {
            commands.entity(entity).remove::<Window118>();
        }
        119 => {
            commands.entity(entity).remove::<Window119>();
        }
        120 => {
            commands.entity(entity).remove::<Window120>();
        }
        121 => {
            commands.entity(entity).remove::<Window121>();
        }
        122 => {
            commands.entity(entity).remove::<Window122>();
        }
        123 => {
            commands.entity(entity).remove::<Window123>();
        }
        124 => {
            commands.entity(entity).remove::<Window124>();
        }
        125 => {
            commands.entity(entity).remove::<Window125>();
        }
        126 => {
            commands.entity(entity).remove::<Window126>();
        }
        127 => {
            commands.entity(entity).remove::<Window127>();
        }
        128 => {
            commands.entity(entity).remove::<Window128>();
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
