use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_reflect::Reflect;
use indexmap::IndexSet;

#[derive(Component, Reflect)]
pub struct KomotoolRing(#[reflect(ignore)] pub IndexSet<Entity>);

#[derive(Component, Reflect)]
pub struct MonocleContainer(pub Option<Entity>);

#[derive(Component, Reflect)]
pub struct FloatingWindow;

#[derive(Component, Reflect)]
pub struct FocusedWindow(pub Option<Entity>);

#[derive(Component, Reflect)]
pub struct FocusedContainer(pub Option<Entity>);

#[derive(Component, Reflect)]
pub struct LastFocusedContainer(pub Option<Entity>);

#[derive(Component, Reflect)]
pub struct FocusedWorkspace(pub Option<Entity>);

#[derive(Component, Reflect)]
pub struct LastFocusedWorkspace(pub Option<Entity>);

#[derive(Component, Reflect)]
pub struct MaximizedWindow(pub Option<Entity>);

#[derive(Component, Reflect)]
pub struct Focused(pub i32);
