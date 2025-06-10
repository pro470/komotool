use crate::components::*;
use bevy_ecs::component::ComponentId;
use bevy_ecs::resource::Resource;
use bevy_ecs::world::World;
use bevy_reflect::Reflect;
use indexmap::IndexSet;
use std::ops::{Deref, DerefMut};

#[derive(Resource, Default, Reflect)]
pub struct Windowmakerset(#[reflect(ignore)] IndexSet<ComponentId>);

impl Deref for Windowmakerset {
    type Target = IndexSet<ComponentId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Windowmakerset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Resource, Default, Reflect)]
pub struct Containermakerset(#[reflect(ignore)] IndexSet<ComponentId>);

impl Deref for Containermakerset {
    type Target = IndexSet<ComponentId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Containermakerset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Resource, Default, Reflect)]
pub struct Workspacemakerset(#[reflect(ignore)] IndexSet<ComponentId>);

impl Deref for Workspacemakerset {
    type Target = IndexSet<ComponentId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Workspacemakerset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Resource, Default, Reflect)]
pub struct Monitormakerset(#[reflect(ignore)] IndexSet<ComponentId>);

impl Deref for Monitormakerset {
    type Target = IndexSet<ComponentId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Monitormakerset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Resource, Reflect)]
pub struct MonocleContainermakerid(ComponentId);

impl Deref for MonocleContainermakerid {
    type Target = ComponentId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Resource, Reflect)]
pub struct MaximizedWindowmakerid(ComponentId);

impl Deref for MaximizedWindowmakerid {
    type Target = ComponentId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn register_maker_sets(world: &mut World) {
    let mut windowmakerset = Windowmakerset::default();

    windowmakerset.insert(world.register_component::<Window1>());
    windowmakerset.insert(world.register_component::<Window2>());
    windowmakerset.insert(world.register_component::<Window3>());
    windowmakerset.insert(world.register_component::<Window4>());
    windowmakerset.insert(world.register_component::<Window5>());
    windowmakerset.insert(world.register_component::<Window6>());
    windowmakerset.insert(world.register_component::<Window7>());
    windowmakerset.insert(world.register_component::<Window8>());
    windowmakerset.insert(world.register_component::<Window9>());
    windowmakerset.insert(world.register_component::<Window10>());
    windowmakerset.insert(world.register_component::<Window11>());
    windowmakerset.insert(world.register_component::<Window12>());
    windowmakerset.insert(world.register_component::<Window13>());
    windowmakerset.insert(world.register_component::<Window14>());
    windowmakerset.insert(world.register_component::<Window15>());
    windowmakerset.insert(world.register_component::<Window16>());
    windowmakerset.insert(world.register_component::<Window17>());
    windowmakerset.insert(world.register_component::<Window18>());
    windowmakerset.insert(world.register_component::<Window19>());
    windowmakerset.insert(world.register_component::<Window20>());
    windowmakerset.insert(world.register_component::<Window21>());
    windowmakerset.insert(world.register_component::<Window22>());
    windowmakerset.insert(world.register_component::<Window23>());
    windowmakerset.insert(world.register_component::<Window24>());
    windowmakerset.insert(world.register_component::<Window25>());
    windowmakerset.insert(world.register_component::<Window26>());
    windowmakerset.insert(world.register_component::<Window27>());
    windowmakerset.insert(world.register_component::<Window28>());
    windowmakerset.insert(world.register_component::<Window29>());
    windowmakerset.insert(world.register_component::<Window30>());
    windowmakerset.insert(world.register_component::<Window31>());
    windowmakerset.insert(world.register_component::<Window32>());
    windowmakerset.insert(world.register_component::<Window33>());
    windowmakerset.insert(world.register_component::<Window34>());
    windowmakerset.insert(world.register_component::<Window35>());
    windowmakerset.insert(world.register_component::<Window36>());
    windowmakerset.insert(world.register_component::<Window37>());
    windowmakerset.insert(world.register_component::<Window38>());
    windowmakerset.insert(world.register_component::<Window39>());
    windowmakerset.insert(world.register_component::<Window40>());
    windowmakerset.insert(world.register_component::<Window41>());
    windowmakerset.insert(world.register_component::<Window42>());
    windowmakerset.insert(world.register_component::<Window43>());
    windowmakerset.insert(world.register_component::<Window44>());
    windowmakerset.insert(world.register_component::<Window45>());
    windowmakerset.insert(world.register_component::<Window46>());
    windowmakerset.insert(world.register_component::<Window47>());
    windowmakerset.insert(world.register_component::<Window48>());
    windowmakerset.insert(world.register_component::<Window49>());
    windowmakerset.insert(world.register_component::<Window50>());
    windowmakerset.insert(world.register_component::<Window51>());
    windowmakerset.insert(world.register_component::<Window52>());
    windowmakerset.insert(world.register_component::<Window53>());
    windowmakerset.insert(world.register_component::<Window54>());
    windowmakerset.insert(world.register_component::<Window55>());
    windowmakerset.insert(world.register_component::<Window56>());
    windowmakerset.insert(world.register_component::<Window57>());
    windowmakerset.insert(world.register_component::<Window58>());
    windowmakerset.insert(world.register_component::<Window59>());
    windowmakerset.insert(world.register_component::<Window60>());
    windowmakerset.insert(world.register_component::<Window61>());
    windowmakerset.insert(world.register_component::<Window62>());
    windowmakerset.insert(world.register_component::<Window63>());
    windowmakerset.insert(world.register_component::<Window64>());
    windowmakerset.insert(world.register_component::<Window65>());
    windowmakerset.insert(world.register_component::<Window66>());
    windowmakerset.insert(world.register_component::<Window67>());
    windowmakerset.insert(world.register_component::<Window68>());
    windowmakerset.insert(world.register_component::<Window69>());
    windowmakerset.insert(world.register_component::<Window70>());
    windowmakerset.insert(world.register_component::<Window71>());
    windowmakerset.insert(world.register_component::<Window72>());
    windowmakerset.insert(world.register_component::<Window73>());
    windowmakerset.insert(world.register_component::<Window74>());
    windowmakerset.insert(world.register_component::<Window75>());
    windowmakerset.insert(world.register_component::<Window76>());
    windowmakerset.insert(world.register_component::<Window77>());
    windowmakerset.insert(world.register_component::<Window78>());
    windowmakerset.insert(world.register_component::<Window79>());
    windowmakerset.insert(world.register_component::<Window80>());
    windowmakerset.insert(world.register_component::<Window81>());
    windowmakerset.insert(world.register_component::<Window82>());
    windowmakerset.insert(world.register_component::<Window83>());
    windowmakerset.insert(world.register_component::<Window84>());
    windowmakerset.insert(world.register_component::<Window85>());
    windowmakerset.insert(world.register_component::<Window86>());
    windowmakerset.insert(world.register_component::<Window87>());
    windowmakerset.insert(world.register_component::<Window88>());
    windowmakerset.insert(world.register_component::<Window89>());
    windowmakerset.insert(world.register_component::<Window90>());
    windowmakerset.insert(world.register_component::<Window91>());
    windowmakerset.insert(world.register_component::<Window92>());
    windowmakerset.insert(world.register_component::<Window93>());
    windowmakerset.insert(world.register_component::<Window94>());
    windowmakerset.insert(world.register_component::<Window95>());
    windowmakerset.insert(world.register_component::<Window96>());
    windowmakerset.insert(world.register_component::<Window97>());
    windowmakerset.insert(world.register_component::<Window98>());
    windowmakerset.insert(world.register_component::<Window99>());
    windowmakerset.insert(world.register_component::<Window100>());
    windowmakerset.insert(world.register_component::<Window101>());
    windowmakerset.insert(world.register_component::<Window102>());
    windowmakerset.insert(world.register_component::<Window103>());
    windowmakerset.insert(world.register_component::<Window104>());
    windowmakerset.insert(world.register_component::<Window105>());
    windowmakerset.insert(world.register_component::<Window106>());
    windowmakerset.insert(world.register_component::<Window107>());
    windowmakerset.insert(world.register_component::<Window108>());
    windowmakerset.insert(world.register_component::<Window109>());
    windowmakerset.insert(world.register_component::<Window110>());
    windowmakerset.insert(world.register_component::<Window111>());
    windowmakerset.insert(world.register_component::<Window112>());
    windowmakerset.insert(world.register_component::<Window113>());
    windowmakerset.insert(world.register_component::<Window114>());
    windowmakerset.insert(world.register_component::<Window115>());
    windowmakerset.insert(world.register_component::<Window116>());
    windowmakerset.insert(world.register_component::<Window117>());
    windowmakerset.insert(world.register_component::<Window118>());
    windowmakerset.insert(world.register_component::<Window119>());
    windowmakerset.insert(world.register_component::<Window120>());
    windowmakerset.insert(world.register_component::<Window121>());
    windowmakerset.insert(world.register_component::<Window122>());
    windowmakerset.insert(world.register_component::<Window123>());
    windowmakerset.insert(world.register_component::<Window124>());
    windowmakerset.insert(world.register_component::<Window125>());
    windowmakerset.insert(world.register_component::<Window126>());
    windowmakerset.insert(world.register_component::<Window127>());
    windowmakerset.insert(world.register_component::<Window128>());

    world.insert_resource(windowmakerset);

    let mut containermakerset = Containermakerset::default();

    containermakerset.insert(world.register_component::<Container1>());
    containermakerset.insert(world.register_component::<Container2>());
    containermakerset.insert(world.register_component::<Container3>());
    containermakerset.insert(world.register_component::<Container4>());
    containermakerset.insert(world.register_component::<Container5>());
    containermakerset.insert(world.register_component::<Container6>());
    containermakerset.insert(world.register_component::<Container7>());
    containermakerset.insert(world.register_component::<Container8>());
    containermakerset.insert(world.register_component::<Container9>());
    containermakerset.insert(world.register_component::<Container10>());
    containermakerset.insert(world.register_component::<Container11>());
    containermakerset.insert(world.register_component::<Container12>());
    containermakerset.insert(world.register_component::<Container13>());
    containermakerset.insert(world.register_component::<Container14>());
    containermakerset.insert(world.register_component::<Container15>());
    containermakerset.insert(world.register_component::<Container16>());
    containermakerset.insert(world.register_component::<Container17>());
    containermakerset.insert(world.register_component::<Container18>());
    containermakerset.insert(world.register_component::<Container19>());
    containermakerset.insert(world.register_component::<Container20>());
    containermakerset.insert(world.register_component::<Container21>());
    containermakerset.insert(world.register_component::<Container22>());
    containermakerset.insert(world.register_component::<Container23>());
    containermakerset.insert(world.register_component::<Container24>());
    containermakerset.insert(world.register_component::<Container25>());
    containermakerset.insert(world.register_component::<Container26>());
    containermakerset.insert(world.register_component::<Container27>());
    containermakerset.insert(world.register_component::<Container28>());
    containermakerset.insert(world.register_component::<Container29>());
    containermakerset.insert(world.register_component::<Container30>());
    containermakerset.insert(world.register_component::<Container31>());
    containermakerset.insert(world.register_component::<Container32>());
    containermakerset.insert(world.register_component::<Container33>());
    containermakerset.insert(world.register_component::<Container34>());
    containermakerset.insert(world.register_component::<Container35>());
    containermakerset.insert(world.register_component::<Container36>());
    containermakerset.insert(world.register_component::<Container37>());
    containermakerset.insert(world.register_component::<Container38>());
    containermakerset.insert(world.register_component::<Container39>());
    containermakerset.insert(world.register_component::<Container40>());
    containermakerset.insert(world.register_component::<Container41>());
    containermakerset.insert(world.register_component::<Container42>());
    containermakerset.insert(world.register_component::<Container43>());
    containermakerset.insert(world.register_component::<Container44>());
    containermakerset.insert(world.register_component::<Container45>());
    containermakerset.insert(world.register_component::<Container46>());
    containermakerset.insert(world.register_component::<Container47>());
    containermakerset.insert(world.register_component::<Container48>());
    containermakerset.insert(world.register_component::<Container49>());
    containermakerset.insert(world.register_component::<Container50>());
    containermakerset.insert(world.register_component::<Container51>());
    containermakerset.insert(world.register_component::<Container52>());
    containermakerset.insert(world.register_component::<Container53>());
    containermakerset.insert(world.register_component::<Container54>());
    containermakerset.insert(world.register_component::<Container55>());
    containermakerset.insert(world.register_component::<Container56>());
    containermakerset.insert(world.register_component::<Container57>());
    containermakerset.insert(world.register_component::<Container58>());
    containermakerset.insert(world.register_component::<Container59>());
    containermakerset.insert(world.register_component::<Container60>());
    containermakerset.insert(world.register_component::<Container61>());
    containermakerset.insert(world.register_component::<Container62>());
    containermakerset.insert(world.register_component::<Container63>());
    containermakerset.insert(world.register_component::<Container64>());
    containermakerset.insert(world.register_component::<Container65>());
    containermakerset.insert(world.register_component::<Container66>());
    containermakerset.insert(world.register_component::<Container67>());
    containermakerset.insert(world.register_component::<Container68>());
    containermakerset.insert(world.register_component::<Container69>());
    containermakerset.insert(world.register_component::<Container70>());
    containermakerset.insert(world.register_component::<Container71>());
    containermakerset.insert(world.register_component::<Container72>());
    containermakerset.insert(world.register_component::<Container73>());
    containermakerset.insert(world.register_component::<Container74>());
    containermakerset.insert(world.register_component::<Container75>());
    containermakerset.insert(world.register_component::<Container76>());
    containermakerset.insert(world.register_component::<Container77>());
    containermakerset.insert(world.register_component::<Container78>());
    containermakerset.insert(world.register_component::<Container79>());
    containermakerset.insert(world.register_component::<Container80>());
    containermakerset.insert(world.register_component::<Container81>());
    containermakerset.insert(world.register_component::<Container82>());
    containermakerset.insert(world.register_component::<Container83>());
    containermakerset.insert(world.register_component::<Container84>());
    containermakerset.insert(world.register_component::<Container85>());
    containermakerset.insert(world.register_component::<Container86>());
    containermakerset.insert(world.register_component::<Container87>());
    containermakerset.insert(world.register_component::<Container88>());
    containermakerset.insert(world.register_component::<Container89>());
    containermakerset.insert(world.register_component::<Container90>());
    containermakerset.insert(world.register_component::<Container91>());
    containermakerset.insert(world.register_component::<Container92>());
    containermakerset.insert(world.register_component::<Container93>());
    containermakerset.insert(world.register_component::<Container94>());
    containermakerset.insert(world.register_component::<Container95>());
    containermakerset.insert(world.register_component::<Container96>());
    containermakerset.insert(world.register_component::<Container97>());
    containermakerset.insert(world.register_component::<Container98>());
    containermakerset.insert(world.register_component::<Container99>());
    containermakerset.insert(world.register_component::<Container100>());
    containermakerset.insert(world.register_component::<Container101>());
    containermakerset.insert(world.register_component::<Container102>());
    containermakerset.insert(world.register_component::<Container103>());
    containermakerset.insert(world.register_component::<Container104>());
    containermakerset.insert(world.register_component::<Container105>());
    containermakerset.insert(world.register_component::<Container106>());
    containermakerset.insert(world.register_component::<Container107>());
    containermakerset.insert(world.register_component::<Container108>());
    containermakerset.insert(world.register_component::<Container109>());
    containermakerset.insert(world.register_component::<Container110>());
    containermakerset.insert(world.register_component::<Container111>());
    containermakerset.insert(world.register_component::<Container112>());
    containermakerset.insert(world.register_component::<Container113>());
    containermakerset.insert(world.register_component::<Container114>());
    containermakerset.insert(world.register_component::<Container115>());
    containermakerset.insert(world.register_component::<Container116>());
    containermakerset.insert(world.register_component::<Container117>());
    containermakerset.insert(world.register_component::<Container118>());
    containermakerset.insert(world.register_component::<Container119>());
    containermakerset.insert(world.register_component::<Container120>());
    containermakerset.insert(world.register_component::<Container121>());
    containermakerset.insert(world.register_component::<Container122>());
    containermakerset.insert(world.register_component::<Container123>());
    containermakerset.insert(world.register_component::<Container124>());
    containermakerset.insert(world.register_component::<Container125>());
    containermakerset.insert(world.register_component::<Container126>());
    containermakerset.insert(world.register_component::<Container127>());
    containermakerset.insert(world.register_component::<Container128>());

    world.insert_resource(containermakerset);

    let mut workspacemakerset = Workspacemakerset::default();

    workspacemakerset.insert(world.register_component::<Workspace1>());
    workspacemakerset.insert(world.register_component::<Workspace2>());
    workspacemakerset.insert(world.register_component::<Workspace3>());
    workspacemakerset.insert(world.register_component::<Workspace4>());
    workspacemakerset.insert(world.register_component::<Workspace5>());
    workspacemakerset.insert(world.register_component::<Workspace6>());
    workspacemakerset.insert(world.register_component::<Workspace7>());
    workspacemakerset.insert(world.register_component::<Workspace8>());
    workspacemakerset.insert(world.register_component::<Workspace9>());
    workspacemakerset.insert(world.register_component::<Workspace10>());
    workspacemakerset.insert(world.register_component::<Workspace11>());
    workspacemakerset.insert(world.register_component::<Workspace12>());
    workspacemakerset.insert(world.register_component::<Workspace13>());
    workspacemakerset.insert(world.register_component::<Workspace14>());
    workspacemakerset.insert(world.register_component::<Workspace15>());
    workspacemakerset.insert(world.register_component::<Workspace16>());
    workspacemakerset.insert(world.register_component::<Workspace17>());
    workspacemakerset.insert(world.register_component::<Workspace18>());
    workspacemakerset.insert(world.register_component::<Workspace19>());
    workspacemakerset.insert(world.register_component::<Workspace20>());
    workspacemakerset.insert(world.register_component::<Workspace21>());
    workspacemakerset.insert(world.register_component::<Workspace22>());
    workspacemakerset.insert(world.register_component::<Workspace23>());
    workspacemakerset.insert(world.register_component::<Workspace24>());
    workspacemakerset.insert(world.register_component::<Workspace25>());
    workspacemakerset.insert(world.register_component::<Workspace26>());
    workspacemakerset.insert(world.register_component::<Workspace27>());
    workspacemakerset.insert(world.register_component::<Workspace28>());
    workspacemakerset.insert(world.register_component::<Workspace29>());
    workspacemakerset.insert(world.register_component::<Workspace30>());
    workspacemakerset.insert(world.register_component::<Workspace31>());
    workspacemakerset.insert(world.register_component::<Workspace32>());
    workspacemakerset.insert(world.register_component::<Workspace33>());
    workspacemakerset.insert(world.register_component::<Workspace34>());
    workspacemakerset.insert(world.register_component::<Workspace35>());
    workspacemakerset.insert(world.register_component::<Workspace36>());
    workspacemakerset.insert(world.register_component::<Workspace37>());
    workspacemakerset.insert(world.register_component::<Workspace38>());
    workspacemakerset.insert(world.register_component::<Workspace39>());
    workspacemakerset.insert(world.register_component::<Workspace40>());
    workspacemakerset.insert(world.register_component::<Workspace41>());
    workspacemakerset.insert(world.register_component::<Workspace42>());
    workspacemakerset.insert(world.register_component::<Workspace43>());
    workspacemakerset.insert(world.register_component::<Workspace44>());
    workspacemakerset.insert(world.register_component::<Workspace45>());
    workspacemakerset.insert(world.register_component::<Workspace46>());
    workspacemakerset.insert(world.register_component::<Workspace47>());
    workspacemakerset.insert(world.register_component::<Workspace48>());
    workspacemakerset.insert(world.register_component::<Workspace49>());
    workspacemakerset.insert(world.register_component::<Workspace50>());
    workspacemakerset.insert(world.register_component::<Workspace51>());
    workspacemakerset.insert(world.register_component::<Workspace52>());
    workspacemakerset.insert(world.register_component::<Workspace53>());
    workspacemakerset.insert(world.register_component::<Workspace54>());
    workspacemakerset.insert(world.register_component::<Workspace55>());
    workspacemakerset.insert(world.register_component::<Workspace56>());
    workspacemakerset.insert(world.register_component::<Workspace57>());
    workspacemakerset.insert(world.register_component::<Workspace58>());
    workspacemakerset.insert(world.register_component::<Workspace59>());
    workspacemakerset.insert(world.register_component::<Workspace60>());
    workspacemakerset.insert(world.register_component::<Workspace61>());
    workspacemakerset.insert(world.register_component::<Workspace62>());
    workspacemakerset.insert(world.register_component::<Workspace63>());
    workspacemakerset.insert(world.register_component::<Workspace64>());

    world.insert_resource(workspacemakerset);

    let mut monitormakerset = Monitormakerset::default();

    monitormakerset.insert(world.register_component::<Monitor1>());
    monitormakerset.insert(world.register_component::<Monitor2>());
    monitormakerset.insert(world.register_component::<Monitor3>());
    monitormakerset.insert(world.register_component::<Monitor4>());
    monitormakerset.insert(world.register_component::<Monitor5>());
    monitormakerset.insert(world.register_component::<Monitor6>());
    monitormakerset.insert(world.register_component::<Monitor7>());
    monitormakerset.insert(world.register_component::<Monitor8>());
    monitormakerset.insert(world.register_component::<Monitor9>());
    monitormakerset.insert(world.register_component::<Monitor10>());
    monitormakerset.insert(world.register_component::<Monitor11>());
    monitormakerset.insert(world.register_component::<Monitor12>());
    monitormakerset.insert(world.register_component::<Monitor13>());
    monitormakerset.insert(world.register_component::<Monitor14>());
    monitormakerset.insert(world.register_component::<Monitor15>());
    monitormakerset.insert(world.register_component::<Monitor16>());
    monitormakerset.insert(world.register_component::<Monitor17>());
    monitormakerset.insert(world.register_component::<Monitor18>());
    monitormakerset.insert(world.register_component::<Monitor19>());
    monitormakerset.insert(world.register_component::<Monitor20>());
    monitormakerset.insert(world.register_component::<Monitor21>());
    monitormakerset.insert(world.register_component::<Monitor22>());
    monitormakerset.insert(world.register_component::<Monitor23>());
    monitormakerset.insert(world.register_component::<Monitor24>());
    monitormakerset.insert(world.register_component::<Monitor25>());
    monitormakerset.insert(world.register_component::<Monitor26>());
    monitormakerset.insert(world.register_component::<Monitor27>());
    monitormakerset.insert(world.register_component::<Monitor28>());
    monitormakerset.insert(world.register_component::<Monitor29>());
    monitormakerset.insert(world.register_component::<Monitor30>());
    monitormakerset.insert(world.register_component::<Monitor31>());
    monitormakerset.insert(world.register_component::<Monitor32>());

    world.insert_resource(monitormakerset);

    let monocle = world.register_component::<MonocleContainer>();
    world.insert_resource(MonocleContainermakerid(monocle));

    let maximizedwindow = world.register_component::<MaximizedWindow>();
    world.insert_resource(MaximizedWindowmakerid(maximizedwindow));
}
