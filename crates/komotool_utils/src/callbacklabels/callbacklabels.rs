use bevy_mod_scripting::core::callback_labels;

callback_labels!(
    OnPreStartUp => "on_pre_startup",
    OnStartUp => "on_startup",
    OnPostStartUp => "on_post_startup",
    OnPreUpdate => "on_pre_update",
    OnUpdate => "on_update",
    OnPostUpdate => "on_post_update"
);
