use crate::{plugin::SmokeRingPadding, MistSmokeConfig};
use bevy::prelude::*;

#[derive(Resource, Clone, Debug, Reflect)]
pub struct MistTheme {
    pub edge_padding_scale: f32,
    pub frame_thickness_scale: f32,
    pub frame_intensity_scale: f32,
    pub frame_density_scale: f32,
    pub surface_intensity_scale: f32,
    pub surface_density_scale: f32,
    pub accent_boost: f32,
    pub handle_boost: f32,
    pub veil_boost: f32,
    pub panel_fill: Color,
    pub control_fill: Color,
    pub hovered_fill: Color,
    pub pressed_fill: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
}

impl Default for MistTheme {
    fn default() -> Self {
        Self {
            edge_padding_scale: 1.0,
            frame_thickness_scale: 1.08,
            frame_intensity_scale: 1.06,
            frame_density_scale: 1.14,
            surface_intensity_scale: 1.08,
            surface_density_scale: 1.10,
            accent_boost: 1.18,
            handle_boost: 1.14,
            veil_boost: 1.10,
            panel_fill: Color::srgba(0.01, 0.03, 0.05, 0.30),
            control_fill: Color::srgba(0.0, 0.02, 0.04, 0.08),
            hovered_fill: Color::srgba(0.01, 0.04, 0.07, 0.11),
            pressed_fill: Color::srgba(0.02, 0.05, 0.09, 0.14),
            text_primary: Color::srgba(0.97, 0.99, 1.0, 0.98),
            text_secondary: Color::srgba(0.82, 0.88, 0.96, 0.96),
        }
    }
}

impl MistTheme {
    pub fn scaled_padding(&self, base: f32) -> SmokeRingPadding {
        SmokeRingPadding::all((base * self.edge_padding_scale).max(0.0))
    }

    pub fn apply_frame_config(&self, mut config: MistSmokeConfig, emphasis: f32) -> MistSmokeConfig {
        let emphasis = emphasis.max(0.5);
        config.thickness = (config.thickness * self.frame_thickness_scale * emphasis).clamp(0.04, 0.45);
        config.intensity = (config.intensity * self.frame_intensity_scale * emphasis).clamp(0.4, 8.5);
        config.particle_density =
            (config.particle_density * self.frame_density_scale * emphasis).clamp(0.2, 5.5);
        config.particle_size_scale =
            (config.particle_size_scale * emphasis.sqrt()).clamp(0.5, 1.8);
        config
    }

    pub fn apply_surface_config(
        &self,
        mut config: MistSmokeConfig,
        emphasis: f32,
        is_accent: bool,
        is_handle: bool,
        is_veil: bool,
    ) -> MistSmokeConfig {
        let mut factor = emphasis.max(0.5);
        if is_accent {
            factor *= self.accent_boost;
        }
        if is_handle {
            factor *= self.handle_boost;
        }
        if is_veil {
            factor *= self.veil_boost;
        }
        config.intensity =
            (config.intensity * self.surface_intensity_scale * factor).clamp(0.4, 6.5);
        config.particle_density =
            (config.particle_density * self.surface_density_scale * factor).clamp(0.2, 4.5);
        config.particle_size_scale =
            (config.particle_size_scale * factor.sqrt()).clamp(0.5, 1.9);
        config
    }
}
