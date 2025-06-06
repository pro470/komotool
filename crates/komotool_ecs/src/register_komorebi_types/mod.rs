use bevy_app::App;
use bevy_mod_scripting::core::bindings::{DynamicComponent, ScriptValue};
use komorebi_client::*;

pub fn register_komorebi_types(app: &mut App) {
    app.register_type::<Monitor>()
        .register_type::<Window>()
        .register_type::<Container>()
        .register_type::<Workspace>()
        .register_type::<AnimationPrefix>()
        //.register_type::<PerAnimationPrefixConfig>()
        .register_type::<ApplicationSpecificConfiguration>()
        .register_type::<BorderInfo>()
        .register_type::<Colour>()
        .register_type::<Rgb>()
        .register_type::<ApplicationConfiguration>()
        .register_type::<IdWithIdentifier>()
        .register_type::<IdWithIdentifierAndComment>()
        .register_type::<MatchingRule>()
        .register_type::<MatchingStrategy>()
        .register_type::<ApplicationConfigurationGenerator>()
        .register_type::<AnimationStyle>()
        .register_type::<ApplicationIdentifier>()
        .register_type::<Axis>()
        .register_type::<BorderImplementation>()
        .register_type::<BorderStyle>()
        .register_type::<Column>()
        .register_type::<ColumnSplit>()
        .register_type::<ColumnSplitWithCapacity>()
        .register_type::<ColumnWidth>()
        .register_type::<CustomLayout>()
        .register_type::<CycleDirection>()
        .register_type::<DefaultLayout>()
        .register_type::<FocusFollowsMouseImplementation>()
        .register_type::<HidingBehaviour>()
        .register_type::<Layout>()
        .register_type::<MoveBehaviour>()
        .register_type::<OperationBehaviour>()
        .register_type::<OperationDirection>()
        .register_type::<Rect>()
        .register_type::<Sizing>()
        .register_type::<SocketMessage>()
        .register_type::<StackbarLabel>()
        .register_type::<StackbarMode>()
        .register_type::<StateQuery>()
        .register_type::<WindowKind>()
        .register_type::<MonitorNotification>()
        .register_type::<Ring<Monitor>>()
        .register_type::<Ring<Workspace>>()
        .register_type::<Ring<Container>>()
        .register_type::<Ring<Window>>()
        .register_type::<Ring<ScriptValue>>()
        .register_type::<Ring<DynamicComponent>>()
        .register_type::<WindowManagerEvent>()
        .register_type::<WorkspaceGlobals>()
        .register_type::<WorkspaceLayer>()
        .register_type::<AnimationsConfig>()
        .register_type::<AppSpecificConfigurationPath>()
        .register_type::<AspectRatio>()
        .register_type::<BorderColours>()
        .register_type::<CrossBoundaryBehaviour>()
        .register_type::<GlobalState>()
        .register_type::<KomorebiTheme>()
        .register_type::<MonitorConfig>()
        .register_type::<Notification>()
        .register_type::<NotificationEvent>()
        .register_type::<PredefinedAspectRatio>()
        .register_type::<RuleDebug>()
        .register_type::<StackbarConfig>()
        .register_type::<State>()
        .register_type::<StaticConfig>()
        .register_type::<SubscribeOptions>()
        .register_type::<TabsConfig>()
        .register_type::<WindowContainerBehaviour>()
        .register_type::<WindowsApi>()
        .register_type::<WorkspaceConfig>();
}
