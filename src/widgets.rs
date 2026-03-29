use crate::{
    derived_screen_ring,
    particles::{
        MistSmokeConfig, MistSmokeParticle, MistSmokePreset, MistSmokeSurface, MistSmokeTarget,
    },
    theme::MistTheme,
    SmokeBorder, SmokeRingPadding,
};
use bevy::input::{
    keyboard::{Key, KeyCode, KeyboardInput},
    mouse::{MouseButton, MouseScrollUnit, MouseWheel},
    ButtonInput, ButtonState,
};
use bevy::prelude::*;
use bevy::text::{Justify, TextLayout};
use bevy::ui::{ComputedNode, Overflow, RelativeCursorPosition, ScrollPosition};

const CONTROL_HEIGHT: f32 = 50.0;
const CONTROL_RADIUS: f32 = 14.0;
const CONTROL_PADDING_X: f32 = 14.0;
const CONTROL_PADDING_Y: f32 = 10.0;
const SLIDER_HANDLE_WIDTH: f32 = 22.0;
const SLIDER_HANDLE_HEIGHT: f32 = 32.0;
const SLIDER_TRACK_HEIGHT: f32 = 10.0;
const SLIDER_TRACK_INSET: f32 = 16.0;
const PROGRESS_LERP_RATE: f32 = 6.0;
const SWITCH_TRACK_WIDTH: f32 = 54.0;
const SWITCH_TRACK_HEIGHT: f32 = 28.0;
const SWITCH_KNOB_SIZE: f32 = 22.0;
const SCROLL_STEP_PX: f32 = 36.0;
const SCROLLBAR_WIDTH: f32 = 10.0;
const SCROLLBAR_MIN_THUMB_HEIGHT: f32 = 30.0;

fn clamp_choice_index(len: usize, index: usize) -> usize {
    index.min(len.saturating_sub(1))
}

fn panel_fill() -> Color {
    MistTheme::default().panel_fill
}

fn control_fill() -> Color {
    MistTheme::default().control_fill
}

fn hovered_fill() -> Color {
    MistTheme::default().hovered_fill
}

fn pressed_fill() -> Color {
    MistTheme::default().pressed_fill
}

fn text_primary() -> Color {
    MistTheme::default().text_primary
}

fn text_secondary() -> Color {
    MistTheme::default().text_secondary
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
enum MistSmokeRole {
    StandardButton,
    TriggerButton,
    ToolbarButton,
    DropdownOption,
    PanelFrame,
    MenuFrame,
    DataFrame,
    DataItem,
    FeedbackChip,
    ScrollContainer,
    DialogFrame,
    ScalarControl,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MistSmokeSurfaceRole {
    ControlBody,
    ContainerBody,
    OptionBody,
    MenuBody,
    DataBody,
    HeaderBody,
    FeedbackBody,
    AccentChip,
    AccentOrb,
    ScalarTrack,
    ScalarFill,
    BackdropVeil,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MistEdgeState {
    Idle,
    Hovered,
    Pressed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MistSurfaceState {
    Idle,
    Active,
    Pressed,
}

fn edge_state_from_interaction(interaction: Interaction) -> MistEdgeState {
    match interaction {
        Interaction::None => MistEdgeState::Idle,
        Interaction::Hovered => MistEdgeState::Hovered,
        Interaction::Pressed => MistEdgeState::Pressed,
    }
}

fn surface_state_from_interaction(interaction: Interaction) -> MistSurfaceState {
    match interaction {
        Interaction::None => MistSurfaceState::Idle,
        Interaction::Hovered => MistSurfaceState::Active,
        Interaction::Pressed => MistSurfaceState::Pressed,
    }
}

fn smoke_padding_for_role(role: MistSmokeRole) -> SmokeRingPadding {
    let theme = MistTheme::default();
    let base = match role {
        MistSmokeRole::PanelFrame | MistSmokeRole::ScrollContainer | MistSmokeRole::DialogFrame => {
            10.0
        }
        MistSmokeRole::MenuFrame | MistSmokeRole::DataFrame => 9.0,
        MistSmokeRole::FeedbackChip => 8.0,
        MistSmokeRole::DropdownOption | MistSmokeRole::DataItem => 7.0,
        MistSmokeRole::TriggerButton => 12.0,
        MistSmokeRole::StandardButton
        | MistSmokeRole::ToolbarButton
        | MistSmokeRole::ScalarControl => 9.0,
    };
    theme.scaled_padding(base)
}

fn smoke_config_for_edge_state(role: MistSmokeRole, state: MistEdgeState) -> MistSmokeConfig {
    let theme = MistTheme::default();
    let mut smoke = match role {
        MistSmokeRole::StandardButton => {
            MistSmokeConfig::screen_preset(MistSmokePreset::StandardButton)
        }
        MistSmokeRole::TriggerButton | MistSmokeRole::FeedbackChip => {
            MistSmokeConfig::screen_preset(MistSmokePreset::PrimaryAction)
        }
        MistSmokeRole::ToolbarButton => {
            MistSmokeConfig::screen_preset(MistSmokePreset::ToolbarButton)
        }
        MistSmokeRole::DropdownOption | MistSmokeRole::DataItem => {
            MistSmokeConfig::screen_preset(MistSmokePreset::DropdownOption)
        }
        MistSmokeRole::PanelFrame | MistSmokeRole::MenuFrame | MistSmokeRole::DataFrame => {
            MistSmokeConfig::screen_preset(MistSmokePreset::PanelFrame)
        }
        MistSmokeRole::ScrollContainer => {
            MistSmokeConfig::screen_preset(MistSmokePreset::ScrollbarTrack)
        }
        MistSmokeRole::DialogFrame => MistSmokeConfig::screen_preset(MistSmokePreset::DialogFrame),
        MistSmokeRole::ScalarControl => {
            MistSmokeConfig::screen_preset(MistSmokePreset::ScrollbarThumb)
        }
    };

    match role {
        MistSmokeRole::StandardButton => {
            let (thickness, intensity, softness, flow, pulse, density, size) = match state {
                MistEdgeState::Idle => (0.15, 3.80, 0.25, 0.78, 0.10, 1.95, 0.96),
                MistEdgeState::Hovered => (0.17, 4.25, 0.28, 0.82, 0.14, 2.18, 1.02),
                MistEdgeState::Pressed => (0.19, 4.80, 0.31, 0.88, 0.18, 2.42, 1.08),
            };
            smoke.thickness = thickness;
            smoke.intensity = intensity;
            smoke.softness = softness;
            smoke.flow_speed = flow;
            smoke.noise_scale = 25.0;
            smoke.pulse_strength = pulse;
            smoke.particle_density = density;
            smoke.particle_size_scale = size;
        }
        MistSmokeRole::TriggerButton => {
            let (thickness, intensity, softness, flow, pulse, density, size) = match state {
                MistEdgeState::Idle => (0.20, 4.85, 0.26, 0.90, 0.18, 2.68, 1.14),
                MistEdgeState::Hovered => (0.23, 5.50, 0.29, 0.98, 0.22, 2.98, 1.22),
                MistEdgeState::Pressed => (0.26, 6.15, 0.32, 1.06, 0.28, 3.32, 1.32),
            };
            smoke.thickness = thickness;
            smoke.intensity = intensity;
            smoke.softness = softness;
            smoke.flow_speed = flow;
            smoke.noise_scale = 21.0;
            smoke.pulse_strength = pulse;
            smoke.particle_density = density;
            smoke.particle_size_scale = size;
        }
        MistSmokeRole::ToolbarButton => {
            let (thickness, intensity, softness, flow, pulse, density, size) = match state {
                MistEdgeState::Idle => (0.14, 3.65, 0.24, 0.82, 0.12, 1.86, 0.94),
                MistEdgeState::Hovered => (0.16, 4.10, 0.27, 0.88, 0.16, 2.08, 1.00),
                MistEdgeState::Pressed => (0.18, 4.60, 0.30, 0.95, 0.20, 2.32, 1.06),
            };
            smoke.thickness = thickness;
            smoke.intensity = intensity;
            smoke.softness = softness;
            smoke.flow_speed = flow;
            smoke.noise_scale = 24.0;
            smoke.pulse_strength = pulse;
            smoke.particle_density = density;
            smoke.particle_size_scale = size;
        }
        MistSmokeRole::DropdownOption => {
            let (thickness, intensity, softness, flow, pulse, density, size) = match state {
                MistEdgeState::Idle => (0.12, 3.05, 0.28, 0.74, 0.10, 1.64, 0.92),
                MistEdgeState::Hovered => (0.14, 3.45, 0.30, 0.78, 0.13, 1.84, 0.97),
                MistEdgeState::Pressed => (0.16, 3.90, 0.33, 0.84, 0.16, 2.04, 1.02),
            };
            smoke.thickness = thickness;
            smoke.intensity = intensity;
            smoke.softness = softness;
            smoke.flow_speed = flow;
            smoke.noise_scale = 24.0;
            smoke.pulse_strength = pulse;
            smoke.particle_density = density;
            smoke.particle_size_scale = size;
        }
        MistSmokeRole::PanelFrame | MistSmokeRole::MenuFrame | MistSmokeRole::DataFrame => {
            let (thickness, intensity, softness, flow, pulse, density, size) = match state {
                MistEdgeState::Idle => (0.16, 3.55, 0.31, 0.70, 0.10, 1.96, 1.02),
                MistEdgeState::Hovered => (0.18, 4.00, 0.33, 0.76, 0.13, 2.16, 1.08),
                MistEdgeState::Pressed => (0.20, 4.45, 0.35, 0.82, 0.17, 2.36, 1.14),
            };
            smoke.thickness = thickness;
            smoke.intensity = intensity;
            smoke.softness = softness;
            smoke.flow_speed = flow;
            smoke.noise_scale = 25.0;
            smoke.pulse_strength = pulse;
            smoke.particle_density = density;
            smoke.particle_size_scale = size;
        }
        MistSmokeRole::DataItem => {
            let (thickness, intensity, softness, flow, pulse, density, size) = match state {
                MistEdgeState::Idle => (0.13, 3.10, 0.28, 0.76, 0.10, 1.72, 0.94),
                MistEdgeState::Hovered => (0.15, 3.55, 0.30, 0.82, 0.14, 1.92, 1.00),
                MistEdgeState::Pressed => (0.17, 4.00, 0.33, 0.88, 0.18, 2.16, 1.06),
            };
            smoke.thickness = thickness;
            smoke.intensity = intensity;
            smoke.softness = softness;
            smoke.flow_speed = flow;
            smoke.noise_scale = 23.0;
            smoke.pulse_strength = pulse;
            smoke.particle_density = density;
            smoke.particle_size_scale = size;
        }
        MistSmokeRole::FeedbackChip => {
            let (thickness, intensity, softness, flow, pulse, density, size) = match state {
                MistEdgeState::Idle => (0.18, 4.35, 0.26, 0.90, 0.18, 2.30, 1.08),
                MistEdgeState::Hovered => (0.20, 4.90, 0.28, 0.98, 0.22, 2.56, 1.16),
                MistEdgeState::Pressed => (0.23, 5.45, 0.31, 1.06, 0.28, 2.82, 1.24),
            };
            smoke.thickness = thickness;
            smoke.intensity = intensity;
            smoke.softness = softness;
            smoke.flow_speed = flow;
            smoke.noise_scale = 22.0;
            smoke.pulse_strength = pulse;
            smoke.particle_density = density;
            smoke.particle_size_scale = size;
        }
        MistSmokeRole::ScrollContainer => {
            let (thickness, intensity, softness, flow, pulse, density, size) = match state {
                MistEdgeState::Idle => (0.15, 3.30, 0.32, 0.66, 0.10, 1.82, 0.98),
                MistEdgeState::Hovered => (0.16, 3.68, 0.34, 0.70, 0.12, 2.00, 1.02),
                MistEdgeState::Pressed => (0.18, 4.05, 0.36, 0.74, 0.15, 2.18, 1.06),
            };
            smoke.thickness = thickness;
            smoke.intensity = intensity;
            smoke.softness = softness;
            smoke.flow_speed = flow;
            smoke.noise_scale = 25.0;
            smoke.pulse_strength = pulse;
            smoke.particle_density = density;
            smoke.particle_size_scale = size;
        }
        MistSmokeRole::DialogFrame => {
            let (thickness, intensity, softness, flow, pulse, density, size) = match state {
                MistEdgeState::Idle => (0.17, 3.70, 0.32, 0.72, 0.11, 1.98, 1.02),
                MistEdgeState::Hovered => (0.19, 4.10, 0.34, 0.76, 0.14, 2.18, 1.08),
                MistEdgeState::Pressed => (0.21, 4.55, 0.36, 0.82, 0.18, 2.38, 1.14),
            };
            smoke.thickness = thickness;
            smoke.intensity = intensity;
            smoke.softness = softness;
            smoke.flow_speed = flow;
            smoke.noise_scale = 26.0;
            smoke.pulse_strength = pulse;
            smoke.particle_density = density;
            smoke.particle_size_scale = size;
        }
        MistSmokeRole::ScalarControl => {
            let (thickness, intensity, softness, flow, pulse, density, size) = match state {
                MistEdgeState::Idle => (0.14, 3.25, 0.28, 0.80, 0.11, 1.78, 0.96),
                MistEdgeState::Hovered => (0.16, 3.70, 0.30, 0.85, 0.14, 1.98, 1.00),
                MistEdgeState::Pressed => (0.18, 4.10, 0.32, 0.90, 0.18, 2.18, 1.05),
            };
            smoke.thickness = thickness;
            smoke.intensity = intensity;
            smoke.softness = softness;
            smoke.flow_speed = flow;
            smoke.noise_scale = 24.0;
            smoke.pulse_strength = pulse;
            smoke.particle_density = density;
            smoke.particle_size_scale = size;
        }
    }

    densify_edge_smoke(
        theme.apply_frame_config(
            smoke,
            match role {
                MistSmokeRole::TriggerButton => 1.10,
                MistSmokeRole::FeedbackChip => 1.08,
                MistSmokeRole::DialogFrame
                | MistSmokeRole::PanelFrame
                | MistSmokeRole::MenuFrame
                | MistSmokeRole::DataFrame => 1.04,
                MistSmokeRole::DropdownOption | MistSmokeRole::DataItem => 0.98,
                _ => 1.0,
            },
        ),
        role,
    )
}

fn smoke_config_for_role(role: MistSmokeRole, pressed: bool) -> MistSmokeConfig {
    smoke_config_for_edge_state(
        role,
        if pressed {
            MistEdgeState::Pressed
        } else {
            MistEdgeState::Idle
        },
    )
}

fn checkbox_indicator_smoke_config(checked: bool) -> MistSmokeConfig {
    let mut smoke = MistSmokeConfig::screen_preset(MistSmokePreset::DropdownOption);
    smoke.thickness = if checked { 0.15 } else { 0.13 };
    smoke.intensity = if checked { 5.25 } else { 4.60 };
    smoke.softness = if checked { 0.21 } else { 0.19 };
    smoke.flow_speed = if checked { 1.38 } else { 1.24 };
    smoke.noise_scale = 20.0;
    smoke.pulse_strength = if checked { 0.15 } else { 0.10 };
    smoke.particle_density = if checked { 3.72 } else { 3.28 };
    smoke.particle_size_scale = if checked { 0.68 } else { 0.62 };
    smoke
}

fn densify_edge_smoke(mut smoke: MistSmokeConfig, role: MistSmokeRole) -> MistSmokeConfig {
    let (density_gain, size_gain) = match role {
        MistSmokeRole::TriggerButton | MistSmokeRole::FeedbackChip => (1.22, 0.94),
        MistSmokeRole::DialogFrame
        | MistSmokeRole::PanelFrame
        | MistSmokeRole::MenuFrame
        | MistSmokeRole::DataFrame => (1.18, 0.95),
        MistSmokeRole::StandardButton => (1.18, 0.93),
        MistSmokeRole::ToolbarButton
        | MistSmokeRole::DropdownOption
        | MistSmokeRole::DataItem
        | MistSmokeRole::ScrollContainer
        | MistSmokeRole::ScalarControl => (1.16, 0.94),
    };

    smoke.particle_density *= density_gain;
    smoke.particle_size_scale = (smoke.particle_size_scale * size_gain).max(0.58);
    smoke
}

fn smoke_border_for_role_state(
    role: MistSmokeRole,
    state: MistEdgeState,
    _seed: u64,
) -> SmokeBorder {
    derived_screen_ring(smoke_config_for_edge_state(role, state))
}

fn smoke_border_for_role(role: MistSmokeRole, pressed: bool, seed: u64) -> SmokeBorder {
    smoke_border_for_role_state(
        role,
        if pressed {
            MistEdgeState::Pressed
        } else {
            MistEdgeState::Idle
        },
        seed,
    )
}

fn surface_smoke_for_role_state(
    role: MistSmokeSurfaceRole,
    state: MistSurfaceState,
) -> MistSmokeSurface {
    let theme = MistTheme::default();
    let (mut config, inset) = match role {
        MistSmokeSurfaceRole::ControlBody => (
            MistSmokeConfig::screen_preset(MistSmokePreset::StandardButton),
            Vec2::new(8.0, 6.0),
        ),
        MistSmokeSurfaceRole::ContainerBody => (
            MistSmokeConfig::screen_preset(MistSmokePreset::PanelFrame),
            Vec2::new(10.0, 8.0),
        ),
        MistSmokeSurfaceRole::OptionBody => (
            MistSmokeConfig::screen_preset(MistSmokePreset::DropdownOption),
            Vec2::new(6.0, 4.0),
        ),
        MistSmokeSurfaceRole::MenuBody => (
            MistSmokeConfig::screen_preset(MistSmokePreset::PanelFrame),
            Vec2::new(8.0, 6.0),
        ),
        MistSmokeSurfaceRole::DataBody => (
            MistSmokeConfig::screen_preset(MistSmokePreset::PanelFrame),
            Vec2::new(8.0, 6.0),
        ),
        MistSmokeSurfaceRole::HeaderBody => (
            MistSmokeConfig::screen_preset(MistSmokePreset::ToolbarButton),
            Vec2::new(6.0, 4.0),
        ),
        MistSmokeSurfaceRole::FeedbackBody => (
            MistSmokeConfig::screen_preset(MistSmokePreset::PrimaryAction),
            Vec2::new(5.0, 3.0),
        ),
        MistSmokeSurfaceRole::AccentChip => (
            MistSmokeConfig::screen_preset(MistSmokePreset::PrimaryAction),
            Vec2::new(2.0, 2.0),
        ),
        MistSmokeSurfaceRole::AccentOrb => (
            MistSmokeConfig::screen_preset(MistSmokePreset::PrimaryAction),
            Vec2::new(1.0, 1.0),
        ),
        MistSmokeSurfaceRole::ScalarTrack => (
            MistSmokeConfig::screen_preset(MistSmokePreset::ScrollbarTrack),
            Vec2::new(2.0, 2.0),
        ),
        MistSmokeSurfaceRole::ScalarFill => (
            MistSmokeConfig::screen_preset(MistSmokePreset::ScrollbarThumb),
            Vec2::new(1.0, 1.0),
        ),
        MistSmokeSurfaceRole::BackdropVeil => (
            MistSmokeConfig::screen_preset(MistSmokePreset::PanelFrame),
            Vec2::ZERO,
        ),
    };

    match role {
        MistSmokeSurfaceRole::ControlBody => {
            let (thickness, intensity, flow, softness, pulse, density, size) = match state {
                MistSurfaceState::Idle => (0.11, 1.75, 0.62, 0.56, 0.10, 0.78, 0.88),
                MistSurfaceState::Active => (0.14, 2.20, 0.70, 0.52, 0.14, 1.02, 0.98),
                MistSurfaceState::Pressed => (0.17, 2.75, 0.80, 0.48, 0.18, 1.28, 1.08),
            };
            config.thickness = thickness;
            config.intensity = intensity;
            config.flow_speed = flow;
            config.noise_scale = 36.0;
            config.softness = softness;
            config.pulse_strength = pulse;
            config.particle_density = density;
            config.particle_size_scale = size;
        }
        MistSmokeSurfaceRole::ContainerBody => {
            let (thickness, intensity, flow, softness, pulse, density, size) = match state {
                MistSurfaceState::Idle => (0.14, 1.95, 0.56, 0.60, 0.08, 0.90, 0.98),
                MistSurfaceState::Active => (0.17, 2.40, 0.62, 0.56, 0.11, 1.18, 1.06),
                MistSurfaceState::Pressed => (0.20, 2.95, 0.68, 0.54, 0.15, 1.46, 1.14),
            };
            config.thickness = thickness;
            config.intensity = intensity;
            config.flow_speed = flow;
            config.noise_scale = 38.0;
            config.softness = softness;
            config.pulse_strength = pulse;
            config.particle_density = density;
            config.particle_size_scale = size;
        }
        MistSmokeSurfaceRole::OptionBody => {
            let (thickness, intensity, flow, softness, pulse, density, size) = match state {
                MistSurfaceState::Idle => (0.11, 1.55, 0.60, 0.54, 0.08, 0.64, 0.82),
                MistSurfaceState::Active => (0.13, 1.95, 0.66, 0.50, 0.11, 0.88, 0.90),
                MistSurfaceState::Pressed => (0.16, 2.35, 0.74, 0.46, 0.15, 1.12, 1.00),
            };
            config.thickness = thickness;
            config.intensity = intensity;
            config.flow_speed = flow;
            config.noise_scale = 34.0;
            config.softness = softness;
            config.pulse_strength = pulse;
            config.particle_density = density;
            config.particle_size_scale = size;
        }
        MistSmokeSurfaceRole::MenuBody => {
            let (thickness, intensity, flow, softness, pulse, density, size) = match state {
                MistSurfaceState::Idle => (0.13, 1.90, 0.58, 0.58, 0.08, 0.86, 0.94),
                MistSurfaceState::Active => (0.16, 2.35, 0.64, 0.54, 0.11, 1.12, 1.02),
                MistSurfaceState::Pressed => (0.18, 2.80, 0.70, 0.50, 0.15, 1.34, 1.10),
            };
            config.thickness = thickness;
            config.intensity = intensity;
            config.flow_speed = flow;
            config.noise_scale = 38.0;
            config.softness = softness;
            config.pulse_strength = pulse;
            config.particle_density = density;
            config.particle_size_scale = size;
        }
        MistSmokeSurfaceRole::DataBody => {
            let (thickness, intensity, flow, softness, pulse, density, size) = match state {
                MistSurfaceState::Idle => (0.12, 1.78, 0.58, 0.56, 0.08, 0.80, 0.90),
                MistSurfaceState::Active => (0.14, 2.18, 0.64, 0.52, 0.11, 1.04, 0.98),
                MistSurfaceState::Pressed => (0.17, 2.60, 0.70, 0.48, 0.15, 1.28, 1.06),
            };
            config.thickness = thickness;
            config.intensity = intensity;
            config.flow_speed = flow;
            config.noise_scale = 36.0;
            config.softness = softness;
            config.pulse_strength = pulse;
            config.particle_density = density;
            config.particle_size_scale = size;
        }
        MistSmokeSurfaceRole::HeaderBody => {
            let (thickness, intensity, flow, softness, pulse, density, size) = match state {
                MistSurfaceState::Idle => (0.11, 1.70, 0.60, 0.52, 0.08, 0.72, 0.88),
                MistSurfaceState::Active => (0.13, 2.05, 0.66, 0.48, 0.11, 0.94, 0.94),
                MistSurfaceState::Pressed => (0.15, 2.45, 0.72, 0.45, 0.15, 1.16, 1.00),
            };
            config.thickness = thickness;
            config.intensity = intensity;
            config.flow_speed = flow;
            config.noise_scale = 34.0;
            config.softness = softness;
            config.pulse_strength = pulse;
            config.particle_density = density;
            config.particle_size_scale = size;
        }
        MistSmokeSurfaceRole::FeedbackBody => {
            let (thickness, intensity, flow, softness, pulse, density, size) = match state {
                MistSurfaceState::Idle => (0.13, 2.10, 0.68, 0.48, 0.12, 0.92, 0.96),
                MistSurfaceState::Active => (0.16, 2.55, 0.76, 0.44, 0.16, 1.18, 1.04),
                MistSurfaceState::Pressed => (0.18, 3.00, 0.84, 0.42, 0.22, 1.40, 1.12),
            };
            config.thickness = thickness;
            config.intensity = intensity;
            config.flow_speed = flow;
            config.noise_scale = 32.0;
            config.softness = softness;
            config.pulse_strength = pulse;
            config.particle_density = density;
            config.particle_size_scale = size;
        }
        MistSmokeSurfaceRole::AccentChip => {
            let (thickness, intensity, flow, softness, pulse, density, size) = match state {
                MistSurfaceState::Idle => (0.12, 2.20, 0.68, 0.42, 0.14, 1.10, 0.96),
                MistSurfaceState::Active => (0.16, 3.10, 0.82, 0.40, 0.20, 1.70, 1.10),
                MistSurfaceState::Pressed => (0.19, 3.85, 0.94, 0.38, 0.26, 2.10, 1.22),
            };
            config.thickness = thickness;
            config.intensity = intensity;
            config.flow_speed = flow;
            config.noise_scale = 30.0;
            config.softness = softness;
            config.pulse_strength = pulse;
            config.particle_density = density;
            config.particle_size_scale = size;
        }
        MistSmokeSurfaceRole::AccentOrb => {
            let (thickness, intensity, flow, softness, pulse, density, size) = match state {
                MistSurfaceState::Idle => (0.14, 2.35, 0.72, 0.42, 0.14, 1.22, 1.02),
                MistSurfaceState::Active => (0.18, 3.25, 0.86, 0.40, 0.20, 1.82, 1.14),
                MistSurfaceState::Pressed => (0.21, 3.95, 0.96, 0.38, 0.26, 2.24, 1.24),
            };
            config.thickness = thickness;
            config.intensity = intensity;
            config.flow_speed = flow;
            config.noise_scale = 28.0;
            config.softness = softness;
            config.pulse_strength = pulse;
            config.particle_density = density;
            config.particle_size_scale = size;
        }
        MistSmokeSurfaceRole::ScalarTrack => {
            let (thickness, intensity, flow, softness, pulse, density, size) = match state {
                MistSurfaceState::Idle => (0.13, 1.95, 0.68, 0.48, 0.12, 0.90, 1.00),
                MistSurfaceState::Active => (0.16, 2.55, 0.80, 0.46, 0.16, 1.28, 1.10),
                MistSurfaceState::Pressed => (0.19, 3.10, 0.88, 0.44, 0.20, 1.60, 1.18),
            };
            config.thickness = thickness;
            config.intensity = intensity;
            config.flow_speed = flow;
            config.noise_scale = 30.0;
            config.softness = softness;
            config.pulse_strength = pulse;
            config.particle_density = density;
            config.particle_size_scale = size;
        }
        MistSmokeSurfaceRole::ScalarFill => {
            let (thickness, intensity, flow, softness, pulse, density, size) = match state {
                MistSurfaceState::Idle => (0.15, 2.65, 0.74, 0.44, 0.14, 1.28, 1.08),
                MistSurfaceState::Active => (0.18, 3.35, 0.86, 0.42, 0.18, 1.72, 1.18),
                MistSurfaceState::Pressed => (0.21, 3.95, 0.94, 0.40, 0.22, 2.08, 1.28),
            };
            config.thickness = thickness;
            config.intensity = intensity;
            config.flow_speed = flow;
            config.noise_scale = 28.0;
            config.softness = softness;
            config.pulse_strength = pulse;
            config.particle_density = density;
            config.particle_size_scale = size;
        }
        MistSmokeSurfaceRole::BackdropVeil => {
            let (intensity, pulse, density) = match state {
                MistSurfaceState::Idle => (1.85, 0.08, 0.44),
                MistSurfaceState::Active => (2.20, 0.11, 0.60),
                MistSurfaceState::Pressed => (2.55, 0.15, 0.78),
            };
            config.thickness = 0.16;
            config.intensity = intensity;
            config.flow_speed = 0.54;
            config.noise_scale = 42.0;
            config.softness = 0.66;
            config.pulse_strength = pulse;
            config.particle_density = density;
            config.particle_size_scale = 1.56;
        }
    }

    MistSmokeSurface::new(theme.apply_surface_config(
        config,
        match role {
            MistSmokeSurfaceRole::AccentChip | MistSmokeSurfaceRole::FeedbackBody => 1.08,
            MistSmokeSurfaceRole::AccentOrb => 1.06,
            MistSmokeSurfaceRole::BackdropVeil => 1.04,
            _ => 1.0,
        },
        matches!(
            role,
            MistSmokeSurfaceRole::AccentChip | MistSmokeSurfaceRole::FeedbackBody
        ),
        matches!(
            role,
            MistSmokeSurfaceRole::AccentOrb | MistSmokeSurfaceRole::ScalarFill
        ),
        matches!(role, MistSmokeSurfaceRole::BackdropVeil),
    ))
    .with_inset(inset.x, inset.y)
}

fn surface_smoke_for_role(role: MistSmokeSurfaceRole, active: bool) -> MistSmokeSurface {
    surface_smoke_for_role_state(
        role,
        if active {
            MistSurfaceState::Active
        } else {
            MistSurfaceState::Idle
        },
    )
}

fn surface_smoke_for_widget_state(
    role: MistSmokeRole,
    state: MistSurfaceState,
) -> MistSmokeSurface {
    let surface_role = match role {
        MistSmokeRole::PanelFrame | MistSmokeRole::ScrollContainer | MistSmokeRole::DialogFrame => {
            MistSmokeSurfaceRole::ContainerBody
        }
        MistSmokeRole::MenuFrame => MistSmokeSurfaceRole::MenuBody,
        MistSmokeRole::DataFrame | MistSmokeRole::DataItem => MistSmokeSurfaceRole::DataBody,
        MistSmokeRole::FeedbackChip => MistSmokeSurfaceRole::FeedbackBody,
        MistSmokeRole::DropdownOption => MistSmokeSurfaceRole::OptionBody,
        MistSmokeRole::ScalarControl => MistSmokeSurfaceRole::ScalarTrack,
        MistSmokeRole::StandardButton | MistSmokeRole::ToolbarButton => {
            MistSmokeSurfaceRole::ControlBody
        }
        MistSmokeRole::TriggerButton => MistSmokeSurfaceRole::AccentChip,
    };
    surface_smoke_for_role_state(surface_role, state)
}

fn surface_smoke_for_widget_role(role: MistSmokeRole, active: bool) -> MistSmokeSurface {
    surface_smoke_for_widget_state(
        role,
        if active {
            MistSurfaceState::Active
        } else {
            MistSurfaceState::Idle
        },
    )
}

fn text_line(font: &Handle<Font>, value: &str, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(value),
        Node::default(),
        TextFont {
            font: font.clone(),
            font_size: size,
            ..default()
        },
        TextColor(color),
        TextLayout::new_with_justify(Justify::Left).with_no_wrap(),
    )
}

fn text_block(font: &Handle<Font>, value: &str, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(value),
        Node::default(),
        TextFont {
            font: font.clone(),
            font_size: size,
            ..default()
        },
        TextColor(color),
        TextLayout::new_with_justify(Justify::Left),
    )
}

#[derive(Component, Clone, Copy, Debug)]
pub struct MistInteractiveStyle {
    pub idle_fill: Color,
    pub hover_fill: Color,
    pub pressed_fill: Color,
    pub idle_border: Color,
    pub hover_border: Color,
    pub pressed_border: Color,
}

impl Default for MistInteractiveStyle {
    fn default() -> Self {
        Self {
            idle_fill: control_fill(),
            hover_fill: hovered_fill(),
            pressed_fill: pressed_fill(),
            idle_border: Color::NONE,
            hover_border: Color::NONE,
            pressed_border: Color::NONE,
        }
    }
}

fn trigger_interactive_style() -> MistInteractiveStyle {
    MistInteractiveStyle {
        idle_fill: Color::srgba(0.01, 0.03, 0.06, 0.05),
        hover_fill: Color::srgba(0.02, 0.05, 0.09, 0.08),
        pressed_fill: Color::srgba(0.03, 0.07, 0.12, 0.12),
        idle_border: Color::NONE,
        hover_border: Color::NONE,
        pressed_border: Color::NONE,
    }
}

fn data_item_interactive_style() -> MistInteractiveStyle {
    MistInteractiveStyle {
        idle_fill: Color::srgba(0.01, 0.03, 0.05, 0.05),
        hover_fill: Color::srgba(0.02, 0.05, 0.08, 0.08),
        pressed_fill: Color::srgba(0.03, 0.07, 0.11, 0.11),
        idle_border: Color::NONE,
        hover_border: Color::NONE,
        pressed_border: Color::NONE,
    }
}

fn feedback_interactive_style() -> MistInteractiveStyle {
    MistInteractiveStyle {
        idle_fill: Color::srgba(0.01, 0.04, 0.07, 0.10),
        hover_fill: Color::srgba(0.02, 0.06, 0.10, 0.14),
        pressed_fill: Color::srgba(0.03, 0.08, 0.13, 0.18),
        idle_border: Color::NONE,
        hover_border: Color::NONE,
        pressed_border: Color::NONE,
    }
}

#[derive(Component)]
struct MistSmokeHostReady;

#[derive(Component)]
pub struct MistPanel;

#[derive(Component)]
pub struct MistLabel;

#[derive(Component)]
pub struct MistImage;

#[derive(Component)]
pub struct MistButton;

#[derive(Component)]
pub struct MistTrigger;

#[derive(Component)]
pub struct MistCheckbox;

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct MistCheckboxState {
    pub checked: bool,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct MistCheckboxParts {
    indicator: Entity,
    glyph: Entity,
    tag: Entity,
    tag_text: Entity,
}

#[derive(Component, Clone, Debug)]
pub struct MistRadioGroup {
    pub selected: usize,
}

#[derive(Component, Clone, Debug)]
pub struct MistRadioOptions(pub Vec<String>);

#[derive(Component, Clone, Copy, Debug)]
pub struct MistRadioOwner(pub Entity);

#[derive(Component, Clone, Copy, Debug)]
pub struct MistRadioOption {
    pub index: usize,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct MistRadioParts {
    indicator: Entity,
    glyph: Entity,
    label: Entity,
}

#[derive(Component)]
pub struct MistSwitch;

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct MistSwitchState {
    pub on: bool,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct MistSwitchParts {
    track: Entity,
    knob: Entity,
}

#[derive(Component)]
pub struct MistScrollView;

#[derive(Component)]
pub struct MistScrollViewport;

#[derive(Component)]
pub struct MistScrollContent;

#[derive(Component)]
pub struct MistScrollTrack;

#[derive(Component)]
pub struct MistScrollThumb;

#[derive(Component, Clone, Copy, Debug)]
pub struct MistScrollParts {
    pub viewport: Entity,
    pub content: Entity,
    pub track: Entity,
    pub thumb: Entity,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct MistSlider {
    pub min: f32,
    pub max: f32,
}

impl MistSlider {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    fn normalize(&self, value: f32) -> f32 {
        let span = (self.max - self.min).max(f32::EPSILON);
        ((value - self.min) / span).clamp(0.0, 1.0)
    }

    fn denormalize(&self, t: f32) -> f32 {
        self.min + (self.max - self.min) * t.clamp(0.0, 1.0)
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct MistSliderValue(pub f32);

#[derive(Component, Clone, Copy, Debug)]
pub struct MistSliderParts {
    track: Entity,
    fill: Entity,
    handle: Entity,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct MistProgressBar {
    pub displayed: f32,
    pub target: f32,
}

impl Default for MistProgressBar {
    fn default() -> Self {
        Self {
            displayed: 0.0,
            target: 0.0,
        }
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct MistProgressParts {
    fill: Entity,
}

#[derive(Component, Clone, Debug)]
pub struct MistInputField {
    pub value: String,
    pub placeholder: String,
    pub max_chars: Option<usize>,
}

impl MistInputField {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            value: String::new(),
            placeholder: placeholder.into(),
            max_chars: None,
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn with_max_chars(mut self, max_chars: usize) -> Self {
        self.max_chars = Some(max_chars);
        self
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct MistInputParts {
    value_text: Entity,
    placeholder_text: Entity,
    caret: Entity,
}

#[derive(Component)]
pub struct MistInputFocused;

#[derive(Resource, Default)]
struct MistInputFocusState {
    active: Option<Entity>,
}

#[derive(Component)]
pub struct MistTooltipAnchor;

#[derive(Component, Clone, Copy, Debug)]
pub struct MistTooltipOwner(pub Entity);

#[derive(Component, Clone, Debug)]
pub struct MistTooltip {
    pub enabled: bool,
}

impl Default for MistTooltip {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct MistDropdown {
    pub open: bool,
    pub selected: usize,
}

#[derive(Component, Clone, Debug)]
pub struct MistDropdownOptions(pub Vec<String>);

#[derive(Component, Clone, Copy, Debug)]
pub struct MistDropdownParts {
    label_text: Entity,
    menu: Entity,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct MistDropdownOwner(pub Entity);

#[derive(Component)]
pub struct MistDropdownTrigger;

#[derive(Component)]
pub struct MistDropdownItem {
    pub index: usize,
}

#[derive(Component, Clone, Debug)]
pub struct MistTabs {
    pub selected: usize,
}

#[derive(Component, Clone, Debug)]
pub struct MistTabLabels(pub Vec<String>);

#[derive(Component, Clone, Copy, Debug)]
pub struct MistTabOwner(pub Entity);

#[derive(Component, Clone, Copy, Debug)]
pub struct MistTabButton {
    pub index: usize,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct MistTabParts {
    label: Entity,
}

#[derive(Component, Clone, Debug)]
pub struct MistDialog {
    pub open: bool,
    pub dismiss_on_backdrop: bool,
}

impl Default for MistDialog {
    fn default() -> Self {
        Self {
            open: false,
            dismiss_on_backdrop: true,
        }
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct MistDialogParts {
    pub backdrop: Entity,
    pub panel: Entity,
    pub close_button: Entity,
    pub title: Entity,
    pub body: Entity,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct MistDialogOwner(pub Entity);

#[derive(Component)]
pub struct MistDialogBackdrop;

#[derive(Component)]
pub struct MistDialogCloseButton;

#[derive(Component)]
pub struct MistBadge;

#[derive(Component)]
pub struct MistChip;

#[derive(Component)]
pub struct MistStatusPill;

#[derive(Component, Clone, Debug, Default)]
pub struct MistToast {
    pub open: bool,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct MistToastParts {
    pub close_button: Entity,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct MistToastOwner(pub Entity);

#[derive(Component)]
pub struct MistToastCloseButton;

#[derive(Component, Clone, Debug, Default)]
pub struct MistPopover {
    pub open: bool,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct MistPopoverParts {
    pub panel: Entity,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct MistPopoverOwner(pub Entity);

#[derive(Component)]
pub struct MistPopoverAnchor;

#[derive(Component, Clone, Debug, Default)]
pub struct MistContextMenu {
    pub open: bool,
}

#[derive(Component, Clone, Debug)]
pub struct MistContextMenuOptions(pub Vec<String>);

#[derive(Component, Clone, Copy, Debug)]
pub struct MistContextMenuParts {
    pub menu: Entity,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct MistContextMenuOwner(pub Entity);

#[derive(Component, Clone, Copy, Debug)]
pub struct MistContextMenuItem {
    pub index: usize,
}

#[derive(Component)]
pub struct MistMenuList;

#[derive(Component, Clone, Debug)]
pub struct MistMenuListOptions(pub Vec<String>);

#[derive(Component, Clone, Copy, Debug)]
pub struct MistMenuListOwner(pub Entity);

#[derive(Component, Clone, Copy, Debug)]
pub struct MistMenuListItem {
    pub index: usize,
}

#[derive(Component)]
pub struct MistAccordion;

#[derive(Component, Clone, Debug)]
pub struct MistAccordionSections(pub Vec<String>);

#[derive(Component, Clone, Copy, Debug)]
pub struct MistAccordionOwner(pub Entity);

#[derive(Component, Clone, Copy, Debug)]
pub struct MistAccordionSection {
    pub index: usize,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct MistAccordionParts {
    pub body: Entity,
    pub chevron: Entity,
}

#[derive(Component, Clone, Debug)]
pub struct MistAccordionState(pub Vec<bool>);

#[derive(Component)]
pub struct MistSegmentedActionRow;

#[derive(Component, Clone, Debug)]
pub struct MistSegmentedActionLabels(pub Vec<String>);

#[derive(Component, Clone, Copy, Debug)]
pub struct MistSegmentedActionOwner(pub Entity);

#[derive(Component, Clone, Copy, Debug)]
pub struct MistSegmentedActionButton {
    pub index: usize,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct MistSegmentedActionParts {
    pub label: Entity,
}

#[derive(Component, Clone, Debug)]
pub struct MistListView {
    pub selected: Option<usize>,
}

#[derive(Component, Clone, Debug)]
pub struct MistListItems(pub Vec<String>);

#[derive(Component, Clone, Copy, Debug)]
pub struct MistListOwner(pub Entity);

#[derive(Component, Clone, Copy, Debug)]
pub struct MistListItem {
    pub index: usize,
}

#[derive(Component, Clone, Debug)]
pub struct MistTable {
    pub selected: Option<usize>,
}

#[derive(Component, Clone, Debug)]
pub struct MistTableColumns(pub Vec<String>);

#[derive(Component, Clone, Debug)]
pub struct MistTableRows(pub Vec<Vec<String>>);

#[derive(Component, Clone, Copy, Debug)]
pub struct MistTableOwner(pub Entity);

#[derive(Component, Clone, Copy, Debug)]
pub struct MistTableHeaderButton {
    pub index: usize,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct MistTableRowButton {
    pub index: usize,
}

#[derive(Clone, Debug)]
pub struct MistTreeNodeSpec {
    pub label: String,
    pub parent: Option<usize>,
    pub expanded: bool,
}

impl MistTreeNodeSpec {
    pub fn root(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            parent: None,
            expanded: true,
        }
    }

    pub fn child(label: impl Into<String>, parent: usize) -> Self {
        Self {
            label: label.into(),
            parent: Some(parent),
            expanded: false,
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct MistTreeView {
    pub selected: Option<usize>,
}

#[derive(Component, Clone, Debug)]
pub struct MistTreeNodes(pub Vec<MistTreeNodeSpec>);

#[derive(Component, Clone, Copy, Debug)]
pub struct MistTreeOwner(pub Entity);

#[derive(Component, Clone, Copy, Debug)]
pub struct MistTreeItem {
    pub index: usize,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct MistTreeItemParts {
    pub toggle: Entity,
    pub label: Entity,
}

#[derive(Component, Clone, Debug)]
pub struct MistGridView {
    pub selected: Option<usize>,
}

#[derive(Component, Clone, Debug)]
pub struct MistGridItems(pub Vec<String>);

#[derive(Component, Clone, Copy, Debug)]
pub struct MistGridOwner(pub Entity);

#[derive(Component, Clone, Copy, Debug)]
pub struct MistGridItem {
    pub index: usize,
}

#[derive(Event, Message, Clone, Debug)]
pub struct MistButtonPressed {
    pub entity: Entity,
}

#[derive(Event, Message, Clone, Debug)]
pub struct MistTriggerPressed {
    pub entity: Entity,
}

#[derive(Event, Message, Clone, Debug)]
pub struct MistCheckboxChanged {
    pub entity: Entity,
    pub checked: bool,
}

#[derive(Event, Message, Clone, Debug)]
pub struct MistRadioChanged {
    pub entity: Entity,
    pub selected: usize,
    pub label: String,
}

#[derive(Event, Message, Clone, Debug)]
pub struct MistSwitchChanged {
    pub entity: Entity,
    pub on: bool,
}

#[derive(Event, Message, Clone, Debug)]
pub struct MistSliderChanged {
    pub entity: Entity,
    pub value: f32,
}

#[derive(Event, Message, Clone, Debug)]
pub struct MistInputChanged {
    pub entity: Entity,
    pub value: String,
}

#[derive(Event, Message, Clone, Debug)]
pub struct MistInputSubmitted {
    pub entity: Entity,
    pub value: String,
}

#[derive(Event, Message, Clone, Debug)]
pub struct MistDropdownChanged {
    pub entity: Entity,
    pub selected: usize,
    pub label: String,
}

#[derive(Event, Message, Clone, Debug)]
pub struct MistTabsChanged {
    pub entity: Entity,
    pub selected: usize,
    pub label: String,
}

#[derive(Event, Message, Clone, Debug)]
pub struct MistDialogDismissed {
    pub entity: Entity,
}

#[derive(Event, Message, Clone, Debug)]
pub struct MistMenuAction {
    pub entity: Entity,
    pub index: usize,
    pub label: String,
}

#[derive(Event, Message, Clone, Debug)]
pub struct MistAccordionChanged {
    pub entity: Entity,
    pub section: usize,
    pub open: bool,
    pub label: String,
}

#[derive(Event, Message, Clone, Debug)]
pub struct MistSegmentedActionInvoked {
    pub entity: Entity,
    pub selected: usize,
    pub label: String,
}

#[derive(Event, Message, Clone, Debug)]
pub struct MistListSelectionChanged {
    pub entity: Entity,
    pub selected: usize,
    pub label: String,
}

#[derive(Event, Message, Clone, Debug)]
pub struct MistTableRowSelected {
    pub entity: Entity,
    pub row: usize,
}

#[derive(Event, Message, Clone, Debug)]
pub struct MistTableSortRequested {
    pub entity: Entity,
    pub column: usize,
    pub label: String,
}

#[derive(Event, Message, Clone, Debug)]
pub struct MistTreeNodeSelected {
    pub entity: Entity,
    pub node: usize,
    pub label: String,
}

#[derive(Event, Message, Clone, Debug)]
pub struct MistTreeNodeToggled {
    pub entity: Entity,
    pub node: usize,
    pub expanded: bool,
    pub label: String,
}

#[derive(Event, Message, Clone, Debug)]
pub struct MistGridItemSelected {
    pub entity: Entity,
    pub selected: usize,
    pub label: String,
}

#[derive(Event, Message, Clone, Debug)]
pub struct MistToastDismissed {
    pub entity: Entity,
}

pub struct MistUiMessagesPlugin;

impl Plugin for MistUiMessagesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MistInputFocusState>()
            .init_resource::<ButtonInput<MouseButton>>()
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<Messages<MouseWheel>>()
            .init_resource::<Messages<KeyboardInput>>()
            .add_message::<MistButtonPressed>()
            .add_message::<MistTriggerPressed>()
            .add_message::<MistCheckboxChanged>()
            .add_message::<MistRadioChanged>()
            .add_message::<MistSwitchChanged>()
            .add_message::<MistSliderChanged>()
            .add_message::<MistInputChanged>()
            .add_message::<MistInputSubmitted>()
            .add_message::<MistDropdownChanged>()
            .add_message::<MistTabsChanged>()
            .add_message::<MistDialogDismissed>()
            .add_message::<MistMenuAction>()
            .add_message::<MistAccordionChanged>()
            .add_message::<MistSegmentedActionInvoked>()
            .add_message::<MistListSelectionChanged>()
            .add_message::<MistTableRowSelected>()
            .add_message::<MistTableSortRequested>()
            .add_message::<MistTreeNodeSelected>()
            .add_message::<MistTreeNodeToggled>()
            .add_message::<MistGridItemSelected>()
            .add_message::<MistToastDismissed>();
    }
}

pub struct MistUiSmokeHostPlugin;

impl Plugin for MistUiSmokeHostPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                ensure_smoke_runtime_components,
                relax_smoke_host_clipping,
                sync_interactive_styles,
                sync_interactive_smoke_borders,
            ),
        );
    }
}

pub struct MistUiActionPlugin;

impl Plugin for MistUiActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (emit_button_pressed, emit_trigger_pressed));
    }
}

pub struct MistUiSelectionPlugin;

impl Plugin for MistUiSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (toggle_checkboxes, sync_checkbox_visuals).chain(),
                (select_radio_items, sync_radio_visuals).chain(),
                (toggle_switches, sync_switch_visuals).chain(),
                (
                    toggle_dropdowns,
                    select_dropdown_items,
                    close_dropdowns_on_outside_click,
                    sync_dropdowns,
                )
                    .chain(),
                (select_tabs, sync_tabs).chain(),
            ),
        );
    }
}

pub struct MistUiScalarPlugin;

impl Plugin for MistUiScalarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (scroll_mist_views, sync_scrollbar_visuals).chain(),
                (drive_sliders, sync_slider_visuals).chain(),
                animate_progress_bars,
            ),
        );
    }
}

pub struct MistUiInputPlugin;

impl Plugin for MistUiInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    focus_input_fields,
                    type_into_input_fields,
                    sync_input_fields,
                )
                    .chain(),
                sync_tooltips,
            ),
        );
    }
}

pub struct MistUiOverlayPlugin;

impl Plugin for MistUiOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (dismiss_dialogs, dismiss_dialogs_on_escape, sync_dialogs).chain(),
        );
    }
}

pub struct MistUiFeedbackPlugin;

impl Plugin for MistUiFeedbackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (dismiss_toasts, sync_toasts, sync_popovers).chain());
    }
}

pub struct MistUiDataViewPlugin;

impl Plugin for MistUiDataViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    invoke_menu_lists,
                    invoke_context_menu_items,
                    sync_context_menus,
                )
                    .chain(),
                (toggle_accordion_sections, sync_accordion_sections).chain(),
                invoke_segmented_actions,
                select_list_items,
                (select_table_rows, request_table_sort, sync_table_rows).chain(),
                (select_tree_rows, sync_tree_rows).chain(),
                select_grid_items,
            ),
        );
    }
}

pub struct MistUiPlugin;

impl Plugin for MistUiPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<MistUiMessagesPlugin>() {
            app.add_plugins(MistUiMessagesPlugin);
        }
        if !app.is_plugin_added::<MistUiSmokeHostPlugin>() {
            app.add_plugins(MistUiSmokeHostPlugin);
        }
        if !app.is_plugin_added::<MistUiActionPlugin>() {
            app.add_plugins(MistUiActionPlugin);
        }
        if !app.is_plugin_added::<MistUiSelectionPlugin>() {
            app.add_plugins(MistUiSelectionPlugin);
        }
        if !app.is_plugin_added::<MistUiScalarPlugin>() {
            app.add_plugins(MistUiScalarPlugin);
        }
        if !app.is_plugin_added::<MistUiInputPlugin>() {
            app.add_plugins(MistUiInputPlugin);
        }
        if !app.is_plugin_added::<MistUiOverlayPlugin>() {
            app.add_plugins(MistUiOverlayPlugin);
        }
        if !app.is_plugin_added::<MistUiFeedbackPlugin>() {
            app.add_plugins(MistUiFeedbackPlugin);
        }
        if !app.is_plugin_added::<MistUiDataViewPlugin>() {
            app.add_plugins(MistUiDataViewPlugin);
        }
    }
}

fn ensure_smoke_runtime_components(
    mut commands: Commands,
    query: Query<
        (
            Entity,
            &MistSmokeRole,
            Option<&MistSmokeConfig>,
            Option<&MistSmokeTarget>,
            Option<&SmokeBorder>,
        ),
        With<MistSmokeRole>,
    >,
) {
    for (entity, role, config, target, border) in &query {
        let default_config = smoke_config_for_role(*role, false);
        let mut entity_commands = commands.entity(entity);
        if config.is_none() {
            entity_commands.insert(default_config);
        }
        if target.is_none() {
            entity_commands.insert(MistSmokeTarget::screen_ui());
        }
        if border.is_none() {
            entity_commands.insert(derived_screen_ring(default_config));
        }
    }
}

fn relax_smoke_host_clipping(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Node),
        (
            Or<(With<MistSmokeRole>, With<MistSmokeSurface>)>,
            Without<MistSmokeParticle>,
            Without<MistSmokeHostReady>,
        ),
    >,
) {
    for (entity, mut node) in &mut query {
        node.overflow = Overflow::visible();
        commands.entity(entity).insert(MistSmokeHostReady);
    }
}

pub fn mist_panel() -> impl Bundle {
    (
        MistPanel,
        MistSmokeRole::PanelFrame,
        surface_smoke_for_widget_role(MistSmokeRole::PanelFrame, false),
        smoke_border_for_role(MistSmokeRole::PanelFrame, false, 21),
        smoke_padding_for_role(MistSmokeRole::PanelFrame),
        Node {
            padding: UiRect::all(Val::Px(16.0)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            border: UiRect::all(Val::Px(0.0)),
            border_radius: BorderRadius::all(Val::Px(18.0)),
            ..default()
        },
        BackgroundColor(panel_fill()),
        BorderColor::all(Color::NONE),
    )
}

pub fn mist_label(font: &Handle<Font>, value: &str, size: f32) -> impl Bundle {
    (MistLabel, text_block(font, value, size, text_primary()))
}

pub fn mist_image(image: Handle<Image>, size: Vec2) -> impl Bundle {
    (
        MistImage,
        ImageNode::new(image),
        MistSmokeRole::PanelFrame,
        surface_smoke_for_widget_role(MistSmokeRole::PanelFrame, false),
        smoke_border_for_role(MistSmokeRole::PanelFrame, false, 31),
        smoke_padding_for_role(MistSmokeRole::PanelFrame),
        Node {
            width: Val::Px(size.x),
            height: Val::Px(size.y),
            ..default()
        },
    )
}

pub fn spawn_mist_panel(commands: &mut Commands) -> Entity {
    commands.spawn(mist_panel()).id()
}

pub fn spawn_mist_label(
    commands: &mut Commands,
    font: &Handle<Font>,
    value: impl AsRef<str>,
    size: f32,
) -> Entity {
    commands.spawn(mist_label(font, value.as_ref(), size)).id()
}

pub fn spawn_mist_image(commands: &mut Commands, image: Handle<Image>, size: Vec2) -> Entity {
    commands.spawn(mist_image(image, size)).id()
}

pub fn spawn_mist_button(
    commands: &mut Commands,
    font: &Handle<Font>,
    label: impl Into<String>,
    width: f32,
) -> Entity {
    let label = label.into();
    commands
        .spawn((
            MistButton,
            Button,
            MistSmokeRole::StandardButton,
            surface_smoke_for_widget_role(MistSmokeRole::StandardButton, false),
            MistInteractiveStyle::default(),
            smoke_border_for_role(MistSmokeRole::StandardButton, false, 1),
            smoke_padding_for_role(MistSmokeRole::StandardButton),
            Node {
                width: Val::Px(width),
                min_height: Val::Px(CONTROL_HEIGHT),
                padding: UiRect::axes(Val::Px(CONTROL_PADDING_X), Val::Px(CONTROL_PADDING_Y)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(0.0)),
                border_radius: BorderRadius::all(Val::Px(CONTROL_RADIUS)),
                ..default()
            },
            BackgroundColor(control_fill()),
            BorderColor::all(Color::NONE),
            children![text_line(font, &label, 20.0, text_primary())],
        ))
        .id()
}

pub fn spawn_mist_trigger(
    commands: &mut Commands,
    font: &Handle<Font>,
    label: impl Into<String>,
    width: f32,
) -> Entity {
    let label = label.into();
    let root = commands
        .spawn((
            MistTrigger,
            Button,
            MistSmokeRole::TriggerButton,
            surface_smoke_for_widget_role(MistSmokeRole::TriggerButton, false),
            trigger_interactive_style(),
            smoke_border_for_role(MistSmokeRole::TriggerButton, false, 2),
            smoke_padding_for_role(MistSmokeRole::TriggerButton),
            Node {
                width: Val::Px(width),
                min_height: Val::Px(CONTROL_HEIGHT + 6.0),
                padding: UiRect::axes(Val::Px(12.0), Val::Px(CONTROL_PADDING_Y)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                column_gap: Val::Px(12.0),
                border: UiRect::all(Val::Px(0.0)),
                border_radius: BorderRadius::all(Val::Px(CONTROL_RADIUS + 4.0)),
                ..default()
            },
            BackgroundColor(control_fill()),
            BorderColor::all(Color::NONE),
        ))
        .id();

    let tag = commands
        .spawn((
            Node {
                min_width: Val::Px(54.0),
                min_height: Val::Px(28.0),
                padding: UiRect::axes(Val::Px(10.0), Val::Px(6.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..default()
            },
            surface_smoke_for_role(MistSmokeSurfaceRole::AccentChip, true),
            BackgroundColor(Color::NONE),
            BorderColor::all(Color::NONE),
            children![text_line(font, "TRG", 13.0, text_secondary())],
        ))
        .id();
    let label_node = commands
        .spawn((
            Node {
                flex_grow: 1.0,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            children![text_line(font, &label, 20.0, text_primary())],
        ))
        .id();
    let chevron = commands
        .spawn((
            Node {
                min_width: Val::Px(32.0),
                min_height: Val::Px(28.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..default()
            },
            surface_smoke_for_role(MistSmokeSurfaceRole::AccentOrb, true),
            BackgroundColor(Color::NONE),
            BorderColor::all(Color::NONE),
            children![text_line(font, ">>", 16.0, text_primary())],
        ))
        .id();

    commands
        .entity(root)
        .add_children(&[tag, label_node, chevron]);
    root
}

pub fn spawn_mist_checkbox(
    commands: &mut Commands,
    font: &Handle<Font>,
    label: impl Into<String>,
    checked: bool,
) -> Entity {
    let label = label.into();
    let indicator_smoke = checkbox_indicator_smoke_config(checked);
    let root = commands
        .spawn((
            MistCheckbox,
            MistCheckboxState { checked },
            Button,
            MistSmokeRole::StandardButton,
            surface_smoke_for_widget_role(MistSmokeRole::StandardButton, false),
            MistInteractiveStyle::default(),
            smoke_border_for_role(MistSmokeRole::StandardButton, false, 3),
            smoke_padding_for_role(MistSmokeRole::StandardButton),
            Node {
                min_width: Val::Px(220.0),
                min_height: Val::Px(CONTROL_HEIGHT),
                padding: UiRect::axes(Val::Px(CONTROL_PADDING_X), Val::Px(CONTROL_PADDING_Y)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(0.0)),
                border_radius: BorderRadius::all(Val::Px(CONTROL_RADIUS)),
                ..default()
            },
            BackgroundColor(control_fill()),
            BorderColor::all(Color::NONE),
        ))
        .id();
    let content = commands
        .spawn((Node {
            flex_grow: 1.0,
            column_gap: Val::Px(12.0),
            align_items: AlignItems::Center,
            ..default()
        },))
        .id();

    let indicator = commands
        .spawn((
            MistSmokeRole::DropdownOption,
            indicator_smoke,
            derived_screen_ring(indicator_smoke),
            SmokeRingPadding::all(2.0),
            Node {
                width: Val::Px(26.0),
                height: Val::Px(26.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(0.0)),
                border_radius: BorderRadius::all(Val::Px(4.0)),
                ..default()
            },
            surface_smoke_for_role(MistSmokeSurfaceRole::AccentChip, checked),
            BackgroundColor(Color::NONE),
            BorderColor::all(Color::NONE),
        ))
        .id();
    let glyph = commands
        .spawn(text_line(
            font,
            if checked { "✓" } else { "" },
            17.0,
            Color::srgba(0.03, 0.06, 0.10, 1.0),
        ))
        .id();

    let label_text = commands
        .spawn(text_line(font, &label, 18.0, text_primary()))
        .id();
    let tag = commands
        .spawn((
            Node {
                min_width: Val::Px(50.0),
                min_height: Val::Px(26.0),
                padding: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(9.0)),
                ..default()
            },
            surface_smoke_for_role(MistSmokeSurfaceRole::AccentChip, checked),
            BackgroundColor(Color::NONE),
            BorderColor::all(Color::NONE),
        ))
        .id();
    let tag_text = commands
        .spawn(text_line(
            font,
            if checked { "ON" } else { "OFF" },
            13.0,
            text_secondary(),
        ))
        .id();

    commands.entity(indicator).add_child(glyph);
    commands.entity(tag).add_child(tag_text);
    commands
        .entity(content)
        .add_children(&[indicator, label_text]);
    commands.entity(root).add_children(&[content, tag]);
    commands.entity(root).insert(MistCheckboxParts {
        indicator,
        glyph,
        tag,
        tag_text,
    });
    root
}

pub fn spawn_mist_radio_group(
    commands: &mut Commands,
    font: &Handle<Font>,
    width: f32,
    options: impl IntoIterator<Item = impl Into<String>>,
    selected: usize,
) -> Entity {
    let options: Vec<String> = options.into_iter().map(Into::into).collect();
    let selected = clamp_choice_index(options.len(), selected);
    let root = commands
        .spawn((
            MistRadioGroup { selected },
            MistRadioOptions(options.clone()),
            MistSmokeRole::PanelFrame,
            surface_smoke_for_widget_role(MistSmokeRole::PanelFrame, false),
            smoke_border_for_role(MistSmokeRole::PanelFrame, false, 9),
            smoke_padding_for_role(MistSmokeRole::PanelFrame),
            Node {
                width: Val::Px(width),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                border: UiRect::all(Val::Px(0.0)),
                border_radius: BorderRadius::all(Val::Px(18.0)),
                ..default()
            },
            BackgroundColor(control_fill()),
            BorderColor::all(Color::NONE),
        ))
        .id();

    for (index, label) in options.iter().enumerate() {
        let item = commands
            .spawn((
                Button,
                MistRadioOwner(root),
                MistRadioOption { index },
                MistSmokeRole::DropdownOption,
                surface_smoke_for_widget_role(MistSmokeRole::DropdownOption, false),
                smoke_border_for_role(
                    MistSmokeRole::DropdownOption,
                    false,
                    (root.to_bits() ^ index as u64) + 90,
                ),
                smoke_padding_for_role(MistSmokeRole::DropdownOption),
                Node {
                    width: Val::Percent(100.0),
                    min_height: Val::Px(38.0),
                    padding: UiRect::axes(Val::Px(10.0), Val::Px(6.0)),
                    column_gap: Val::Px(10.0),
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(0.0)),
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                BorderColor::all(Color::NONE),
            ))
            .id();

        let indicator = commands
            .spawn((
                Node {
                    width: Val::Px(20.0),
                    height: Val::Px(20.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(0.0)),
                    border_radius: BorderRadius::all(Val::Px(10.0)),
                    ..default()
                },
                surface_smoke_for_role(MistSmokeSurfaceRole::AccentChip, false),
                BackgroundColor(Color::NONE),
                BorderColor::all(Color::NONE),
            ))
            .id();
        let glyph = commands
            .spawn(text_line(
                font,
                "",
                18.0,
                Color::srgba(0.03, 0.06, 0.10, 1.0),
            ))
            .id();
        let label_text = commands
            .spawn(text_line(font, label, 18.0, text_secondary()))
            .id();
        commands.entity(label_text).insert(Node {
            flex_grow: 1.0,
            ..default()
        });

        commands.entity(indicator).add_child(glyph);
        commands.entity(item).add_children(&[indicator, label_text]);
        commands.entity(item).insert(MistRadioParts {
            indicator,
            glyph,
            label: label_text,
        });
        commands.entity(root).add_child(item);
    }

    root
}

pub fn spawn_mist_switch(
    commands: &mut Commands,
    font: &Handle<Font>,
    label: impl Into<String>,
    on: bool,
) -> Entity {
    let label = label.into();
    let root = commands
        .spawn((
            MistSwitch,
            MistSwitchState { on },
            Button,
            MistSmokeRole::StandardButton,
            surface_smoke_for_widget_role(MistSmokeRole::StandardButton, false),
            smoke_border_for_role(MistSmokeRole::StandardButton, false, 10),
            smoke_padding_for_role(MistSmokeRole::StandardButton),
            Node {
                min_width: Val::Px(220.0),
                min_height: Val::Px(CONTROL_HEIGHT),
                padding: UiRect::axes(Val::Px(CONTROL_PADDING_X), Val::Px(CONTROL_PADDING_Y)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(0.0)),
                border_radius: BorderRadius::all(Val::Px(CONTROL_RADIUS)),
                ..default()
            },
            BackgroundColor(control_fill()),
            BorderColor::all(Color::NONE),
        ))
        .id();

    let label_text = commands
        .spawn(text_line(font, &label, 18.0, text_primary()))
        .id();
    let track = commands
        .spawn((
            Node {
                width: Val::Px(SWITCH_TRACK_WIDTH),
                height: Val::Px(SWITCH_TRACK_HEIGHT),
                position_type: PositionType::Relative,
                border: UiRect::all(Val::Px(0.0)),
                border_radius: BorderRadius::all(Val::Px(SWITCH_TRACK_HEIGHT * 0.5)),
                ..default()
            },
            surface_smoke_for_role(MistSmokeSurfaceRole::ScalarTrack, on),
            BackgroundColor(Color::NONE),
            BorderColor::all(Color::NONE),
        ))
        .id();
    let knob = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(2.0),
                top: Val::Px(2.0),
                width: Val::Px(SWITCH_KNOB_SIZE),
                height: Val::Px(SWITCH_KNOB_SIZE),
                border_radius: BorderRadius::all(Val::Px(SWITCH_KNOB_SIZE * 0.5)),
                ..default()
            },
            surface_smoke_for_role(MistSmokeSurfaceRole::AccentOrb, on),
            BackgroundColor(Color::NONE),
        ))
        .id();

    commands.entity(track).add_child(knob);
    commands.entity(root).add_children(&[label_text, track]);
    commands
        .entity(root)
        .insert(MistSwitchParts { track, knob });
    root
}

pub fn spawn_mist_scroll_view(
    commands: &mut Commands,
    width: f32,
    height: f32,
) -> (Entity, Entity) {
    let root = commands
        .spawn((
            MistScrollView,
            MistSmokeRole::ScrollContainer,
            surface_smoke_for_widget_role(MistSmokeRole::ScrollContainer, false),
            smoke_border_for_role(MistSmokeRole::ScrollContainer, false, 12),
            smoke_padding_for_role(MistSmokeRole::ScrollContainer),
            Node {
                width: Val::Px(width),
                height: Val::Px(height),
                padding: UiRect::all(Val::Px(8.0)),
                position_type: PositionType::Relative,
                border: UiRect::all(Val::Px(0.0)),
                border_radius: BorderRadius::all(Val::Px(18.0)),
                ..default()
            },
            BackgroundColor(panel_fill()),
            BorderColor::all(Color::NONE),
        ))
        .id();

    let viewport = commands
        .spawn((
            MistScrollViewport,
            RelativeCursorPosition::default(),
            ScrollPosition::default(),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                overflow: Overflow::scroll_y(),
                padding: UiRect::right(Val::Px(18.0)),
                ..default()
            },
        ))
        .id();

    let content = commands
        .spawn((
            MistScrollContent,
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                ..default()
            },
        ))
        .id();

    let track = commands
        .spawn((
            MistScrollTrack,
            MistSmokeRole::ScalarControl,
            surface_smoke_for_role(MistSmokeSurfaceRole::ScalarTrack, true),
            smoke_border_for_role(MistSmokeRole::ScalarControl, false, 412),
            SmokeRingPadding::all(2.0),
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(10.0),
                top: Val::Px(10.0),
                bottom: Val::Px(10.0),
                width: Val::Px(SCROLLBAR_WIDTH),
                border_radius: BorderRadius::all(Val::Px(SCROLLBAR_WIDTH * 0.5)),
                ..default()
            },
            BackgroundColor(Color::NONE),
            BorderColor::all(Color::NONE),
        ))
        .id();

    let thumb = commands
        .spawn((
            MistScrollThumb,
            MistSmokeRole::ToolbarButton,
            surface_smoke_for_role(MistSmokeSurfaceRole::ScalarFill, true),
            smoke_border_for_role(MistSmokeRole::ToolbarButton, false, 413),
            SmokeRingPadding::all(2.0),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                height: Val::Px(46.0),
                border_radius: BorderRadius::all(Val::Px(SCROLLBAR_WIDTH * 0.5)),
                ..default()
            },
            BackgroundColor(Color::NONE),
            BorderColor::all(Color::NONE),
        ))
        .id();

    commands.entity(viewport).add_child(content);
    commands.entity(track).add_child(thumb);
    commands.entity(root).add_children(&[viewport, track]);
    commands.entity(root).insert(MistScrollParts {
        viewport,
        content,
        track,
        thumb,
    });

    (root, content)
}

pub fn spawn_mist_slider(commands: &mut Commands, width: f32, value: f32) -> Entity {
    let slider = MistSlider::new(0.0, 1.0);
    let t = slider.normalize(value);
    let root = commands
        .spawn((
            slider,
            MistSliderValue(slider.denormalize(t)),
            Button,
            RelativeCursorPosition::default(),
            MistSmokeRole::ScalarControl,
            surface_smoke_for_widget_role(MistSmokeRole::ScalarControl, false),
            MistInteractiveStyle::default(),
            smoke_border_for_role(MistSmokeRole::ScalarControl, false, 4),
            smoke_padding_for_role(MistSmokeRole::ScalarControl),
            Node {
                width: Val::Px(width),
                min_height: Val::Px(40.0),
                position_type: PositionType::Relative,
                border: UiRect::all(Val::Px(0.0)),
                border_radius: BorderRadius::all(Val::Px(18.0)),
                ..default()
            },
            BackgroundColor(control_fill()),
            BorderColor::all(Color::NONE),
        ))
        .id();
    let track = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(SLIDER_TRACK_INSET),
                right: Val::Px(SLIDER_TRACK_INSET),
                top: Val::Px(15.0),
                height: Val::Px(SLIDER_TRACK_HEIGHT),
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(7.0)),
                ..default()
            },
            surface_smoke_for_role(MistSmokeSurfaceRole::ScalarTrack, true),
            BackgroundColor(Color::NONE),
            BorderColor::all(Color::NONE),
        ))
        .id();

    let fill = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                width: Val::Percent(t * 100.0),
                border_radius: BorderRadius::all(Val::Px(7.0)),
                ..default()
            },
            surface_smoke_for_role(MistSmokeSurfaceRole::ScalarFill, t > 0.01),
            BackgroundColor(Color::NONE),
        ))
        .id();
    let mid_tick = commands
        .spawn((
            Node {
                width: Val::Px(2.0),
                height: Val::Px(16.0),
                border_radius: BorderRadius::all(Val::Px(1.0)),
                ..default()
            },
            surface_smoke_for_role(MistSmokeSurfaceRole::HeaderBody, false),
            BackgroundColor(Color::NONE),
            BorderColor::all(Color::NONE),
        ))
        .id();

    let handle = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(
                    SLIDER_TRACK_INSET
                        + t * (width - SLIDER_TRACK_INSET * 2.0 - SLIDER_HANDLE_WIDTH),
                ),
                top: Val::Px(4.0),
                width: Val::Px(SLIDER_HANDLE_WIDTH),
                height: Val::Px(SLIDER_HANDLE_HEIGHT),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..default()
            },
            surface_smoke_for_role(MistSmokeSurfaceRole::AccentOrb, true),
            BackgroundColor(Color::NONE),
            BorderColor::all(Color::NONE),
        ))
        .id();
    let handle_core = commands
        .spawn((
            Node {
                width: Val::Px(3.0),
                height: Val::Px(18.0),
                border_radius: BorderRadius::all(Val::Px(2.0)),
                ..default()
            },
            surface_smoke_for_role(MistSmokeSurfaceRole::AccentChip, true),
            BackgroundColor(Color::NONE),
            BorderColor::all(Color::NONE),
        ))
        .id();

    commands.entity(handle).add_child(handle_core);
    commands.entity(track).add_children(&[fill, mid_tick]);
    commands.entity(root).add_children(&[track, handle]);
    commands.entity(root).insert(MistSliderParts {
        track,
        fill,
        handle,
    });
    root
}

pub fn spawn_mist_progress_bar(commands: &mut Commands, width: f32, target: f32) -> Entity {
    let target = target.clamp(0.0, 1.0);
    let root = commands
        .spawn((
            MistProgressBar {
                displayed: target,
                target,
            },
            MistSmokeRole::ScalarControl,
            surface_smoke_for_role(MistSmokeSurfaceRole::ScalarTrack, false),
            smoke_border_for_role(MistSmokeRole::ScalarControl, false, 5),
            smoke_padding_for_role(MistSmokeRole::ScalarControl),
            Node {
                width: Val::Px(width),
                min_height: Val::Px(24.0),
                position_type: PositionType::Relative,
                border: UiRect::all(Val::Px(0.0)),
                border_radius: BorderRadius::all(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(panel_fill()),
            BorderColor::all(Color::NONE),
        ))
        .id();

    let fill = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                width: Val::Percent(target * 100.0),
                border_radius: BorderRadius::all(Val::Px(12.0)),
                ..default()
            },
            surface_smoke_for_role(MistSmokeSurfaceRole::ScalarFill, target > 0.01),
            BackgroundColor(Color::NONE),
        ))
        .id();

    commands.entity(root).add_child(fill);
    commands.entity(root).insert(MistProgressParts { fill });
    root
}

pub fn spawn_mist_input_field(
    commands: &mut Commands,
    font: &Handle<Font>,
    width: f32,
    config: MistInputField,
) -> Entity {
    let has_text = !config.value.is_empty();
    let root = commands
        .spawn((
            Button,
            MistSmokeRole::StandardButton,
            surface_smoke_for_widget_role(MistSmokeRole::StandardButton, false),
            MistInteractiveStyle::default(),
            smoke_border_for_role(MistSmokeRole::StandardButton, false, 6),
            smoke_padding_for_role(MistSmokeRole::StandardButton),
            config,
            Node {
                width: Val::Px(width),
                min_height: Val::Px(CONTROL_HEIGHT),
                padding: UiRect::axes(Val::Px(CONTROL_PADDING_X), Val::Px(CONTROL_PADDING_Y)),
                column_gap: Val::Px(8.0),
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(0.0)),
                border_radius: BorderRadius::all(Val::Px(CONTROL_RADIUS)),
                ..default()
            },
            BackgroundColor(control_fill()),
            BorderColor::all(Color::NONE),
        ))
        .id();

    let placeholder_text = commands
        .spawn((
            text_line(font, "", 18.0, text_secondary()),
            if has_text {
                Visibility::Hidden
            } else {
                Visibility::Inherited
            },
        ))
        .id();
    let value_text = commands
        .spawn(text_line(font, "", 18.0, text_primary()))
        .id();
    let caret = commands
        .spawn((
            text_line(font, "▏", 20.0, text_primary()),
            Visibility::Hidden,
        ))
        .id();

    commands
        .entity(root)
        .add_children(&[placeholder_text, value_text, caret]);
    commands.entity(root).insert(MistInputParts {
        value_text,
        placeholder_text,
        caret,
    });
    root
}

pub fn attach_mist_tooltip(
    commands: &mut Commands,
    anchor: Entity,
    font: &Handle<Font>,
    label: impl Into<String>,
    max_width: f32,
) -> Entity {
    let label = label.into();
    commands
        .entity(anchor)
        .insert((MistTooltipAnchor, RelativeCursorPosition::default()));

    let tooltip = commands
        .spawn((
            MistTooltip::default(),
            MistTooltipOwner(anchor),
            MistSmokeRole::PanelFrame,
            surface_smoke_for_widget_role(MistSmokeRole::PanelFrame, false),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                bottom: Val::Percent(100.0),
                max_width: Val::Px(max_width),
                padding: UiRect::all(Val::Px(12.0)),
                margin: UiRect::bottom(Val::Px(8.0)),
                border: UiRect::all(Val::Px(0.0)),
                border_radius: BorderRadius::all(Val::Px(14.0)),
                display: Display::None,
                ..default()
            },
            smoke_border_for_role(MistSmokeRole::PanelFrame, false, 13),
            smoke_padding_for_role(MistSmokeRole::PanelFrame),
            BackgroundColor(panel_fill()),
            BorderColor::all(Color::NONE),
            GlobalZIndex(80),
            Visibility::Hidden,
            children![text_block(font, &label, 15.0, text_primary())],
        ))
        .id();

    commands.entity(anchor).add_child(tooltip);
    tooltip
}

pub fn spawn_mist_tabs(
    commands: &mut Commands,
    font: &Handle<Font>,
    width: f32,
    labels: impl IntoIterator<Item = impl Into<String>>,
    selected: usize,
) -> Entity {
    let labels: Vec<String> = labels.into_iter().map(Into::into).collect();
    let selected = clamp_choice_index(labels.len(), selected);
    let root = commands
        .spawn((
            MistTabs { selected },
            MistTabLabels(labels.clone()),
            MistSmokeRole::PanelFrame,
            surface_smoke_for_widget_role(MistSmokeRole::PanelFrame, false),
            smoke_border_for_role(MistSmokeRole::PanelFrame, false, 11),
            smoke_padding_for_role(MistSmokeRole::PanelFrame),
            Node {
                width: Val::Px(width),
                min_height: Val::Px(CONTROL_HEIGHT),
                padding: UiRect::all(Val::Px(6.0)),
                column_gap: Val::Px(6.0),
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(0.0)),
                border_radius: BorderRadius::all(Val::Px(18.0)),
                ..default()
            },
            BackgroundColor(control_fill()),
            BorderColor::all(Color::NONE),
        ))
        .id();

    for (index, label) in labels.iter().enumerate() {
        let button = commands
            .spawn((
                Button,
                MistTabOwner(root),
                MistTabButton { index },
                MistSmokeRole::DropdownOption,
                surface_smoke_for_widget_role(MistSmokeRole::DropdownOption, index == selected),
                smoke_border_for_role(
                    MistSmokeRole::DropdownOption,
                    false,
                    (root.to_bits() ^ index as u64) + 110,
                ),
                smoke_padding_for_role(MistSmokeRole::DropdownOption),
                Node {
                    flex_grow: 1.0,
                    min_height: Val::Px(CONTROL_HEIGHT - 12.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(0.0)),
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                BorderColor::all(Color::NONE),
            ))
            .id();
        let label_text = commands
            .spawn(text_line(font, label, 17.0, text_secondary()))
            .id();
        commands.entity(button).add_child(label_text);
        commands
            .entity(button)
            .insert(MistTabParts { label: label_text });
        commands.entity(root).add_child(button);
    }

    root
}

pub fn spawn_mist_dialog(
    commands: &mut Commands,
    font: &Handle<Font>,
    title: impl Into<String>,
    body: impl Into<String>,
    width: f32,
) -> Entity {
    let title = title.into();
    let body = body.into();
    let root = commands
        .spawn((
            MistDialog::default(),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::None,
                ..default()
            },
            Visibility::Hidden,
            GlobalZIndex(120),
        ))
        .id();

    let backdrop = commands
        .spawn((
            MistDialogBackdrop,
            MistDialogOwner(root),
            Button,
            surface_smoke_for_role(MistSmokeSurfaceRole::BackdropVeil, false),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .id();

    let panel = commands
        .spawn((
            MistSmokeRole::DialogFrame,
            surface_smoke_for_widget_role(MistSmokeRole::DialogFrame, false),
            smoke_border_for_role(MistSmokeRole::DialogFrame, false, 14),
            smoke_padding_for_role(MistSmokeRole::DialogFrame),
            Node {
                width: Val::Px(width),
                max_width: Val::Percent(88.0),
                padding: UiRect::all(Val::Px(18.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(14.0),
                border: UiRect::all(Val::Px(0.0)),
                border_radius: BorderRadius::all(Val::Px(22.0)),
                ..default()
            },
            BackgroundColor(panel_fill()),
            BorderColor::all(Color::NONE),
        ))
        .id();

    let header = commands
        .spawn((Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            column_gap: Val::Px(12.0),
            ..default()
        },))
        .id();

    let title_text = commands
        .spawn(text_line(font, &title, 24.0, text_primary()))
        .id();
    commands.entity(title_text).insert(Node {
        flex_grow: 1.0,
        ..default()
    });

    let close_button = commands
        .spawn((
            MistDialogCloseButton,
            MistDialogOwner(root),
            Button,
            MistSmokeRole::ToolbarButton,
            surface_smoke_for_widget_role(MistSmokeRole::ToolbarButton, false),
            MistInteractiveStyle::default(),
            smoke_border_for_role(MistSmokeRole::ToolbarButton, false, 15),
            smoke_padding_for_role(MistSmokeRole::ToolbarButton),
            Node {
                width: Val::Px(40.0),
                min_height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(0.0)),
                border_radius: BorderRadius::all(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(control_fill()),
            BorderColor::all(Color::NONE),
            children![text_line(font, "x", 18.0, text_primary())],
        ))
        .id();

    let body_text = commands
        .spawn(text_block(font, &body, 17.0, text_secondary()))
        .id();

    commands
        .entity(header)
        .add_children(&[title_text, close_button]);
    commands.entity(panel).add_children(&[header, body_text]);
    commands.entity(root).add_children(&[backdrop, panel]);
    commands.entity(root).insert(MistDialogParts {
        backdrop,
        panel,
        close_button,
        title: title_text,
        body: body_text,
    });

    root
}

pub fn spawn_mist_dropdown(
    commands: &mut Commands,
    font: &Handle<Font>,
    width: f32,
    options: impl IntoIterator<Item = impl Into<String>>,
) -> Entity {
    let options: Vec<String> = options.into_iter().map(Into::into).collect();
    let root = commands
        .spawn((
            MistDropdown::default(),
            MistDropdownOptions(options.clone()),
            Node {
                position_type: PositionType::Relative,
                width: Val::Px(width),
                min_height: Val::Px(CONTROL_HEIGHT),
                ..default()
            },
        ))
        .id();

    let trigger = commands
        .spawn((
            MistTrigger,
            MistDropdownTrigger,
            MistDropdownOwner(root),
            Button,
            MistSmokeRole::TriggerButton,
            surface_smoke_for_widget_role(MistSmokeRole::TriggerButton, false),
            trigger_interactive_style(),
            smoke_border_for_role(MistSmokeRole::TriggerButton, false, 7),
            smoke_padding_for_role(MistSmokeRole::TriggerButton),
            Node {
                width: Val::Percent(100.0),
                min_height: Val::Px(CONTROL_HEIGHT),
                padding: UiRect::axes(Val::Px(CONTROL_PADDING_X), Val::Px(CONTROL_PADDING_Y)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(0.0)),
                border_radius: BorderRadius::all(Val::Px(CONTROL_RADIUS)),
                ..default()
            },
            BackgroundColor(control_fill()),
            BorderColor::all(Color::NONE),
        ))
        .id();

    let label_text = commands
        .spawn(text_line(font, "", 18.0, text_primary()))
        .id();
    let chevron = commands
        .spawn(text_line(font, "v", 18.0, text_secondary()))
        .id();
    commands
        .entity(trigger)
        .add_children(&[label_text, chevron]);

    let menu = commands
        .spawn((
            MistSmokeRole::PanelFrame,
            surface_smoke_for_widget_role(MistSmokeRole::PanelFrame, false),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(CONTROL_HEIGHT + 4.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(8.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                display: Display::None,
                border: UiRect::all(Val::Px(0.0)),
                border_radius: BorderRadius::all(Val::Px(16.0)),
                ..default()
            },
            smoke_border_for_role(MistSmokeRole::PanelFrame, false, 8),
            smoke_padding_for_role(MistSmokeRole::PanelFrame),
            BackgroundColor(panel_fill()),
            BorderColor::all(Color::NONE),
            Visibility::Hidden,
            GlobalZIndex(20),
        ))
        .id();

    for (index, option) in options.iter().enumerate() {
        let item = commands
            .spawn((
                MistDropdownOwner(root),
                MistDropdownItem { index },
                Button,
                MistSmokeRole::DropdownOption,
                surface_smoke_for_widget_role(MistSmokeRole::DropdownOption, false),
                MistInteractiveStyle::default(),
                smoke_border_for_role(
                    MistSmokeRole::DropdownOption,
                    false,
                    (root.to_bits() ^ index as u64) + 70,
                ),
                smoke_padding_for_role(MistSmokeRole::DropdownOption),
                Node {
                    width: Val::Percent(100.0),
                    min_height: Val::Px(34.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(0.0)),
                    border_radius: BorderRadius::all(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(control_fill()),
                BorderColor::all(Color::NONE),
                children![text_line(font, option, 17.0, text_primary())],
            ))
            .id();
        commands.entity(menu).add_child(item);
    }

    commands.entity(root).add_children(&[trigger, menu]);
    commands
        .entity(root)
        .insert(MistDropdownParts { label_text, menu });
    root
}

fn tree_depth(nodes: &[MistTreeNodeSpec], index: usize) -> usize {
    let mut depth = 0usize;
    let mut current = nodes.get(index).and_then(|node| node.parent);
    while let Some(parent) = current {
        depth += 1;
        current = nodes.get(parent).and_then(|node| node.parent);
    }
    depth
}

fn tree_has_children(nodes: &[MistTreeNodeSpec], index: usize) -> bool {
    nodes.iter().any(|node| node.parent == Some(index))
}

fn tree_branch_visible(nodes: &[MistTreeNodeSpec], index: usize) -> bool {
    let mut current = nodes.get(index).and_then(|node| node.parent);
    while let Some(parent) = current {
        let Some(parent_node) = nodes.get(parent) else {
            break;
        };
        if !parent_node.expanded {
            return false;
        }
        current = parent_node.parent;
    }
    true
}

pub fn spawn_mist_badge(
    commands: &mut Commands,
    font: &Handle<Font>,
    label: impl Into<String>,
) -> Entity {
    let label = label.into();
    commands
        .spawn((
            MistBadge,
            MistSmokeRole::FeedbackChip,
            surface_smoke_for_widget_role(MistSmokeRole::FeedbackChip, false),
            smoke_border_for_role(MistSmokeRole::FeedbackChip, false, 160),
            smoke_padding_for_role(MistSmokeRole::FeedbackChip),
            Node {
                min_height: Val::Px(32.0),
                padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(999.0)),
                ..default()
            },
            BackgroundColor(Color::NONE),
            BorderColor::all(Color::NONE),
            children![text_line(font, &label, 15.0, text_primary())],
        ))
        .id()
}

pub fn spawn_mist_chip(
    commands: &mut Commands,
    font: &Handle<Font>,
    label: impl Into<String>,
    width: f32,
) -> Entity {
    let label = label.into();
    commands
        .spawn((
            MistChip,
            MistButton,
            Button,
            MistSmokeRole::FeedbackChip,
            surface_smoke_for_widget_role(MistSmokeRole::FeedbackChip, false),
            feedback_interactive_style(),
            smoke_border_for_role(MistSmokeRole::FeedbackChip, false, 161),
            smoke_padding_for_role(MistSmokeRole::FeedbackChip),
            Node {
                width: Val::Px(width),
                min_height: Val::Px(36.0),
                padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(999.0)),
                ..default()
            },
            BackgroundColor(Color::NONE),
            BorderColor::all(Color::NONE),
            children![text_line(font, &label, 15.0, text_primary())],
        ))
        .id()
}

pub fn spawn_mist_status_pill(
    commands: &mut Commands,
    font: &Handle<Font>,
    label: impl Into<String>,
    active: bool,
) -> Entity {
    let label = label.into();
    let root = commands
        .spawn((
            MistStatusPill,
            MistSmokeRole::FeedbackChip,
            surface_smoke_for_widget_role(MistSmokeRole::FeedbackChip, active),
            smoke_border_for_role(MistSmokeRole::FeedbackChip, active, 162),
            smoke_padding_for_role(MistSmokeRole::FeedbackChip),
            Node {
                min_height: Val::Px(34.0),
                padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                column_gap: Val::Px(8.0),
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(999.0)),
                ..default()
            },
            BackgroundColor(Color::NONE),
            BorderColor::all(Color::NONE),
        ))
        .id();

    let orb = commands
        .spawn((
            Node {
                width: Val::Px(14.0),
                height: Val::Px(14.0),
                border_radius: BorderRadius::all(Val::Px(999.0)),
                ..default()
            },
            surface_smoke_for_role(MistSmokeSurfaceRole::AccentOrb, active),
            BackgroundColor(Color::NONE),
        ))
        .id();
    let text = commands
        .spawn(text_line(font, &label, 15.0, text_primary()))
        .id();
    commands.entity(root).add_children(&[orb, text]);
    root
}

pub fn spawn_mist_toast(
    commands: &mut Commands,
    font: &Handle<Font>,
    title: impl Into<String>,
    body: impl Into<String>,
    width: f32,
) -> Entity {
    let title = title.into();
    let body = body.into();
    let root = commands
        .spawn((
            MistToast { open: true },
            MistSmokeRole::DialogFrame,
            surface_smoke_for_widget_role(MistSmokeRole::DialogFrame, false),
            smoke_border_for_role(MistSmokeRole::DialogFrame, false, 170),
            smoke_padding_for_role(MistSmokeRole::DialogFrame),
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(28.0),
                top: Val::Px(28.0),
                width: Val::Px(width),
                max_width: Val::Percent(32.0),
                padding: UiRect::all(Val::Px(16.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                border_radius: BorderRadius::all(Val::Px(18.0)),
                ..default()
            },
            BackgroundColor(panel_fill()),
            BorderColor::all(Color::NONE),
            GlobalZIndex(90),
        ))
        .id();

    let header = commands
        .spawn((Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            column_gap: Val::Px(10.0),
            ..default()
        },))
        .id();
    let title_text = commands
        .spawn(text_line(font, &title, 18.0, text_primary()))
        .id();
    commands.entity(title_text).insert(Node {
        flex_grow: 1.0,
        ..default()
    });
    let close = commands
        .spawn((
            MistToastCloseButton,
            MistToastOwner(root),
            Button,
            MistSmokeRole::ToolbarButton,
            surface_smoke_for_widget_role(MistSmokeRole::ToolbarButton, false),
            MistInteractiveStyle::default(),
            smoke_border_for_role(MistSmokeRole::ToolbarButton, false, 171),
            smoke_padding_for_role(MistSmokeRole::ToolbarButton),
            Node {
                width: Val::Px(34.0),
                min_height: Val::Px(34.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::NONE),
            BorderColor::all(Color::NONE),
            children![text_line(font, "x", 16.0, text_primary())],
        ))
        .id();
    let body_text = commands
        .spawn(text_block(font, &body, 15.0, text_secondary()))
        .id();

    commands.entity(header).add_children(&[title_text, close]);
    commands.entity(root).add_children(&[header, body_text]);
    commands.entity(root).insert(MistToastParts {
        close_button: close,
    });
    root
}

pub fn spawn_mist_popover(
    commands: &mut Commands,
    font: &Handle<Font>,
    title: impl Into<String>,
    body: impl Into<String>,
    width: f32,
) -> Entity {
    let title = title.into();
    let body = body.into();
    let root = commands
        .spawn((
            MistPopover { open: true },
            MistSmokeRole::MenuFrame,
            surface_smoke_for_widget_role(MistSmokeRole::MenuFrame, false),
            smoke_border_for_role(MistSmokeRole::MenuFrame, false, 172),
            smoke_padding_for_role(MistSmokeRole::MenuFrame),
            Node {
                width: Val::Px(width),
                padding: UiRect::all(Val::Px(14.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                border_radius: BorderRadius::all(Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(panel_fill()),
            BorderColor::all(Color::NONE),
        ))
        .id();
    commands
        .entity(root)
        .insert(MistPopoverParts { panel: root });
    commands.entity(root).with_children(|parent| {
        parent.spawn(text_line(font, &title, 18.0, text_primary()));
        parent.spawn(text_block(font, &body, 15.0, text_secondary()));
    });
    root
}

pub fn spawn_mist_menu_list<I, S>(
    commands: &mut Commands,
    font: &Handle<Font>,
    width: f32,
    options: I,
) -> Entity
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let options: Vec<String> = options.into_iter().map(Into::into).collect();
    let root = commands
        .spawn((
            MistMenuList,
            MistMenuListOptions(options.clone()),
            MistSmokeRole::MenuFrame,
            surface_smoke_for_widget_role(MistSmokeRole::MenuFrame, false),
            smoke_border_for_role(MistSmokeRole::MenuFrame, false, 173),
            smoke_padding_for_role(MistSmokeRole::MenuFrame),
            Node {
                width: Val::Px(width),
                padding: UiRect::all(Val::Px(8.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                border_radius: BorderRadius::all(Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(panel_fill()),
            BorderColor::all(Color::NONE),
        ))
        .id();

    for (index, option) in options.iter().enumerate() {
        let item = commands
            .spawn((
                MistMenuListOwner(root),
                MistMenuListItem { index },
                Button,
                MistSmokeRole::DataItem,
                surface_smoke_for_widget_role(MistSmokeRole::DataItem, false),
                data_item_interactive_style(),
                smoke_border_for_role(
                    MistSmokeRole::DataItem,
                    false,
                    (root.to_bits() ^ index as u64) + 174,
                ),
                smoke_padding_for_role(MistSmokeRole::DataItem),
                Node {
                    width: Val::Percent(100.0),
                    min_height: Val::Px(34.0),
                    padding: UiRect::axes(Val::Px(10.0), Val::Px(8.0)),
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                BorderColor::all(Color::NONE),
                children![text_line(font, option, 16.0, text_primary())],
            ))
            .id();
        commands.entity(root).add_child(item);
    }

    root
}

pub fn spawn_mist_context_menu<I, S>(
    commands: &mut Commands,
    font: &Handle<Font>,
    width: f32,
    options: I,
) -> Entity
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let options: Vec<String> = options.into_iter().map(Into::into).collect();
    let root = commands
        .spawn((
            MistContextMenu { open: true },
            MistContextMenuOptions(options.clone()),
            Node {
                position_type: PositionType::Relative,
                width: Val::Px(width),
                ..default()
            },
        ))
        .id();

    let menu = commands
        .spawn((
            MistSmokeRole::MenuFrame,
            surface_smoke_for_widget_role(MistSmokeRole::MenuFrame, false),
            smoke_border_for_role(MistSmokeRole::MenuFrame, false, 175),
            smoke_padding_for_role(MistSmokeRole::MenuFrame),
            Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(8.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                border_radius: BorderRadius::all(Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(panel_fill()),
            BorderColor::all(Color::NONE),
        ))
        .id();

    for (index, option) in options.iter().enumerate() {
        let item = commands
            .spawn((
                MistContextMenuOwner(root),
                MistContextMenuItem { index },
                Button,
                MistSmokeRole::DataItem,
                surface_smoke_for_widget_role(MistSmokeRole::DataItem, false),
                data_item_interactive_style(),
                smoke_border_for_role(
                    MistSmokeRole::DataItem,
                    false,
                    (root.to_bits() ^ index as u64) + 176,
                ),
                smoke_padding_for_role(MistSmokeRole::DataItem),
                Node {
                    width: Val::Percent(100.0),
                    min_height: Val::Px(34.0),
                    padding: UiRect::axes(Val::Px(10.0), Val::Px(8.0)),
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                BorderColor::all(Color::NONE),
                children![text_line(font, option, 16.0, text_primary())],
            ))
            .id();
        commands.entity(menu).add_child(item);
    }

    commands.entity(root).add_child(menu);
    commands.entity(root).insert(MistContextMenuParts { menu });
    root
}

pub fn spawn_mist_accordion<I, S1, S2>(
    commands: &mut Commands,
    font: &Handle<Font>,
    width: f32,
    sections: I,
) -> Entity
where
    I: IntoIterator<Item = (S1, S2)>,
    S1: Into<String>,
    S2: Into<String>,
{
    let sections: Vec<(String, String)> = sections
        .into_iter()
        .map(|(label, body)| (label.into(), body.into()))
        .collect();
    let root = commands
        .spawn((
            MistAccordion,
            MistAccordionSections(sections.iter().map(|(label, _)| label.clone()).collect()),
            MistAccordionState(vec![true; sections.len()]),
            MistSmokeRole::DataFrame,
            surface_smoke_for_widget_role(MistSmokeRole::DataFrame, false),
            smoke_border_for_role(MistSmokeRole::DataFrame, false, 177),
            smoke_padding_for_role(MistSmokeRole::DataFrame),
            Node {
                width: Val::Px(width),
                padding: UiRect::all(Val::Px(8.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                border_radius: BorderRadius::all(Val::Px(18.0)),
                ..default()
            },
            BackgroundColor(panel_fill()),
            BorderColor::all(Color::NONE),
        ))
        .id();

    for (index, (label, body)) in sections.iter().enumerate() {
        let section = commands
            .spawn((Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                ..default()
            },))
            .id();
        let header = commands
            .spawn((
                MistAccordionOwner(root),
                MistAccordionSection { index },
                Button,
                MistSmokeRole::DataItem,
                surface_smoke_for_widget_role(MistSmokeRole::DataItem, false),
                data_item_interactive_style(),
                smoke_border_for_role(
                    MistSmokeRole::DataItem,
                    false,
                    (root.to_bits() ^ index as u64) + 178,
                ),
                smoke_padding_for_role(MistSmokeRole::DataItem),
                Node {
                    width: Val::Percent(100.0),
                    min_height: Val::Px(38.0),
                    padding: UiRect::axes(Val::Px(10.0), Val::Px(8.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                BorderColor::all(Color::NONE),
            ))
            .id();
        let title = commands
            .spawn(text_line(font, label, 16.0, text_primary()))
            .id();
        let chevron = commands
            .spawn(text_line(font, "▾", 16.0, text_secondary()))
            .id();
        commands.entity(header).add_children(&[title, chevron]);
        let body_node = commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    padding: UiRect::axes(Val::Px(10.0), Val::Px(6.0)),
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    ..default()
                },
                MistSmokeSurface::new(
                    surface_smoke_for_role(MistSmokeSurfaceRole::DataBody, true).config,
                )
                .with_inset(8.0, 6.0),
                BackgroundColor(Color::NONE),
                children![text_block(font, body, 15.0, text_secondary())],
            ))
            .id();
        commands.entity(header).insert(MistAccordionParts {
            body: body_node,
            chevron,
        });
        commands.entity(section).add_children(&[header, body_node]);
        commands.entity(root).add_child(section);
    }

    root
}

pub fn spawn_mist_segmented_action_row<I, S>(
    commands: &mut Commands,
    font: &Handle<Font>,
    width: f32,
    labels: I,
) -> Entity
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let labels: Vec<String> = labels.into_iter().map(Into::into).collect();
    let root = commands
        .spawn((
            MistSegmentedActionRow,
            MistSegmentedActionLabels(labels.clone()),
            MistSmokeRole::DataFrame,
            surface_smoke_for_widget_role(MistSmokeRole::DataFrame, false),
            smoke_border_for_role(MistSmokeRole::DataFrame, false, 179),
            smoke_padding_for_role(MistSmokeRole::DataFrame),
            Node {
                width: Val::Px(width),
                padding: UiRect::all(Val::Px(6.0)),
                column_gap: Val::Px(6.0),
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(panel_fill()),
            BorderColor::all(Color::NONE),
        ))
        .id();
    for (index, label) in labels.iter().enumerate() {
        let item = commands
            .spawn((
                MistSegmentedActionOwner(root),
                MistSegmentedActionButton { index },
                Button,
                MistSmokeRole::DataItem,
                surface_smoke_for_widget_role(MistSmokeRole::DataItem, false),
                data_item_interactive_style(),
                smoke_border_for_role(
                    MistSmokeRole::DataItem,
                    false,
                    (root.to_bits() ^ index as u64) + 180,
                ),
                smoke_padding_for_role(MistSmokeRole::DataItem),
                Node {
                    flex_grow: 1.0,
                    min_height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                BorderColor::all(Color::NONE),
            ))
            .id();
        let label_entity = commands
            .spawn(text_line(font, label, 15.0, text_primary()))
            .id();
        commands.entity(item).add_child(label_entity);
        commands.entity(item).insert(MistSegmentedActionParts {
            label: label_entity,
        });
        commands.entity(root).add_child(item);
    }
    root
}

pub fn spawn_mist_list_view<I, S>(
    commands: &mut Commands,
    font: &Handle<Font>,
    width: f32,
    height: f32,
    items: I,
    selected: Option<usize>,
) -> Entity
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let items: Vec<String> = items.into_iter().map(Into::into).collect();
    let (root, content) = spawn_mist_scroll_view(commands, width, height);
    commands
        .entity(root)
        .insert((MistListView { selected }, MistListItems(items.clone())));
    for (index, label) in items.iter().enumerate() {
        let row = commands
            .spawn((
                MistListOwner(root),
                MistListItem { index },
                Button,
                MistSmokeRole::DataItem,
                surface_smoke_for_widget_role(MistSmokeRole::DataItem, selected == Some(index)),
                data_item_interactive_style(),
                smoke_border_for_role(
                    MistSmokeRole::DataItem,
                    selected == Some(index),
                    (root.to_bits() ^ index as u64) + 181,
                ),
                smoke_padding_for_role(MistSmokeRole::DataItem),
                Node {
                    width: Val::Percent(100.0),
                    min_height: Val::Px(36.0),
                    padding: UiRect::axes(Val::Px(10.0), Val::Px(8.0)),
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                BorderColor::all(Color::NONE),
                children![text_line(font, label, 15.0, text_primary())],
            ))
            .id();
        commands.entity(content).add_child(row);
    }
    root
}

pub fn spawn_mist_table(
    commands: &mut Commands,
    font: &Handle<Font>,
    width: f32,
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
) -> Entity {
    let root = commands
        .spawn((
            MistTable { selected: None },
            MistTableColumns(columns.clone()),
            MistTableRows(rows.clone()),
            MistSmokeRole::DataFrame,
            surface_smoke_for_widget_role(MistSmokeRole::DataFrame, false),
            smoke_border_for_role(MistSmokeRole::DataFrame, false, 182),
            smoke_padding_for_role(MistSmokeRole::DataFrame),
            Node {
                width: Val::Px(width),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                border_radius: BorderRadius::all(Val::Px(18.0)),
                ..default()
            },
            BackgroundColor(panel_fill()),
            BorderColor::all(Color::NONE),
        ))
        .id();

    let header = commands
        .spawn((Node {
            width: Val::Percent(100.0),
            column_gap: Val::Px(6.0),
            align_items: AlignItems::Center,
            ..default()
        },))
        .id();
    for (index, label) in columns.iter().enumerate() {
        let cell = commands
            .spawn((
                MistTableOwner(root),
                MistTableHeaderButton { index },
                Button,
                MistSmokeRole::ToolbarButton,
                surface_smoke_for_role(MistSmokeSurfaceRole::HeaderBody, false),
                MistInteractiveStyle::default(),
                smoke_border_for_role(
                    MistSmokeRole::ToolbarButton,
                    false,
                    (root.to_bits() ^ index as u64) + 183,
                ),
                smoke_padding_for_role(MistSmokeRole::ToolbarButton),
                Node {
                    flex_grow: 1.0,
                    min_height: Val::Px(34.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                BorderColor::all(Color::NONE),
                children![text_line(font, label, 14.0, text_secondary())],
            ))
            .id();
        commands.entity(header).add_child(cell);
    }
    commands.entity(root).add_child(header);

    for (row_index, row) in rows.iter().enumerate() {
        let row_entity = commands
            .spawn((
                MistTableOwner(root),
                MistTableRowButton { index: row_index },
                Button,
                MistSmokeRole::DataItem,
                surface_smoke_for_widget_role(MistSmokeRole::DataItem, false),
                data_item_interactive_style(),
                smoke_border_for_role(
                    MistSmokeRole::DataItem,
                    false,
                    (root.to_bits() ^ row_index as u64) + 184,
                ),
                smoke_padding_for_role(MistSmokeRole::DataItem),
                Node {
                    width: Val::Percent(100.0),
                    padding: UiRect::axes(Val::Px(8.0), Val::Px(6.0)),
                    column_gap: Val::Px(6.0),
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                BorderColor::all(Color::NONE),
            ))
            .id();
        for cell in row {
            let cell_text = commands
                .spawn((
                    Node {
                        flex_grow: 1.0,
                        ..default()
                    },
                    children![text_line(font, cell, 14.0, text_primary())],
                ))
                .id();
            commands.entity(row_entity).add_child(cell_text);
        }
        commands.entity(root).add_child(row_entity);
    }

    root
}

pub fn spawn_mist_tree_view(
    commands: &mut Commands,
    font: &Handle<Font>,
    width: f32,
    nodes: Vec<MistTreeNodeSpec>,
    selected: Option<usize>,
) -> Entity {
    let root = commands
        .spawn((
            MistTreeView { selected },
            MistTreeNodes(nodes.clone()),
            MistSmokeRole::DataFrame,
            surface_smoke_for_widget_role(MistSmokeRole::DataFrame, false),
            smoke_border_for_role(MistSmokeRole::DataFrame, false, 185),
            smoke_padding_for_role(MistSmokeRole::DataFrame),
            Node {
                width: Val::Px(width),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                border_radius: BorderRadius::all(Val::Px(18.0)),
                ..default()
            },
            BackgroundColor(panel_fill()),
            BorderColor::all(Color::NONE),
        ))
        .id();

    for (index, node) in nodes.iter().enumerate() {
        let row = commands
            .spawn((
                MistTreeOwner(root),
                MistTreeItem { index },
                Button,
                MistSmokeRole::DataItem,
                surface_smoke_for_widget_role(MistSmokeRole::DataItem, selected == Some(index)),
                data_item_interactive_style(),
                smoke_border_for_role(
                    MistSmokeRole::DataItem,
                    selected == Some(index),
                    (root.to_bits() ^ index as u64) + 186,
                ),
                smoke_padding_for_role(MistSmokeRole::DataItem),
                Node {
                    width: Val::Percent(100.0),
                    min_height: Val::Px(34.0),
                    padding: UiRect::new(
                        Val::Px(10.0 + (tree_depth(&nodes, index) as f32 * 16.0)),
                        Val::Px(10.0),
                        Val::Px(6.0),
                        Val::Px(6.0),
                    ),
                    column_gap: Val::Px(8.0),
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                BorderColor::all(Color::NONE),
            ))
            .id();
        let toggle = commands
            .spawn(text_line(
                font,
                if tree_has_children(&nodes, index) {
                    if node.expanded {
                        "▾"
                    } else {
                        "▸"
                    }
                } else {
                    "•"
                },
                14.0,
                text_secondary(),
            ))
            .id();
        let label = commands
            .spawn(text_line(font, &node.label, 15.0, text_primary()))
            .id();
        commands.entity(row).add_children(&[toggle, label]);
        commands
            .entity(row)
            .insert(MistTreeItemParts { toggle, label });
        commands.entity(root).add_child(row);
    }

    root
}

pub fn spawn_mist_grid_view<I, S>(
    commands: &mut Commands,
    font: &Handle<Font>,
    width: f32,
    columns: usize,
    items: I,
    selected: Option<usize>,
) -> Entity
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let items: Vec<String> = items.into_iter().map(Into::into).collect();
    let root = commands
        .spawn((
            MistGridView { selected },
            MistGridItems(items.clone()),
            MistSmokeRole::DataFrame,
            surface_smoke_for_widget_role(MistSmokeRole::DataFrame, false),
            smoke_border_for_role(MistSmokeRole::DataFrame, false, 187),
            smoke_padding_for_role(MistSmokeRole::DataFrame),
            Node {
                width: Val::Px(width),
                padding: UiRect::all(Val::Px(10.0)),
                flex_wrap: FlexWrap::Wrap,
                column_gap: Val::Px(8.0),
                row_gap: Val::Px(8.0),
                border_radius: BorderRadius::all(Val::Px(18.0)),
                ..default()
            },
            BackgroundColor(panel_fill()),
            BorderColor::all(Color::NONE),
        ))
        .id();

    let columns = columns.max(1) as f32;
    let tile_width = ((width - 20.0) / columns).max(96.0);
    for (index, label) in items.iter().enumerate() {
        let item = commands
            .spawn((
                MistGridOwner(root),
                MistGridItem { index },
                Button,
                MistSmokeRole::DataItem,
                surface_smoke_for_widget_role(MistSmokeRole::DataItem, selected == Some(index)),
                data_item_interactive_style(),
                smoke_border_for_role(
                    MistSmokeRole::DataItem,
                    selected == Some(index),
                    (root.to_bits() ^ index as u64) + 188,
                ),
                smoke_padding_for_role(MistSmokeRole::DataItem),
                Node {
                    width: Val::Px(tile_width - 8.0),
                    min_height: Val::Px(76.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    border_radius: BorderRadius::all(Val::Px(14.0)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                BorderColor::all(Color::NONE),
                children![text_block(font, label, 15.0, text_primary())],
            ))
            .id();
        commands.entity(root).add_child(item);
    }

    root
}

fn sync_interactive_styles(
    mut query: Query<
        (
            &Interaction,
            &MistInteractiveStyle,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, style, mut fill, mut border) in &mut query {
        match *interaction {
            Interaction::None => {
                fill.0 = style.idle_fill;
                *border = BorderColor::all(style.idle_border);
            }
            Interaction::Hovered => {
                fill.0 = style.hover_fill;
                *border = BorderColor::all(style.hover_border);
            }
            Interaction::Pressed => {
                fill.0 = style.pressed_fill;
                *border = BorderColor::all(style.pressed_border);
            }
        }
    }
}

fn sync_interactive_smoke_borders(
    mut query: Query<
        (
            Entity,
            &Interaction,
            &MistSmokeRole,
            &mut SmokeBorder,
            Option<&mut MistSmokeConfig>,
            Option<&mut MistSmokeSurface>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (entity, interaction, role, mut smoke, config, surface) in &mut query {
        let edge_state = edge_state_from_interaction(*interaction);
        let next_config = smoke_config_for_edge_state(*role, edge_state);
        *smoke = derived_screen_ring(next_config);
        if let Some(mut config) = config {
            *config = next_config;
        } else {
            *smoke = smoke_border_for_role_state(*role, edge_state, entity.to_bits());
        }
        if let Some(mut surface) = surface {
            *surface =
                surface_smoke_for_widget_state(*role, surface_state_from_interaction(*interaction));
        }
    }
}

fn emit_button_pressed(
    query: Query<(Entity, &Interaction), (Changed<Interaction>, With<MistButton>)>,
    mut events: MessageWriter<MistButtonPressed>,
) {
    for (entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            events.write(MistButtonPressed { entity });
        }
    }
}

fn emit_trigger_pressed(
    query: Query<(Entity, &Interaction), (Changed<Interaction>, With<MistTrigger>)>,
    mut events: MessageWriter<MistTriggerPressed>,
) {
    for (entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            events.write(MistTriggerPressed { entity });
        }
    }
}

fn toggle_checkboxes(
    mut query: Query<
        (Entity, &Interaction, &mut MistCheckboxState),
        (Changed<Interaction>, With<MistCheckbox>),
    >,
    mut events: MessageWriter<MistCheckboxChanged>,
) {
    for (entity, interaction, mut state) in &mut query {
        if *interaction != Interaction::Pressed {
            continue;
        }
        state.checked = !state.checked;
        events.write(MistCheckboxChanged {
            entity,
            checked: state.checked,
        });
    }
}

fn sync_checkbox_visuals(
    query: Query<
        (&MistCheckboxState, &MistCheckboxParts),
        Or<(Changed<MistCheckboxState>, Added<MistCheckboxState>)>,
    >,
    mut surfaces: Query<&mut MistSmokeSurface>,
    mut smoke_configs: Query<&mut MistSmokeConfig>,
    mut borders: Query<&mut SmokeBorder>,
    mut text_query: Query<&mut Text>,
) {
    for (state, parts) in &query {
        let indicator_smoke = checkbox_indicator_smoke_config(state.checked);
        if let Ok(mut surface) = surfaces.get_mut(parts.indicator) {
            *surface = surface_smoke_for_role(MistSmokeSurfaceRole::AccentChip, state.checked);
        }
        if let Ok(mut smoke_config) = smoke_configs.get_mut(parts.indicator) {
            *smoke_config = indicator_smoke;
        }
        if let Ok(mut border) = borders.get_mut(parts.indicator) {
            *border = derived_screen_ring(indicator_smoke);
        }
        if let Ok(mut tag_surface) = surfaces.get_mut(parts.tag) {
            *tag_surface = surface_smoke_for_role(MistSmokeSurfaceRole::AccentChip, state.checked);
        }
        if let Ok(mut text) = text_query.get_mut(parts.glyph) {
            text.clear();
            text.push_str(if state.checked { "✓" } else { "" });
        }
        if let Ok(mut text) = text_query.get_mut(parts.tag_text) {
            text.clear();
            text.push_str(if state.checked { "ON" } else { "OFF" });
        }
    }
}

fn scroll_mist_views(
    mut wheel_events: MessageReader<MouseWheel>,
    roots: Query<&MistScrollParts, With<MistScrollView>>,
    mut viewports: Query<
        (&RelativeCursorPosition, &ComputedNode, &mut ScrollPosition),
        With<MistScrollViewport>,
    >,
    content_nodes: Query<&ComputedNode, With<MistScrollContent>>,
) {
    let mut delta_pixels = 0.0;
    for event in wheel_events.read() {
        delta_pixels += match event.unit {
            MouseScrollUnit::Line => event.y * SCROLL_STEP_PX,
            MouseScrollUnit::Pixel => event.y,
        };
    }

    if delta_pixels.abs() <= f32::EPSILON {
        return;
    }

    for parts in &roots {
        let Ok((cursor, viewport_node, mut scroll_position)) = viewports.get_mut(parts.viewport)
        else {
            continue;
        };
        if cursor.normalized.is_none() {
            continue;
        }
        let Ok(content_node) = content_nodes.get(parts.content) else {
            continue;
        };

        let max_offset = (content_node.size().y - viewport_node.size().y).max(0.0);
        scroll_position.0.y = (scroll_position.0.y - delta_pixels).clamp(0.0, max_offset);
    }
}

fn sync_scrollbar_visuals(
    roots: Query<&MistScrollParts, With<MistScrollView>>,
    viewports: Query<(&ComputedNode, &ScrollPosition), With<MistScrollViewport>>,
    content_nodes: Query<&ComputedNode, With<MistScrollContent>>,
    track_nodes: Query<&ComputedNode, With<MistScrollTrack>>,
    mut nodes: Query<&mut Node>,
    mut surfaces: Query<&mut MistSmokeSurface>,
    mut borders: Query<&mut SmokeBorder>,
) {
    for parts in &roots {
        let Ok((viewport_node, scroll_position)) = viewports.get(parts.viewport) else {
            continue;
        };
        let Ok(content_node) = content_nodes.get(parts.content) else {
            continue;
        };
        let Ok(track_node) = track_nodes.get(parts.track) else {
            continue;
        };

        let viewport_h = viewport_node.size().y.max(1.0);
        let content_h = content_node.size().y.max(viewport_h);
        let track_h = track_node.size().y.max(1.0);
        let max_offset = (content_h - viewport_h).max(0.0);
        let visible_ratio = (viewport_h / content_h).clamp(0.0, 1.0);
        let thumb_min = SCROLLBAR_MIN_THUMB_HEIGHT.min(track_h);
        let thumb_h = (track_h * visible_ratio).clamp(thumb_min, track_h);
        let progress = if max_offset <= f32::EPSILON {
            0.0
        } else {
            (scroll_position.0.y / max_offset).clamp(0.0, 1.0)
        };
        let thumb_top = (track_h - thumb_h).max(0.0) * progress;
        let scrollable = max_offset > 2.0;

        if let Ok(mut thumb_node) = nodes.get_mut(parts.thumb) {
            thumb_node.top = Val::Px(thumb_top);
            thumb_node.height = Val::Px(thumb_h);
            thumb_node.display = if scrollable {
                Display::Flex
            } else {
                Display::None
            };
        }

        if let Ok(mut track_surface) = surfaces.get_mut(parts.track) {
            *track_surface = surface_smoke_for_role(MistSmokeSurfaceRole::ScalarTrack, scrollable);
        }
        if let Ok(mut thumb_surface) = surfaces.get_mut(parts.thumb) {
            *thumb_surface = surface_smoke_for_role(MistSmokeSurfaceRole::ScalarFill, scrollable);
        }
        if let Ok(mut track_border) = borders.get_mut(parts.track) {
            *track_border = smoke_border_for_role(MistSmokeRole::ScalarControl, scrollable, 412);
        }
        if let Ok(mut thumb_border) = borders.get_mut(parts.thumb) {
            *thumb_border = smoke_border_for_role(MistSmokeRole::ToolbarButton, scrollable, 413);
        }
    }
}

fn sync_tooltips(
    anchors: Query<&RelativeCursorPosition, With<MistTooltipAnchor>>,
    mut tooltips: Query<(&MistTooltip, &MistTooltipOwner, &mut Visibility, &mut Node)>,
) {
    for (tooltip, owner, mut visibility, mut node) in &mut tooltips {
        let hovered = anchors
            .get(owner.0)
            .ok()
            .and_then(|cursor| cursor.normalized)
            .is_some();
        let visible = tooltip.enabled && hovered;
        *visibility = if visible {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        node.display = if visible {
            Display::Flex
        } else {
            Display::None
        };
    }
}

fn select_radio_items(
    mut query: Query<
        (&Interaction, &MistRadioOption, &MistRadioOwner),
        (Changed<Interaction>, With<Button>),
    >,
    mut groups: Query<(&MistRadioOptions, &mut MistRadioGroup)>,
    mut events: MessageWriter<MistRadioChanged>,
) {
    for (interaction, option, owner) in &mut query {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Ok((options, mut group)) = groups.get_mut(owner.0) {
            group.selected = clamp_choice_index(options.0.len(), option.index);
            let label = options.0.get(group.selected).cloned().unwrap_or_default();
            events.write(MistRadioChanged {
                entity: owner.0,
                selected: group.selected,
                label,
            });
        }
    }
}

fn sync_radio_visuals(
    groups: Query<&MistRadioGroup>,
    items: Query<
        (
            Entity,
            &MistRadioOwner,
            &MistRadioOption,
            &MistRadioParts,
            &Interaction,
        ),
        With<Button>,
    >,
    mut backgrounds: Query<&mut BackgroundColor>,
    mut borders: Query<&mut BorderColor>,
    mut texts: Query<&mut Text>,
    mut text_colors: Query<&mut TextColor>,
    mut surfaces: Query<&mut MistSmokeSurface>,
) {
    for (entity, owner, option, parts, interaction) in &items {
        let Ok(group) = groups.get(owner.0) else {
            continue;
        };
        let selected = group.selected == option.index;

        if let Ok(mut background) = backgrounds.get_mut(entity) {
            background.0 = Color::NONE;
        }
        if let Ok(mut border) = borders.get_mut(entity) {
            *border = BorderColor::all(Color::NONE);
        }
        if let Ok(mut item_surface) = surfaces.get_mut(entity) {
            *item_surface = surface_smoke_for_role(
                MistSmokeSurfaceRole::OptionBody,
                selected || *interaction != Interaction::None,
            );
        }
        if let Ok(mut indicator_surface) = surfaces.get_mut(parts.indicator) {
            *indicator_surface = surface_smoke_for_role(MistSmokeSurfaceRole::AccentChip, selected);
        }
        if let Ok(mut glyph_text) = texts.get_mut(parts.glyph) {
            glyph_text.clear();
            glyph_text.push_str(if selected { "•" } else { "" });
        }
        if let Ok(mut label_color) = text_colors.get_mut(parts.label) {
            label_color.0 = if selected {
                text_primary()
            } else {
                text_secondary()
            };
        }
    }
}

fn toggle_switches(
    mut query: Query<
        (Entity, &Interaction, &mut MistSwitchState),
        (Changed<Interaction>, With<MistSwitch>),
    >,
    mut events: MessageWriter<MistSwitchChanged>,
) {
    for (entity, interaction, mut state) in &mut query {
        if *interaction != Interaction::Pressed {
            continue;
        }
        state.on = !state.on;
        events.write(MistSwitchChanged {
            entity,
            on: state.on,
        });
    }
}

fn sync_switch_visuals(
    query: Query<(Entity, &MistSwitchState, &MistSwitchParts, &Interaction), With<MistSwitch>>,
    mut backgrounds: Query<&mut BackgroundColor>,
    mut borders: Query<&mut BorderColor>,
    mut nodes: Query<&mut Node>,
    mut surfaces: Query<&mut MistSmokeSurface>,
) {
    for (entity, state, parts, interaction) in &query {
        if let Ok(mut background) = backgrounds.get_mut(entity) {
            background.0 = Color::NONE;
        }
        if let Ok(mut border) = borders.get_mut(entity) {
            *border = BorderColor::all(Color::NONE);
        }
        if let Ok(mut track_fill) = backgrounds.get_mut(parts.track) {
            track_fill.0 = Color::NONE;
        }
        if let Ok(mut track_border) = borders.get_mut(parts.track) {
            *track_border = BorderColor::all(Color::NONE);
        }
        if let Ok(mut track_surface) = surfaces.get_mut(parts.track) {
            *track_surface = surface_smoke_for_role(
                MistSmokeSurfaceRole::ScalarTrack,
                state.on || *interaction != Interaction::None,
            );
        }
        if let Ok(mut knob_surface) = surfaces.get_mut(parts.knob) {
            *knob_surface = surface_smoke_for_role(
                MistSmokeSurfaceRole::AccentOrb,
                state.on || *interaction != Interaction::None,
            );
        }
        if let Ok(mut knob_node) = nodes.get_mut(parts.knob) {
            let left = if state.on {
                SWITCH_TRACK_WIDTH - SWITCH_KNOB_SIZE - 3.0
            } else {
                2.0
            };
            knob_node.left = Val::Px(left);
        }
    }
}

fn drive_sliders(
    mouse: Res<ButtonInput<MouseButton>>,
    mut query: Query<
        (
            Entity,
            &Interaction,
            &RelativeCursorPosition,
            &MistSlider,
            &mut MistSliderValue,
        ),
        With<Button>,
    >,
    mut events: MessageWriter<MistSliderChanged>,
) {
    if !mouse.pressed(MouseButton::Left) {
        return;
    }

    for (entity, interaction, cursor, slider, mut value) in &mut query {
        if *interaction == Interaction::None {
            continue;
        }
        let Some(normalized) = cursor.normalized else {
            continue;
        };
        let next = slider.denormalize(normalized.x);
        if (next - value.0).abs() <= f32::EPSILON {
            continue;
        }
        value.0 = next;
        events.write(MistSliderChanged {
            entity,
            value: next,
        });
    }
}

fn sync_slider_visuals(
    query: Query<
        (
            &MistSlider,
            &MistSliderValue,
            &MistSliderParts,
            &ComputedNode,
        ),
        Or<(Changed<MistSliderValue>, Added<MistSliderValue>)>,
    >,
    mut nodes: Query<&mut Node>,
    mut surfaces: Query<&mut MistSmokeSurface>,
) {
    for (slider, value, parts, node) in &query {
        let normalized = slider.normalize(value.0);
        if let Ok(mut fill) = nodes.get_mut(parts.fill) {
            fill.width = Val::Percent(normalized * 100.0);
        }
        if let Ok(mut track_surface) = surfaces.get_mut(parts.track) {
            *track_surface = surface_smoke_for_role(MistSmokeSurfaceRole::ScalarTrack, true);
        }
        if let Ok(mut fill_surface) = surfaces.get_mut(parts.fill) {
            *fill_surface =
                surface_smoke_for_role(MistSmokeSurfaceRole::ScalarFill, normalized > 0.01);
        }
        if let Ok(mut handle_surface) = surfaces.get_mut(parts.handle) {
            *handle_surface = surface_smoke_for_role(MistSmokeSurfaceRole::AccentOrb, true);
        }
        if let Ok(mut handle) = nodes.get_mut(parts.handle) {
            let usable = (node.size().x - SLIDER_TRACK_INSET * 2.0 - SLIDER_HANDLE_WIDTH).max(0.0);
            let left = SLIDER_TRACK_INSET + usable * normalized;
            handle.left = Val::Px(left);
        }
    }
}

fn animate_progress_bars(
    time: Res<Time>,
    mut query: Query<(&mut MistProgressBar, &MistProgressParts)>,
    mut nodes: Query<&mut Node>,
    mut surfaces: Query<&mut MistSmokeSurface>,
) {
    let delta = time.delta_secs().clamp(0.0, 0.25);
    let lerp_factor = 1.0 - (-delta * PROGRESS_LERP_RATE).exp();

    for (mut bar, parts) in &mut query {
        let target = bar.target.clamp(0.0, 1.0);
        bar.displayed += (target - bar.displayed) * lerp_factor;
        if (target - bar.displayed).abs() < 0.001 {
            bar.displayed = target;
        }

        if let Ok(mut node) = nodes.get_mut(parts.fill) {
            node.width = Val::Percent(bar.displayed.clamp(0.0, 1.0) * 100.0);
        }
        if let Ok(mut fill_surface) = surfaces.get_mut(parts.fill) {
            *fill_surface =
                surface_smoke_for_role(MistSmokeSurfaceRole::ScalarFill, bar.displayed > 0.01);
        }
    }
}

fn focus_input_fields(
    mouse: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    mut focus: ResMut<MistInputFocusState>,
    query: Query<(Entity, &Interaction), (Changed<Interaction>, With<MistInputField>)>,
) {
    let mut pressed_input = None;
    for (entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            pressed_input = Some(entity);
        }
    }

    if let Some(entity) = pressed_input {
        if focus.active != Some(entity) {
            if let Some(previous) = focus.active.take() {
                commands.entity(previous).remove::<MistInputFocused>();
            }
            commands.entity(entity).insert(MistInputFocused);
            focus.active = Some(entity);
        }
        return;
    }

    if mouse.just_pressed(MouseButton::Left) {
        if let Some(previous) = focus.active.take() {
            commands.entity(previous).remove::<MistInputFocused>();
        }
    }
}

fn type_into_input_fields(
    mut commands: Commands,
    mut key_evr: MessageReader<KeyboardInput>,
    mut focus: ResMut<MistInputFocusState>,
    mut query: Query<(Entity, &mut MistInputField), With<MistInputFocused>>,
    mut changed: MessageWriter<MistInputChanged>,
    mut submitted: MessageWriter<MistInputSubmitted>,
) {
    let Some(active) = focus.active else {
        return;
    };

    let Ok((entity, mut input)) = query.get_mut(active) else {
        focus.active = None;
        return;
    };

    for ev in key_evr.read() {
        if ev.state != ButtonState::Pressed {
            continue;
        }

        let mut input_changed = false;
        if ev.key_code == KeyCode::Escape || matches!(ev.logical_key, Key::Escape) {
            commands.entity(entity).remove::<MistInputFocused>();
            focus.active = None;
            break;
        } else if ev.key_code == KeyCode::Enter || matches!(ev.logical_key, Key::Enter) {
            submitted.write(MistInputSubmitted {
                entity,
                value: input.value.clone(),
            });
            continue;
        } else if ev.key_code == KeyCode::Backspace || matches!(ev.logical_key, Key::Backspace) {
            if input.value.pop().is_some() {
                input_changed = true;
            }
        } else if let Key::Character(ch) = &ev.logical_key {
            let value = ch.as_str();
            if !value.is_empty() && !value.chars().all(char::is_control) {
                let next_len = input.value.chars().count() + value.chars().count();
                if input.max_chars.is_none_or(|max| next_len <= max) {
                    input.value.push_str(value);
                    input_changed = true;
                }
            }
        }

        if input_changed {
            changed.write(MistInputChanged {
                entity,
                value: input.value.clone(),
            });
        }
    }
}

fn sync_input_fields(
    focused: Query<(), With<MistInputFocused>>,
    query: Query<(&MistInputField, &MistInputParts, Entity)>,
    mut texts: Query<&mut Text>,
    mut visibility: Query<&mut Visibility>,
) {
    for (input, parts, entity) in &query {
        if let Ok(mut value_text) = texts.get_mut(parts.value_text) {
            value_text.clear();
            value_text.push_str(&input.value);
        }
        if let Ok(mut placeholder_text) = texts.get_mut(parts.placeholder_text) {
            placeholder_text.clear();
            placeholder_text.push_str(&input.placeholder);
        }
        if let Ok(mut placeholder_visibility) = visibility.get_mut(parts.placeholder_text) {
            *placeholder_visibility = if input.value.is_empty() {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
        if let Ok(mut caret_visibility) = visibility.get_mut(parts.caret) {
            *caret_visibility = if focused.contains(entity) {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
    }
}

fn toggle_dropdowns(
    mut query: Query<
        (&Interaction, &MistDropdownOwner),
        (Changed<Interaction>, With<MistDropdownTrigger>),
    >,
    mut dropdowns: Query<&mut MistDropdown>,
) {
    for (interaction, owner) in &mut query {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Ok(mut dropdown) = dropdowns.get_mut(owner.0) {
            dropdown.open = !dropdown.open;
        }
    }
}

fn select_dropdown_items(
    mut items: Query<
        (&Interaction, &MistDropdownItem, &MistDropdownOwner),
        (Changed<Interaction>, With<Button>),
    >,
    mut dropdowns: Query<(&MistDropdownOptions, &mut MistDropdown)>,
    mut events: MessageWriter<MistDropdownChanged>,
) {
    for (interaction, item, owner) in &mut items {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Ok((options, mut dropdown)) = dropdowns.get_mut(owner.0) {
            dropdown.selected = item.index.min(options.0.len().saturating_sub(1));
            dropdown.open = false;
            let label = options
                .0
                .get(dropdown.selected)
                .cloned()
                .unwrap_or_default();
            events.write(MistDropdownChanged {
                entity: owner.0,
                selected: dropdown.selected,
                label,
            });
        }
    }
}

fn close_dropdowns_on_outside_click(
    mouse: Res<ButtonInput<MouseButton>>,
    mut dropdowns: Query<(Entity, &mut MistDropdown)>,
    related_buttons: Query<
        (&MistDropdownOwner, &Interaction),
        Or<(With<MistDropdownTrigger>, With<MistDropdownItem>)>,
    >,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    for (entity, mut dropdown) in &mut dropdowns {
        if !dropdown.open {
            continue;
        }
        let interacting = related_buttons
            .iter()
            .any(|(owner, interaction)| owner.0 == entity && *interaction != Interaction::None);
        if !interacting {
            dropdown.open = false;
        }
    }
}

fn sync_dropdowns(
    query: Query<
        (&MistDropdown, &MistDropdownOptions, &MistDropdownParts),
        Or<(Changed<MistDropdown>, Added<MistDropdown>)>,
    >,
    mut texts: Query<&mut Text>,
    mut nodes: Query<&mut Node>,
    mut visibility: Query<&mut Visibility>,
) {
    for (dropdown, options, parts) in &query {
        let label = options
            .0
            .get(dropdown.selected)
            .map(String::as_str)
            .unwrap_or("");
        if let Ok(mut text) = texts.get_mut(parts.label_text) {
            text.clear();
            text.push_str(label);
        }
        if let Ok(mut node) = nodes.get_mut(parts.menu) {
            node.display = if dropdown.open {
                Display::Flex
            } else {
                Display::None
            };
        }
        if let Ok(mut menu_visibility) = visibility.get_mut(parts.menu) {
            *menu_visibility = if dropdown.open {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
    }
}

fn select_tabs(
    mut query: Query<
        (&Interaction, &MistTabButton, &MistTabOwner),
        (Changed<Interaction>, With<Button>),
    >,
    mut tabs_query: Query<(&MistTabLabels, &mut MistTabs)>,
    mut events: MessageWriter<MistTabsChanged>,
) {
    for (interaction, button, owner) in &mut query {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Ok((labels, mut tabs)) = tabs_query.get_mut(owner.0) {
            tabs.selected = clamp_choice_index(labels.0.len(), button.index);
            let label = labels.0.get(tabs.selected).cloned().unwrap_or_default();
            events.write(MistTabsChanged {
                entity: owner.0,
                selected: tabs.selected,
                label,
            });
        }
    }
}

fn sync_tabs(
    tabs_query: Query<&MistTabs>,
    buttons: Query<
        (
            Entity,
            &MistTabOwner,
            &MistTabButton,
            &MistTabParts,
            &Interaction,
        ),
        With<Button>,
    >,
    mut backgrounds: Query<&mut BackgroundColor>,
    mut borders: Query<&mut BorderColor>,
    mut text_colors: Query<&mut TextColor>,
    mut surfaces: Query<&mut MistSmokeSurface>,
) {
    for (entity, owner, button, parts, interaction) in &buttons {
        let Ok(tabs) = tabs_query.get(owner.0) else {
            continue;
        };
        let selected = tabs.selected == button.index;

        if let Ok(mut background) = backgrounds.get_mut(entity) {
            background.0 = Color::NONE;
        }
        if let Ok(mut border) = borders.get_mut(entity) {
            *border = BorderColor::all(Color::NONE);
        }
        if let Ok(mut surface) = surfaces.get_mut(entity) {
            *surface = surface_smoke_for_role(
                MistSmokeSurfaceRole::OptionBody,
                selected || *interaction != Interaction::None,
            );
        }
        if let Ok(mut text_color) = text_colors.get_mut(parts.label) {
            text_color.0 = if selected {
                text_primary()
            } else {
                text_secondary()
            };
        }
    }
}

fn dismiss_dialogs(
    mut controls: Query<
        (&Interaction, Option<&MistDialogBackdrop>, &MistDialogOwner),
        (
            Changed<Interaction>,
            Or<(With<MistDialogBackdrop>, With<MistDialogCloseButton>)>,
        ),
    >,
    mut dialogs: Query<&mut MistDialog>,
    mut events: MessageWriter<MistDialogDismissed>,
) {
    for (interaction, backdrop, owner) in &mut controls {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let Ok(mut dialog) = dialogs.get_mut(owner.0) else {
            continue;
        };
        if backdrop.is_some() && !dialog.dismiss_on_backdrop {
            continue;
        }

        if dialog.open {
            dialog.open = false;
            events.write(MistDialogDismissed { entity: owner.0 });
        }
    }
}

fn dismiss_dialogs_on_escape(
    keys: Res<ButtonInput<KeyCode>>,
    mut dialogs: Query<(Entity, &mut MistDialog)>,
    mut events: MessageWriter<MistDialogDismissed>,
) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }

    for (entity, mut dialog) in &mut dialogs {
        if dialog.open {
            dialog.open = false;
            events.write(MistDialogDismissed { entity });
        }
    }
}

fn sync_dialogs(
    query: Query<(&MistDialog, Entity), Or<(Changed<MistDialog>, Added<MistDialog>)>>,
    mut visibility: Query<&mut Visibility>,
    mut nodes: Query<&mut Node>,
) {
    for (dialog, entity) in &query {
        if let Ok(mut dialog_visibility) = visibility.get_mut(entity) {
            *dialog_visibility = if dialog.open {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
        if let Ok(mut dialog_node) = nodes.get_mut(entity) {
            dialog_node.display = if dialog.open {
                Display::Flex
            } else {
                Display::None
            };
        }
    }
}

fn dismiss_toasts(
    mut controls: Query<
        (&Interaction, &MistToastOwner),
        (Changed<Interaction>, With<MistToastCloseButton>),
    >,
    mut toasts: Query<&mut MistToast>,
    mut events: MessageWriter<MistToastDismissed>,
) {
    for (interaction, owner) in &mut controls {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Ok(mut toast) = toasts.get_mut(owner.0) {
            toast.open = false;
            events.write(MistToastDismissed { entity: owner.0 });
        }
    }
}

fn sync_toasts(
    query: Query<(&MistToast, Entity), Or<(Changed<MistToast>, Added<MistToast>)>>,
    mut visibility: Query<&mut Visibility>,
    mut nodes: Query<&mut Node>,
) {
    for (toast, entity) in &query {
        if let Ok(mut toast_visibility) = visibility.get_mut(entity) {
            *toast_visibility = if toast.open {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
        if let Ok(mut toast_node) = nodes.get_mut(entity) {
            toast_node.display = if toast.open {
                Display::Flex
            } else {
                Display::None
            };
        }
    }
}

fn sync_popovers(
    query: Query<(&MistPopover, Entity), Or<(Changed<MistPopover>, Added<MistPopover>)>>,
    mut visibility: Query<&mut Visibility>,
    mut nodes: Query<&mut Node>,
) {
    for (popover, entity) in &query {
        if let Ok(mut popover_visibility) = visibility.get_mut(entity) {
            *popover_visibility = if popover.open {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
        if let Ok(mut popover_node) = nodes.get_mut(entity) {
            popover_node.display = if popover.open {
                Display::Flex
            } else {
                Display::None
            };
        }
    }
}

fn invoke_menu_lists(
    mut items: Query<
        (&Interaction, &MistMenuListItem, &MistMenuListOwner),
        (Changed<Interaction>, With<Button>),
    >,
    roots: Query<&MistMenuListOptions>,
    mut events: MessageWriter<MistMenuAction>,
) {
    for (interaction, item, owner) in &mut items {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Ok(options) = roots.get(owner.0) {
            events.write(MistMenuAction {
                entity: owner.0,
                index: item.index,
                label: options.0.get(item.index).cloned().unwrap_or_default(),
            });
        }
    }
}

fn invoke_context_menu_items(
    mut items: Query<
        (&Interaction, &MistContextMenuItem, &MistContextMenuOwner),
        (Changed<Interaction>, With<Button>),
    >,
    mut roots: Query<(&MistContextMenuOptions, &mut MistContextMenu)>,
    mut events: MessageWriter<MistMenuAction>,
) {
    for (interaction, item, owner) in &mut items {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Ok((options, mut menu)) = roots.get_mut(owner.0) {
            menu.open = false;
            events.write(MistMenuAction {
                entity: owner.0,
                index: item.index,
                label: options.0.get(item.index).cloned().unwrap_or_default(),
            });
        }
    }
}

fn sync_context_menus(
    query: Query<
        (&MistContextMenu, &MistContextMenuParts),
        Or<(Changed<MistContextMenu>, Added<MistContextMenu>)>,
    >,
    mut nodes: Query<&mut Node>,
    mut visibility: Query<&mut Visibility>,
) {
    for (menu, parts) in &query {
        if let Ok(mut node) = nodes.get_mut(parts.menu) {
            node.display = if menu.open {
                Display::Flex
            } else {
                Display::None
            };
        }
        if let Ok(mut menu_visibility) = visibility.get_mut(parts.menu) {
            *menu_visibility = if menu.open {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
    }
}

fn toggle_accordion_sections(
    mut sections: Query<
        (&Interaction, &MistAccordionSection, &MistAccordionOwner),
        (Changed<Interaction>, With<Button>),
    >,
    mut roots: Query<(&MistAccordionSections, &mut MistAccordionState)>,
    mut events: MessageWriter<MistAccordionChanged>,
) {
    for (interaction, section, owner) in &mut sections {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Ok((labels, mut state)) = roots.get_mut(owner.0) {
            if let Some(open) = state.0.get_mut(section.index) {
                *open = !*open;
                events.write(MistAccordionChanged {
                    entity: owner.0,
                    section: section.index,
                    open: *open,
                    label: labels.0.get(section.index).cloned().unwrap_or_default(),
                });
            }
        }
    }
}

fn sync_accordion_sections(
    roots: Query<&MistAccordionState>,
    sections: Query<(
        &MistAccordionOwner,
        &MistAccordionSection,
        &MistAccordionParts,
    )>,
    mut nodes: Query<&mut Node>,
    mut texts: Query<&mut Text>,
) {
    for (owner, section, parts) in &sections {
        let Ok(state) = roots.get(owner.0) else {
            continue;
        };
        let open = state.0.get(section.index).copied().unwrap_or(false);
        if let Ok(mut node) = nodes.get_mut(parts.body) {
            node.display = if open { Display::Flex } else { Display::None };
        }
        if let Ok(mut chevron) = texts.get_mut(parts.chevron) {
            chevron.clear();
            chevron.push_str(if open { "▾" } else { "▸" });
        }
    }
}

fn invoke_segmented_actions(
    mut items: Query<
        (
            &Interaction,
            &MistSegmentedActionButton,
            &MistSegmentedActionOwner,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    roots: Query<&MistSegmentedActionLabels>,
    mut events: MessageWriter<MistSegmentedActionInvoked>,
) {
    for (interaction, button, owner) in &mut items {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Ok(labels) = roots.get(owner.0) {
            events.write(MistSegmentedActionInvoked {
                entity: owner.0,
                selected: button.index,
                label: labels.0.get(button.index).cloned().unwrap_or_default(),
            });
        }
    }
}

fn select_list_items(
    mut pressed_rows: Query<
        (&Interaction, &MistListItem, &MistListOwner),
        (Changed<Interaction>, With<Button>),
    >,
    mut roots: Query<(&MistListItems, &mut MistListView)>,
    rows: Query<(Entity, &MistListItem, &MistListOwner)>,
    mut surfaces: Query<&mut MistSmokeSurface>,
    mut borders: Query<&mut SmokeBorder>,
    mut events: MessageWriter<MistListSelectionChanged>,
) {
    for (interaction, item, owner) in &mut pressed_rows {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Ok((labels, mut list)) = roots.get_mut(owner.0) {
            list.selected = Some(item.index);
            events.write(MistListSelectionChanged {
                entity: owner.0,
                selected: item.index,
                label: labels.0.get(item.index).cloned().unwrap_or_default(),
            });
        }
        for (entity, row_item, row_owner) in &rows {
            if row_owner.0 != owner.0 {
                continue;
            }
            let selected = row_item.index == item.index;
            if let Ok(mut surface) = surfaces.get_mut(entity) {
                *surface = surface_smoke_for_widget_role(MistSmokeRole::DataItem, selected);
            }
            if let Ok(mut border) = borders.get_mut(entity) {
                *border =
                    smoke_border_for_role(MistSmokeRole::DataItem, selected, entity.to_bits());
            }
        }
    }
}

fn select_table_rows(
    mut pressed_rows: Query<
        (&Interaction, &MistTableRowButton, &MistTableOwner),
        (Changed<Interaction>, With<Button>),
    >,
    mut tables: Query<&mut MistTable>,
    mut events: MessageWriter<MistTableRowSelected>,
) {
    for (interaction, row, owner) in &mut pressed_rows {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Ok(mut table) = tables.get_mut(owner.0) {
            table.selected = Some(row.index);
            events.write(MistTableRowSelected {
                entity: owner.0,
                row: row.index,
            });
        }
    }
}

fn request_table_sort(
    mut headers: Query<
        (&Interaction, &MistTableHeaderButton, &MistTableOwner),
        (Changed<Interaction>, With<Button>),
    >,
    tables: Query<&MistTableColumns>,
    mut events: MessageWriter<MistTableSortRequested>,
) {
    for (interaction, header, owner) in &mut headers {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Ok(columns) = tables.get(owner.0) {
            events.write(MistTableSortRequested {
                entity: owner.0,
                column: header.index,
                label: columns.0.get(header.index).cloned().unwrap_or_default(),
            });
        }
    }
}

fn sync_table_rows(
    tables: Query<&MistTable>,
    rows: Query<(Entity, &MistTableRowButton, &MistTableOwner)>,
    mut surfaces: Query<&mut MistSmokeSurface>,
    mut borders: Query<&mut SmokeBorder>,
) {
    for (entity, row, owner) in &rows {
        let Ok(table) = tables.get(owner.0) else {
            continue;
        };
        let selected = table.selected == Some(row.index);
        if let Ok(mut surface) = surfaces.get_mut(entity) {
            *surface = surface_smoke_for_widget_role(MistSmokeRole::DataItem, selected);
        }
        if let Ok(mut border) = borders.get_mut(entity) {
            *border = smoke_border_for_role(MistSmokeRole::DataItem, selected, entity.to_bits());
        }
    }
}

fn select_tree_rows(
    mut pressed_rows: Query<
        (&Interaction, &MistTreeItem, &MistTreeOwner),
        (Changed<Interaction>, With<Button>),
    >,
    mut trees: Query<(&mut MistTreeView, &mut MistTreeNodes)>,
    mut selected_events: MessageWriter<MistTreeNodeSelected>,
    mut toggled_events: MessageWriter<MistTreeNodeToggled>,
) {
    for (interaction, item, owner) in &mut pressed_rows {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Ok((mut tree, mut nodes)) = trees.get_mut(owner.0) {
            tree.selected = Some(item.index);
            let label = nodes
                .0
                .get(item.index)
                .map(|node| node.label.clone())
                .unwrap_or_default();
            selected_events.write(MistTreeNodeSelected {
                entity: owner.0,
                node: item.index,
                label: label.clone(),
            });
            if tree_has_children(&nodes.0, item.index) {
                if let Some(node) = nodes.0.get_mut(item.index) {
                    node.expanded = !node.expanded;
                    toggled_events.write(MistTreeNodeToggled {
                        entity: owner.0,
                        node: item.index,
                        expanded: node.expanded,
                        label,
                    });
                }
            }
        }
    }
}

fn sync_tree_rows(
    trees: Query<(&MistTreeView, &MistTreeNodes)>,
    rows: Query<(Entity, &MistTreeItem, &MistTreeOwner, &MistTreeItemParts)>,
    mut surfaces: Query<&mut MistSmokeSurface>,
    mut borders: Query<&mut SmokeBorder>,
    mut texts: Query<&mut Text>,
    mut nodes_query: Query<&mut Node>,
) {
    for (entity, item, owner, parts) in &rows {
        let Ok((tree, nodes)) = trees.get(owner.0) else {
            continue;
        };
        let selected = tree.selected == Some(item.index);
        if let Ok(mut surface) = surfaces.get_mut(entity) {
            *surface = surface_smoke_for_widget_role(MistSmokeRole::DataItem, selected);
        }
        if let Ok(mut border) = borders.get_mut(entity) {
            *border = smoke_border_for_role(MistSmokeRole::DataItem, selected, entity.to_bits());
        }
        if let Ok(mut node) = nodes_query.get_mut(entity) {
            node.display = if tree_branch_visible(&nodes.0, item.index) {
                Display::Flex
            } else {
                Display::None
            };
        }
        if let Ok(mut toggle) = texts.get_mut(parts.toggle) {
            toggle.clear();
            let value = if tree_has_children(&nodes.0, item.index) {
                if nodes.0.get(item.index).is_some_and(|node| node.expanded) {
                    "▾"
                } else {
                    "▸"
                }
            } else {
                "•"
            };
            toggle.push_str(value);
        }
    }
}

fn select_grid_items(
    mut pressed_items: Query<
        (&Interaction, &MistGridItem, &MistGridOwner),
        (Changed<Interaction>, With<Button>),
    >,
    mut grids: Query<(&MistGridItems, &mut MistGridView)>,
    items: Query<(Entity, &MistGridItem, &MistGridOwner)>,
    mut surfaces: Query<&mut MistSmokeSurface>,
    mut borders: Query<&mut SmokeBorder>,
    mut events: MessageWriter<MistGridItemSelected>,
) {
    for (interaction, item, owner) in &mut pressed_items {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Ok((labels, mut grid)) = grids.get_mut(owner.0) {
            grid.selected = Some(item.index);
            events.write(MistGridItemSelected {
                entity: owner.0,
                selected: item.index,
                label: labels.0.get(item.index).cloned().unwrap_or_default(),
            });
        }
        for (entity, grid_item, grid_owner) in &items {
            if grid_owner.0 != owner.0 {
                continue;
            }
            let selected = grid_item.index == item.index;
            if let Ok(mut surface) = surfaces.get_mut(entity) {
                *surface = surface_smoke_for_widget_role(MistSmokeRole::DataItem, selected);
            }
            if let Ok(mut border) = borders.get_mut(entity) {
                *border =
                    smoke_border_for_role(MistSmokeRole::DataItem, selected, entity.to_bits());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::{
        asset::AssetPlugin,
        ecs::{hierarchy::Children, world::CommandQueue},
    };

    fn subtree_count_with<T: Component>(world: &World, root: Entity) -> usize {
        fn recurse<T: Component>(world: &World, entity: Entity, count: &mut usize) {
            if world.entity(entity).contains::<T>() {
                *count += 1;
            }
            if let Some(children) = world.entity(entity).get::<Children>() {
                for child in children.iter() {
                    recurse::<T>(world, child, count);
                }
            }
        }

        let mut count = 0usize;
        recurse::<T>(world, root, &mut count);
        count
    }

    #[test]
    fn mist_ui_plugin_registers_messages_and_focus_state() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(AssetPlugin::default());
        app.add_plugins(MistUiPlugin);
        assert!(app.world().contains_resource::<MistInputFocusState>());
        assert!(app
            .world()
            .contains_resource::<Messages<MistButtonPressed>>());
        assert!(app
            .world()
            .contains_resource::<Messages<MistRadioChanged>>());
        assert!(app
            .world()
            .contains_resource::<Messages<MistSwitchChanged>>());
        assert!(app
            .world()
            .contains_resource::<Messages<MistDropdownChanged>>());
        assert!(app.world().contains_resource::<Messages<MistTabsChanged>>());
        assert!(app
            .world()
            .contains_resource::<Messages<MistDialogDismissed>>());
    }

    #[test]
    fn mist_ui_plugin_installs_atomic_widget_plugins() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));
        app.add_plugins(MistUiPlugin);

        assert!(app.is_plugin_added::<MistUiMessagesPlugin>());
        assert!(app.is_plugin_added::<MistUiSmokeHostPlugin>());
        assert!(app.is_plugin_added::<MistUiActionPlugin>());
        assert!(app.is_plugin_added::<MistUiSelectionPlugin>());
        assert!(app.is_plugin_added::<MistUiScalarPlugin>());
        assert!(app.is_plugin_added::<MistUiInputPlugin>());
        assert!(app.is_plugin_added::<MistUiOverlayPlugin>());
        assert!(app.is_plugin_added::<MistUiFeedbackPlugin>());
        assert!(app.is_plugin_added::<MistUiDataViewPlugin>());
    }

    #[test]
    fn slider_normalization_is_stable() {
        let slider = MistSlider::new(-1.0, 3.0);
        assert!((slider.normalize(1.0) - 0.5).abs() < 1e-5);
        assert!((slider.denormalize(0.5) - 1.0).abs() < 1e-5);
    }

    #[test]
    fn clamp_choice_index_handles_empty_groups() {
        assert_eq!(clamp_choice_index(0, 3), 0);
        assert_eq!(clamp_choice_index(3, 9), 2);
    }

    #[test]
    fn pressed_smoke_configs_are_denser_than_idle() {
        let idle = smoke_config_for_role(MistSmokeRole::ToolbarButton, false);
        let pressed = smoke_config_for_role(MistSmokeRole::ToolbarButton, true);

        assert!(pressed.thickness >= idle.thickness);
        assert!(pressed.intensity >= idle.intensity);
        assert!(pressed.particle_density >= idle.particle_density);
        assert!(pressed.particle_size_scale >= idle.particle_size_scale);
    }

    #[test]
    fn hovered_smoke_configs_bridge_idle_and_pressed() {
        let idle = smoke_config_for_edge_state(MistSmokeRole::StandardButton, MistEdgeState::Idle);
        let hovered =
            smoke_config_for_edge_state(MistSmokeRole::StandardButton, MistEdgeState::Hovered);
        let pressed =
            smoke_config_for_edge_state(MistSmokeRole::StandardButton, MistEdgeState::Pressed);

        assert!(hovered.thickness >= idle.thickness);
        assert!(hovered.intensity >= idle.intensity);
        assert!(hovered.particle_density >= idle.particle_density);
        assert!(hovered.thickness <= pressed.thickness);
        assert!(hovered.intensity <= pressed.intensity);
        assert!(hovered.particle_density <= pressed.particle_density);
    }

    #[test]
    fn trigger_smoke_is_more_assertive_than_standard_button() {
        let trigger =
            smoke_config_for_edge_state(MistSmokeRole::TriggerButton, MistEdgeState::Idle);
        let button =
            smoke_config_for_edge_state(MistSmokeRole::StandardButton, MistEdgeState::Idle);

        assert!(trigger.intensity > button.intensity);
        assert!(trigger.particle_density > button.particle_density);
        assert!(trigger.thickness >= button.thickness);
    }

    #[test]
    fn smoke_hosts_allow_visible_overflow_for_full_border() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, bevy::input::InputPlugin));
        app.add_plugins(AssetPlugin::default());
        app.add_plugins(MistUiPlugin);

        let entity = app
            .world_mut()
            .spawn((
                MistSmokeRole::StandardButton,
                MistSmokeSurface::new(MistSmokeConfig::default()),
                Node::default(),
            ))
            .id();

        app.update();

        let node = app
            .world()
            .entity(entity)
            .get::<Node>()
            .expect("node exists");
        assert_eq!(node.overflow, Overflow::visible());
        assert!(app.world().entity(entity).contains::<MistSmokeHostReady>());
    }

    #[test]
    fn control_body_surface_smoke_is_visible_at_idle() {
        let idle =
            surface_smoke_for_role_state(MistSmokeSurfaceRole::ControlBody, MistSurfaceState::Idle);
        let active = surface_smoke_for_role_state(
            MistSmokeSurfaceRole::ControlBody,
            MistSurfaceState::Active,
        );

        assert!(idle.config.intensity >= 1.7);
        assert!(idle.config.particle_density >= 0.7);
        assert!(active.config.particle_density > idle.config.particle_density);
        assert!(active.config.particle_size_scale > idle.config.particle_size_scale);
    }

    #[test]
    fn trigger_surface_smoke_is_more_visible_than_standard_button() {
        let trigger =
            surface_smoke_for_widget_state(MistSmokeRole::TriggerButton, MistSurfaceState::Idle);
        let button =
            surface_smoke_for_widget_state(MistSmokeRole::StandardButton, MistSurfaceState::Idle);

        assert!(trigger.config.intensity > button.config.intensity);
        assert!(trigger.config.particle_density > button.config.particle_density);
        assert!(trigger.config.particle_size_scale > button.config.particle_size_scale);
    }

    #[test]
    fn checkbox_indicator_has_its_own_smoke_runtime() {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            bevy::input::InputPlugin,
            AssetPlugin::default(),
        ));
        app.add_plugins(MistUiPlugin);

        let font = Handle::<Font>::default();
        let checkbox = {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, app.world_mut());
            let checkbox = spawn_mist_checkbox(&mut commands, &font, "Track", true);
            queue.apply(app.world_mut());
            checkbox
        };

        app.update();

        let parts = *app
            .world()
            .entity(checkbox)
            .get::<MistCheckboxParts>()
            .expect("checkbox parts should exist");
        assert!(app
            .world()
            .entity(parts.indicator)
            .contains::<MistSmokeRole>());
        assert!(app
            .world()
            .entity(parts.indicator)
            .contains::<SmokeBorder>());
        assert!(app
            .world()
            .entity(parts.indicator)
            .contains::<MistSmokeSurface>());
        let config = *app
            .world()
            .entity(parts.indicator)
            .get::<MistSmokeConfig>()
            .expect("indicator smoke config should exist");
        assert!(app
            .world()
            .entity(parts.indicator)
            .contains::<MistSmokeConfig>());
        assert!(app
            .world()
            .entity(parts.indicator)
            .contains::<MistSmokeTarget>());
        assert!(config.particle_density >= 3.2);
        assert!(config.flow_speed >= 1.2);
    }

    #[test]
    fn checkbox_exposes_state_tag_readout() {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            bevy::input::InputPlugin,
            AssetPlugin::default(),
        ));
        app.add_plugins(MistUiPlugin);

        let font = Handle::<Font>::default();
        let checkbox = {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, app.world_mut());
            let checkbox = spawn_mist_checkbox(&mut commands, &font, "Track", true);
            queue.apply(app.world_mut());
            checkbox
        };

        app.update();

        let parts = *app
            .world()
            .entity(checkbox)
            .get::<MistCheckboxParts>()
            .expect("checkbox parts should exist");
        assert!(app.world().entity(parts.tag).contains::<MistSmokeSurface>());
        let tag_text = app
            .world()
            .entity(parts.tag_text)
            .get::<Text>()
            .expect("state tag text should exist");
        assert_eq!(tag_text.as_str(), "ON");
    }

    #[test]
    fn scroll_view_spawns_smoke_scrollbar_parts() {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            bevy::input::InputPlugin,
            AssetPlugin::default(),
        ));
        app.add_plugins(MistUiPlugin);

        let scroll_view = {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, app.world_mut());
            let (scroll_view, _content) = spawn_mist_scroll_view(&mut commands, 220.0, 160.0);
            queue.apply(app.world_mut());
            scroll_view
        };

        app.update();

        let parts = *app
            .world()
            .entity(scroll_view)
            .get::<MistScrollParts>()
            .expect("scroll parts should exist");
        assert!(app
            .world()
            .entity(parts.track)
            .contains::<MistScrollTrack>());
        assert!(app.world().entity(parts.track).contains::<SmokeBorder>());
        assert!(app
            .world()
            .entity(parts.track)
            .contains::<MistSmokeSurface>());
        assert!(app
            .world()
            .entity(parts.thumb)
            .contains::<MistScrollThumb>());
        assert!(app.world().entity(parts.thumb).contains::<SmokeBorder>());
        assert!(app
            .world()
            .entity(parts.thumb)
            .contains::<MistSmokeSurface>());
    }

    #[test]
    fn slider_spawns_signature_track_and_handle() {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            bevy::input::InputPlugin,
            AssetPlugin::default(),
        ));
        app.add_plugins(MistUiPlugin);

        let slider = {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, app.world_mut());
            let slider = spawn_mist_slider(&mut commands, 220.0, 0.72);
            queue.apply(app.world_mut());
            slider
        };

        app.update();

        let parts = *app
            .world()
            .entity(slider)
            .get::<MistSliderParts>()
            .expect("slider parts should exist");
        assert!(app
            .world()
            .entity(parts.track)
            .contains::<MistSmokeSurface>());
        let handle = app
            .world()
            .entity(parts.handle)
            .get::<Node>()
            .expect("handle node should exist");
        assert_eq!(handle.height, Val::Px(SLIDER_HANDLE_HEIGHT));
        assert_eq!(handle.width, Val::Px(SLIDER_HANDLE_WIDTH));
    }

    #[test]
    fn smoke_widgets_attach_runtime_components_across_public_suite() {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            bevy::input::InputPlugin,
            AssetPlugin::default(),
        ));
        app.add_plugins(MistUiPlugin);

        let font = Handle::<Font>::default();
        let image = Handle::<Image>::default();

        let (
            panel,
            graphic,
            button,
            trigger,
            checkbox,
            radio_group,
            switcher,
            scroll_view,
            slider,
            progress,
            dropdown,
            input,
            tooltip,
            tabs,
            dialog,
            badge,
            chip,
            status_pill,
            toast,
            popover,
            context_menu,
            menu_list,
            accordion,
            segmented,
            list_view,
            table,
            tree,
            grid,
        ) = {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, app.world_mut());

            let panel = spawn_mist_panel(&mut commands);
            let graphic = spawn_mist_image(&mut commands, image.clone(), Vec2::new(64.0, 64.0));
            let button = spawn_mist_button(&mut commands, &font, "Confirm", 220.0);
            let trigger = spawn_mist_trigger(&mut commands, &font, "Open", 180.0);
            let checkbox = spawn_mist_checkbox(&mut commands, &font, "Track", true);
            let radio_group = spawn_mist_radio_group(
                &mut commands,
                &font,
                220.0,
                ["Balanced", "Dense", "Signal"],
                1,
            );
            let switcher = spawn_mist_switch(&mut commands, &font, "Reactive", true);
            let (scroll_view, _) = spawn_mist_scroll_view(&mut commands, 220.0, 160.0);
            let slider = spawn_mist_slider(&mut commands, 220.0, 0.72);
            let progress = spawn_mist_progress_bar(&mut commands, 220.0, 0.46);
            let dropdown = spawn_mist_dropdown(&mut commands, &font, 220.0, ["English", "中文"]);
            let input = spawn_mist_input_field(
                &mut commands,
                &font,
                260.0,
                MistInputField::new("operator@rope.dev").with_value("rope"),
            );
            let tooltip = attach_mist_tooltip(&mut commands, button, &font, "Tooltip body", 240.0);
            let tabs = spawn_mist_tabs(
                &mut commands,
                &font,
                240.0,
                ["Overview", "Nodes", "Logs"],
                0,
            );
            let dialog = spawn_mist_dialog(&mut commands, &font, "Mist", "Body", 420.0);
            let badge = spawn_mist_badge(&mut commands, &font, "READY");
            let chip = spawn_mist_chip(&mut commands, &font, "Dense Mist", 160.0);
            let status_pill = spawn_mist_status_pill(&mut commands, &font, "Stable", true);
            let toast = spawn_mist_toast(&mut commands, &font, "Toast", "Body", 320.0);
            let popover = spawn_mist_popover(&mut commands, &font, "Popover", "Body", 280.0);
            let context_menu = spawn_mist_context_menu(
                &mut commands,
                &font,
                220.0,
                ["Inspect", "Pause", "Retire"],
            );
            let menu_list =
                spawn_mist_menu_list(&mut commands, &font, 220.0, ["Inspect", "Pause", "Retire"]);
            let accordion = spawn_mist_accordion(
                &mut commands,
                &font,
                320.0,
                vec![
                    ("Signal".to_string(), "A".to_string()),
                    ("Density".to_string(), "B".to_string()),
                ],
            );
            let segmented = spawn_mist_segmented_action_row(
                &mut commands,
                &font,
                260.0,
                ["Deploy", "Trace", "Quarantine"],
            );
            let list_view =
                spawn_mist_list_view(&mut commands, &font, 220.0, 160.0, ["A", "B", "C"], Some(1));
            let table = spawn_mist_table(
                &mut commands,
                &font,
                420.0,
                vec!["Node".to_string(), "Load".to_string()],
                vec![
                    vec!["alpha".to_string(), "72%".to_string()],
                    vec!["beta".to_string(), "58%".to_string()],
                ],
            );
            let tree = spawn_mist_tree_view(
                &mut commands,
                &font,
                320.0,
                vec![
                    MistTreeNodeSpec::root("Root"),
                    MistTreeNodeSpec::child("Child", 0),
                ],
                Some(0),
            );
            let grid = spawn_mist_grid_view(
                &mut commands,
                &font,
                320.0,
                2,
                ["Alpha", "Beta", "Gamma", "Delta"],
                Some(2),
            );

            queue.apply(app.world_mut());
            (
                panel,
                graphic,
                button,
                trigger,
                checkbox,
                radio_group,
                switcher,
                scroll_view,
                slider,
                progress,
                dropdown,
                input,
                tooltip,
                tabs,
                dialog,
                badge,
                chip,
                status_pill,
                toast,
                popover,
                context_menu,
                menu_list,
                accordion,
                segmented,
                list_view,
                table,
                tree,
                grid,
            )
        };

        app.update();

        for widget in [
            panel,
            graphic,
            button,
            trigger,
            checkbox,
            radio_group,
            switcher,
            scroll_view,
            slider,
            progress,
            dropdown,
            input,
            tooltip,
            tabs,
            dialog,
            badge,
            chip,
            status_pill,
            toast,
            popover,
            context_menu,
            menu_list,
            accordion,
            segmented,
            list_view,
            table,
            tree,
            grid,
        ] {
            assert!(
                subtree_count_with::<SmokeBorder>(app.world(), widget) >= 1,
                "widget subtree {:?} should expose SmokeBorder",
                widget
            );
            assert!(
                subtree_count_with::<MistSmokeSurface>(app.world(), widget) >= 1,
                "widget subtree {:?} should expose MistSmokeSurface",
                widget
            );
            assert!(
                subtree_count_with::<MistSmokeConfig>(app.world(), widget) >= 1,
                "widget subtree {:?} should expose MistSmokeConfig",
                widget
            );
            assert!(
                subtree_count_with::<MistSmokeTarget>(app.world(), widget) >= 1,
                "widget subtree {:?} should expose MistSmokeTarget",
                widget
            );
        }
    }
}
