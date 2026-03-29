use crate::{
    particles::SmokeParticlesPlugin,
    plugin::MistSmokeBackend,
    theme::MistTheme,
    SmokeRingPlugin,
};
use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct MistRingSmokePlugin;

impl Plugin for MistRingSmokePlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<SmokeRingPlugin>() {
            app.add_plugins(SmokeRingPlugin);
        }
    }
}

pub struct MistParticleSmokePlugin;

impl Plugin for MistParticleSmokePlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<SmokeParticlesPlugin>() {
            app.add_plugins(SmokeParticlesPlugin);
        }
    }
}

pub struct MistSmokePlugin;

impl Plugin for MistSmokePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MistSmokeBackend>();
        if !app.is_plugin_added::<MistRingSmokePlugin>() {
            app.add_plugins(MistRingSmokePlugin);
        }
        if !app.is_plugin_added::<MistParticleSmokePlugin>() {
            app.add_plugins(MistParticleSmokePlugin);
        }
    }
}

pub struct MistSmokeCorePlugin;

impl Plugin for MistSmokeCorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MistTheme>();
        if !app.is_plugin_added::<MistSmokePlugin>() {
            app.add_plugins(MistSmokePlugin);
        }
    }
}

pub struct MistActionControlsPlugin;

impl Plugin for MistActionControlsPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::widgets::MistUiMessagesPlugin>() {
            app.add_plugins(crate::widgets::MistUiMessagesPlugin);
        }
        if !app.is_plugin_added::<crate::widgets::MistUiSmokeHostPlugin>() {
            app.add_plugins(crate::widgets::MistUiSmokeHostPlugin);
        }
        if !app.is_plugin_added::<crate::widgets::MistUiActionPlugin>() {
            app.add_plugins(crate::widgets::MistUiActionPlugin);
        }
    }
}

pub struct MistToggleControlsPlugin;

impl Plugin for MistToggleControlsPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::widgets::MistUiMessagesPlugin>() {
            app.add_plugins(crate::widgets::MistUiMessagesPlugin);
        }
        if !app.is_plugin_added::<crate::widgets::MistUiSmokeHostPlugin>() {
            app.add_plugins(crate::widgets::MistUiSmokeHostPlugin);
        }
        if !app.is_plugin_added::<crate::widgets::MistUiSelectionPlugin>() {
            app.add_plugins(crate::widgets::MistUiSelectionPlugin);
        }
    }
}

pub struct MistScalarControlsPlugin;

impl Plugin for MistScalarControlsPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::widgets::MistUiMessagesPlugin>() {
            app.add_plugins(crate::widgets::MistUiMessagesPlugin);
        }
        if !app.is_plugin_added::<crate::widgets::MistUiSmokeHostPlugin>() {
            app.add_plugins(crate::widgets::MistUiSmokeHostPlugin);
        }
        if !app.is_plugin_added::<crate::widgets::MistUiScalarPlugin>() {
            app.add_plugins(crate::widgets::MistUiScalarPlugin);
        }
    }
}

pub struct MistOverlayControlsPlugin;

impl Plugin for MistOverlayControlsPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::widgets::MistUiMessagesPlugin>() {
            app.add_plugins(crate::widgets::MistUiMessagesPlugin);
        }
        if !app.is_plugin_added::<crate::widgets::MistUiSmokeHostPlugin>() {
            app.add_plugins(crate::widgets::MistUiSmokeHostPlugin);
        }
        if !app.is_plugin_added::<crate::widgets::MistUiOverlayPlugin>() {
            app.add_plugins(crate::widgets::MistUiOverlayPlugin);
        }
    }
}

pub struct MistFeedbackControlsPlugin;

impl Plugin for MistFeedbackControlsPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::widgets::MistUiMessagesPlugin>() {
            app.add_plugins(crate::widgets::MistUiMessagesPlugin);
        }
        if !app.is_plugin_added::<crate::widgets::MistUiSmokeHostPlugin>() {
            app.add_plugins(crate::widgets::MistUiSmokeHostPlugin);
        }
        if !app.is_plugin_added::<crate::widgets::MistUiFeedbackPlugin>() {
            app.add_plugins(crate::widgets::MistUiFeedbackPlugin);
        }
    }
}

pub struct MistTextEntryControlsPlugin;

impl Plugin for MistTextEntryControlsPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::widgets::MistUiMessagesPlugin>() {
            app.add_plugins(crate::widgets::MistUiMessagesPlugin);
        }
        if !app.is_plugin_added::<crate::widgets::MistUiSmokeHostPlugin>() {
            app.add_plugins(crate::widgets::MistUiSmokeHostPlugin);
        }
        if !app.is_plugin_added::<crate::widgets::MistUiInputPlugin>() {
            app.add_plugins(crate::widgets::MistUiInputPlugin);
        }
    }
}

pub struct MistDataViewControlsPlugin;

impl Plugin for MistDataViewControlsPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::widgets::MistUiMessagesPlugin>() {
            app.add_plugins(crate::widgets::MistUiMessagesPlugin);
        }
        if !app.is_plugin_added::<crate::widgets::MistUiSmokeHostPlugin>() {
            app.add_plugins(crate::widgets::MistUiSmokeHostPlugin);
        }
        if !app.is_plugin_added::<crate::widgets::MistUiDataViewPlugin>() {
            app.add_plugins(crate::widgets::MistUiDataViewPlugin);
        }
    }
}

pub struct MistUiPlugins;

impl PluginGroup for MistUiPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(MistSmokeCorePlugin)
            .add(MistActionControlsPlugin)
            .add(MistToggleControlsPlugin)
            .add(MistScalarControlsPlugin)
            .add(MistOverlayControlsPlugin)
            .add(MistFeedbackControlsPlugin)
            .add(MistTextEntryControlsPlugin)
            .add(MistDataViewControlsPlugin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MistSmokeBackend, MistSmokeBudget, MistTheme};
    use bevy::asset::AssetPlugin;

    #[test]
    fn plugin_group_registers_smoke_backend_and_budget() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));
        app.add_plugins(MistUiPlugins);

        assert_eq!(
            *app.world().resource::<MistSmokeBackend>(),
            MistSmokeBackend::Particles
        );
        assert!(app.world().contains_resource::<MistTheme>());
        assert!(app.world().contains_resource::<MistSmokeBudget>());
        assert!(app.is_plugin_added::<MistRingSmokePlugin>());
        assert!(app.is_plugin_added::<MistParticleSmokePlugin>());
        assert!(app.is_plugin_added::<MistSmokeCorePlugin>());
        assert!(app.is_plugin_added::<MistActionControlsPlugin>());
        assert!(app.is_plugin_added::<MistToggleControlsPlugin>());
        assert!(app.is_plugin_added::<MistScalarControlsPlugin>());
        assert!(app.is_plugin_added::<MistOverlayControlsPlugin>());
        assert!(app.is_plugin_added::<MistFeedbackControlsPlugin>());
        assert!(app.is_plugin_added::<MistTextEntryControlsPlugin>());
        assert!(app.is_plugin_added::<MistDataViewControlsPlugin>());
    }
}
