use bevy_app::App;
use bevy_ecs::component::Component;
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
