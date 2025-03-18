use bevy_mod_scripting::core::callback_labels;

callback_labels!(
    OnPreStartUp => "on_pre_startup",
    OnStartUp => "on_startup",
    OnPostStartUp => "on_post_startup",
    OnPreUpdate => "on_pre_update",
    OnUpdate => "on_update",
    OnPostUpdate => "on_post_update"
);

impl Default for OnUpdate {
    fn default() -> Self {
        Self
    }
}

impl Default for OnPreUpdate {
    fn default() -> Self {
        Self
    }
}

impl Default for OnPostUpdate {
    fn default() -> Self {
        Self
    }
}

impl Default for OnStartUp {
    fn default() -> Self {
        Self
    }
}

impl Default for OnPreStartUp {
    fn default() -> Self {
        Self
    }
}

impl Default for OnPostStartUp {
    fn default() -> Self {
        Self
    }
}

