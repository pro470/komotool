use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::Resource;
use bevy_reflect::Reflect;

#[derive(Component, Reflect)]
pub struct MonocleContainer;

#[derive(Component, Reflect)]
pub struct FloatingWindow;

#[derive(Component, Reflect)]
pub struct MaximizedWindow;

#[derive(Component, Reflect)]
pub struct Focused;

#[derive(Component, Reflect)]
pub struct LastFocused;

#[derive(Component, Reflect)]
pub struct FocusedGlobal;
