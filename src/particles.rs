use crate::{plugin::MistSmokeBackend, SmokeBorder};
use bevy::{
    asset::RenderAssetUsages,
    ecs::hierarchy::ChildOf,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    ui::ComputedNode,
};
use rand::Rng;
use std::collections::HashMap;

const MIST_SMOKE_TEXTURE_SIZE: u32 = 64;
const MIST_SMOKE_MAX_PARTICLE_SIZE_PX: f32 = 34.0;

fn clamp_screen_particle_size(size: Vec2) -> Vec2 {
    Vec2::new(
        size.x.clamp(0.5, MIST_SMOKE_MAX_PARTICLE_SIZE_PX),
        size.y.clamp(0.5, MIST_SMOKE_MAX_PARTICLE_SIZE_PX),
    )
}

fn clamp_with_dynamic_floor(value: f32, preferred_min: f32, size_bound: f32) -> f32 {
    let max_bound = size_bound.max(1.0);
    let min_bound = preferred_min.min(max_bound);
    value.clamp(min_bound, max_bound)
}

fn clamp_offset_to_half_extents(offset: Vec2, half_extents: Vec2) -> Vec2 {
    let safe_half_extents = half_extents.max(Vec2::splat(0.001));
    let mut scale = 1.0_f32;
    if offset.x.abs() > safe_half_extents.x && offset.x.abs() > f32::EPSILON {
        scale = scale.min(safe_half_extents.x / offset.x.abs());
    }
    if offset.y.abs() > safe_half_extents.y && offset.y.abs() > f32::EPSILON {
        scale = scale.min(safe_half_extents.y / offset.y.abs());
    }
    offset * scale.clamp(0.0, 1.0)
}

fn clamp_offset_to_ring_band(
    offset: Vec2,
    outer_half_extents: Vec2,
    inner_keepout_half_extents: Vec2,
) -> Vec2 {
    let clamped_outer = clamp_offset_to_half_extents(offset, outer_half_extents);
    if clamped_outer.x.abs() >= inner_keepout_half_extents.x
        || clamped_outer.y.abs() >= inner_keepout_half_extents.y
    {
        return clamped_outer;
    }

    let push_x = inner_keepout_half_extents.x - clamped_outer.x.abs();
    let push_y = inner_keepout_half_extents.y - clamped_outer.y.abs();
    let mut pushed = clamped_outer;

    if push_x <= push_y {
        pushed.x = inner_keepout_half_extents
            .x
            .copysign(if clamped_outer.x.abs() > 1e-6 {
                clamped_outer.x
            } else {
                1.0
            });
    } else {
        pushed.y = inner_keepout_half_extents
            .y
            .copysign(if clamped_outer.y.abs() > 1e-6 {
                clamped_outer.y
            } else {
                1.0
            });
    }

    clamp_offset_to_half_extents(pushed, outer_half_extents)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Default)]
pub enum MistSmokeDomain {
    #[default]
    ScreenUi,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Default)]
pub enum MistSmokeOverlayMode {
    #[default]
    ParticlesOnly,
    ParticlesPlusRing,
    RingOnly,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Default)]
pub enum MistSmokePlacement {
    #[default]
    BorderOrbit,
    SurfaceCloud,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Default)]
pub enum MistSmokePreset {
    #[default]
    StandardButton,
    PrimaryAction,
    ToolbarButton,
    DropdownOption,
    ScrollbarTrack,
    ScrollbarThumb,
    PanelFrame,
    DialogFrame,
}

#[derive(Component, Debug, Clone, Copy, Reflect, Default)]
pub struct MistSmokeTarget {
    pub domain: MistSmokeDomain,
}

impl MistSmokeTarget {
    pub const fn screen_ui() -> Self {
        Self {
            domain: MistSmokeDomain::ScreenUi,
        }
    }
}

#[derive(Component, Debug, Clone, Copy, Reflect, Default)]
pub struct NoMistSmoke;

#[derive(Component, Debug, Clone, Copy, Reflect, Default)]
pub struct MistSmokeEmitter;

#[derive(Component, Debug, Clone, Copy, Reflect)]
pub struct MistSmokeSurface {
    pub config: MistSmokeConfig,
    pub inset_px: Vec2,
}

impl MistSmokeSurface {
    pub fn new(config: MistSmokeConfig) -> Self {
        Self {
            config,
            inset_px: Vec2::ZERO,
        }
    }

    pub fn with_inset(mut self, horizontal: f32, vertical: f32) -> Self {
        self.inset_px = Vec2::new(horizontal, vertical);
        self
    }
}

#[derive(Component, Debug, Clone, Copy, Reflect)]
pub struct MistSmokeConfig {
    pub domain: MistSmokeDomain,
    pub preset: MistSmokePreset,
    pub overlay_mode: MistSmokeOverlayMode,
    pub thickness: f32,
    pub intensity: f32,
    pub flow_speed: f32,
    pub noise_scale: f32,
    pub softness: f32,
    pub pulse_strength: f32,
    pub particle_density: f32,
    pub particle_size_scale: f32,
}

impl Default for MistSmokeConfig {
    fn default() -> Self {
        Self::screen_preset(MistSmokePreset::StandardButton)
    }
}

impl MistSmokeConfig {
    pub fn from_preset(domain: MistSmokeDomain, preset: MistSmokePreset) -> Self {
        let mut config = match preset {
            MistSmokePreset::StandardButton => Self {
                domain,
                preset,
                overlay_mode: MistSmokeOverlayMode::ParticlesOnly,
                thickness: 0.26,
                intensity: 4.8,
                flow_speed: 0.76,
                noise_scale: 28.0,
                softness: 0.30,
                pulse_strength: 0.16,
                particle_density: 1.0,
                particle_size_scale: 1.0,
            },
            MistSmokePreset::PrimaryAction => Self {
                domain,
                preset,
                overlay_mode: MistSmokeOverlayMode::ParticlesOnly,
                thickness: 0.30,
                intensity: 5.6,
                flow_speed: 0.82,
                noise_scale: 32.0,
                softness: 0.32,
                pulse_strength: 0.22,
                particle_density: 1.10,
                particle_size_scale: 1.02,
            },
            MistSmokePreset::ToolbarButton => Self {
                domain,
                preset,
                overlay_mode: MistSmokeOverlayMode::ParticlesOnly,
                thickness: 0.28,
                intensity: 5.2,
                flow_speed: 0.80,
                noise_scale: 30.0,
                softness: 0.31,
                pulse_strength: 0.20,
                particle_density: 1.04,
                particle_size_scale: 1.0,
            },
            MistSmokePreset::DropdownOption => Self {
                domain,
                preset,
                overlay_mode: MistSmokeOverlayMode::ParticlesOnly,
                thickness: 0.26,
                intensity: 4.9,
                flow_speed: 0.76,
                noise_scale: 28.0,
                softness: 0.30,
                pulse_strength: 0.18,
                particle_density: 0.98,
                particle_size_scale: 0.98,
            },
            MistSmokePreset::ScrollbarTrack => Self {
                domain,
                preset,
                overlay_mode: MistSmokeOverlayMode::ParticlesOnly,
                thickness: 0.22,
                intensity: 4.2,
                flow_speed: 0.74,
                noise_scale: 26.0,
                softness: 0.34,
                pulse_strength: 0.16,
                particle_density: 0.90,
                particle_size_scale: 0.94,
            },
            MistSmokePreset::ScrollbarThumb => Self {
                domain,
                preset,
                overlay_mode: MistSmokeOverlayMode::ParticlesOnly,
                thickness: 0.30,
                intensity: 5.5,
                flow_speed: 0.84,
                noise_scale: 34.0,
                softness: 0.33,
                pulse_strength: 0.22,
                particle_density: 1.08,
                particle_size_scale: 1.04,
            },
            MistSmokePreset::PanelFrame => Self {
                domain,
                preset,
                overlay_mode: MistSmokeOverlayMode::ParticlesOnly,
                thickness: 0.24,
                intensity: 4.6,
                flow_speed: 0.74,
                noise_scale: 26.0,
                softness: 0.36,
                pulse_strength: 0.16,
                particle_density: 0.94,
                particle_size_scale: 0.98,
            },
            MistSmokePreset::DialogFrame => Self {
                domain,
                preset,
                overlay_mode: MistSmokeOverlayMode::ParticlesOnly,
                thickness: 0.27,
                intensity: 5.0,
                flow_speed: 0.76,
                noise_scale: 28.0,
                softness: 0.34,
                pulse_strength: 0.18,
                particle_density: 1.00,
                particle_size_scale: 1.0,
            },
        };

        config.thickness = (config.thickness * 0.46).clamp(0.10, 0.18);
        config.intensity = (config.intensity * 0.85).clamp(2.8, 4.4);
        config.flow_speed = (config.flow_speed * 0.95).clamp(0.3, 1.4);
        config.noise_scale = (config.noise_scale + 10.0).clamp(24.0, 64.0);
        config.softness = (config.softness + 0.06).clamp(0.24, 0.46);
        config.pulse_strength = (config.pulse_strength * 0.82).clamp(0.10, 0.22);
        config.particle_density = (config.particle_density * 1.6).clamp(0.8, 2.0);
        config.particle_size_scale = (config.particle_size_scale * 1.25).clamp(0.9, 1.4);
        config.overlay_mode = MistSmokeOverlayMode::ParticlesOnly;
        config
    }

    pub fn screen_preset(preset: MistSmokePreset) -> Self {
        Self::from_preset(MistSmokeDomain::ScreenUi, preset)
    }

    pub fn with_overlay_mode(mut self, overlay_mode: MistSmokeOverlayMode) -> Self {
        self.overlay_mode = overlay_mode;
        self
    }

    pub fn with_thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn with_intensity(mut self, intensity: f32) -> Self {
        self.intensity = intensity;
        self
    }

    pub fn with_flow_speed(mut self, flow_speed: f32) -> Self {
        self.flow_speed = flow_speed;
        self
    }

    pub fn with_noise_scale(mut self, noise_scale: f32) -> Self {
        self.noise_scale = noise_scale;
        self
    }

    pub fn with_softness(mut self, softness: f32) -> Self {
        self.softness = softness;
        self
    }

    pub fn with_pulse_strength(mut self, pulse_strength: f32) -> Self {
        self.pulse_strength = pulse_strength;
        self
    }

    pub fn with_particle_density(mut self, particle_density: f32) -> Self {
        self.particle_density = particle_density;
        self
    }

    pub fn with_particle_size_scale(mut self, particle_size_scale: f32) -> Self {
        self.particle_size_scale = particle_size_scale;
        self
    }

    pub fn supports_particles(self) -> bool {
        !matches!(self.overlay_mode, MistSmokeOverlayMode::RingOnly)
    }

    pub fn supports_ring(self) -> bool {
        !matches!(self.overlay_mode, MistSmokeOverlayMode::ParticlesOnly)
    }
}

pub fn derived_screen_ring(config: MistSmokeConfig) -> SmokeBorder {
    SmokeBorder {
        color: Color::srgb(0.97, 0.99, 1.0),
        pulse_color: Color::srgb(0.72, 0.90, 1.0),
        thickness: config.thickness.clamp(0.10, 0.24),
        intensity: config.intensity.clamp(1.6, 4.6),
        flow_speed: config.flow_speed.clamp(0.1, 2.0),
        noise_scale: config.noise_scale.clamp(12.0, 84.0),
        softness: config.softness.clamp(0.12, 0.6),
        pulse_strength: config.pulse_strength.clamp(0.08, 0.75),
    }
}

#[derive(Resource, Clone, Debug)]
pub struct MistSmokeBudget {
    pub max_live_global: usize,
    pub max_live_per_emitter: usize,
    pub max_spawn_global_per_frame: usize,
    pub max_spawn_per_emitter_per_tick: usize,
    pub overload_avg_frame_ms: f32,
    pub overload_duration_secs: f32,
    pub min_spawn_scale: f32,
    pub min_lifetime_scale: f32,
    pub degrade_rate: f32,
    pub recovery_rate: f32,
}

impl Default for MistSmokeBudget {
    fn default() -> Self {
        Self {
            max_live_global: 1_800,
            max_live_per_emitter: 96,
            max_spawn_global_per_frame: 120,
            max_spawn_per_emitter_per_tick: 10,
            overload_avg_frame_ms: 16.8,
            overload_duration_secs: 0.8,
            min_spawn_scale: 0.35,
            min_lifetime_scale: 0.58,
            degrade_rate: 1.10,
            recovery_rate: 0.40,
        }
    }
}

#[derive(Resource, Clone, Debug)]
struct MistSmokeAdaptiveState {
    avg_dt_secs: f32,
    overload_secs: f32,
    spawn_scale: f32,
    lifetime_scale: f32,
}

impl Default for MistSmokeAdaptiveState {
    fn default() -> Self {
        Self {
            avg_dt_secs: 1.0 / 60.0,
            overload_secs: 0.0,
            spawn_scale: 1.0,
            lifetime_scale: 1.0,
        }
    }
}

#[derive(Resource, Clone)]
struct MistSmokeSprite {
    handle: Handle<Image>,
}

#[derive(Component, Debug, Clone, Copy)]
struct MistSmokeSurfaceShell {
    emitter: Entity,
}

#[derive(Resource, Default)]
struct MistSmokeParticlePool {
    free: Vec<Entity>,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MistSmokeRuntimeSet;

#[derive(Component)]
struct MistSmokeEmitterState {
    timer: Timer,
}

#[derive(Component)]
struct InactiveMistSmokeParticle;

#[derive(Component, Clone, Debug)]
pub struct MistSmokeParticle {
    pub emitter: Entity,
    pub domain: MistSmokeDomain,
    pub placement: MistSmokePlacement,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub local_offset: Vec3,
    pub outer_half_extents: Vec2,
    pub inner_keepout_half_extents: Vec2,
    pub age_secs: f32,
    pub lifetime_secs: f32,
    pub rotation_speed: f32,
    pub start_size: Vec2,
    pub end_size: Vec2,
    pub start_color: LinearRgba,
    pub end_color: LinearRgba,
    pub follow_emitter: bool,
}

pub struct SmokeParticlesPlugin;

impl Plugin for SmokeParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MistSmokeDomain>()
            .register_type::<MistSmokeOverlayMode>()
            .register_type::<MistSmokePlacement>()
            .register_type::<MistSmokePreset>()
            .register_type::<MistSmokeTarget>()
            .register_type::<MistSmokeSurface>()
            .register_type::<MistSmokeEmitter>()
            .register_type::<MistSmokeConfig>()
            .configure_sets(Update, MistSmokeRuntimeSet)
            .init_resource::<MistSmokeBudget>()
            .init_resource::<MistSmokeAdaptiveState>()
            .init_resource::<MistSmokeParticlePool>()
            .add_systems(Startup, init_mist_smoke_sprite_texture)
            .add_systems(
                Update,
                (
                    sync_mist_smoke_surfaces,
                    sync_mist_smoke_emitter_state,
                    update_mist_smoke_adaptive_budget,
                    spawn_mist_smoke_particles
                        .after(sync_mist_smoke_surfaces)
                        .after(sync_mist_smoke_emitter_state)
                        .after(update_mist_smoke_adaptive_budget),
                    update_mist_smoke_particles.after(spawn_mist_smoke_particles),
                )
                    .in_set(MistSmokeRuntimeSet),
            );
    }
}

fn sync_mist_smoke_surfaces(
    mut commands: Commands,
    parents: Query<
        (Entity, Option<&MistSmokeSurface>, Option<&MistSmokeSurfaceShell>),
        Or<(With<MistSmokeSurface>, With<MistSmokeSurfaceShell>)>,
    >,
    mut emitters: Query<
        (&mut MistSmokeConfig, &mut MistSmokeTarget, &mut MistSmokePlacement, &mut Node),
        Without<NoMistSmoke>,
    >,
) {
    for (entity, surface, shell) in &parents {
        let Some(surface) = surface else {
            if let Some(shell) = shell {
                if let Ok(mut emitter) = commands.get_entity(shell.emitter) {
                    emitter.despawn();
                }
                commands.entity(entity).remove::<MistSmokeSurfaceShell>();
            }
            continue;
        };

        let surface_node = Node {
            position_type: PositionType::Absolute,
            left: Val::Px(surface.inset_px.x),
            right: Val::Px(surface.inset_px.x),
            top: Val::Px(surface.inset_px.y),
            bottom: Val::Px(surface.inset_px.y),
            ..default()
        };

        if let Some(shell) = shell {
            if let Ok((mut config, mut target, mut placement, mut node)) =
                emitters.get_mut(shell.emitter)
            {
                *config = surface.config;
                *target = MistSmokeTarget::screen_ui();
                *placement = MistSmokePlacement::SurfaceCloud;
                *node = surface_node;
                continue;
            }

            commands.entity(entity).remove::<MistSmokeSurfaceShell>();
        }

        let emitter = commands.spawn((
            Name::new("Mist Smoke Surface"),
            MistSmokePlacement::SurfaceCloud,
            MistSmokeTarget::screen_ui(),
            surface.config,
            surface_node,
            BackgroundColor(Color::NONE),
            BorderColor::all(Color::NONE),
            Visibility::Inherited,
        ));
        let emitter = emitter.id();
        commands.entity(entity).add_child(emitter);
        commands
            .entity(entity)
            .insert(MistSmokeSurfaceShell { emitter });
    }
}

fn init_mist_smoke_sprite_texture(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    existing: Option<Res<MistSmokeSprite>>,
) {
    if existing.is_some() {
        return;
    }

    let size = MIST_SMOKE_TEXTURE_SIZE.max(8);
    let mut pixels = vec![0u8; (size * size * 4) as usize];
    let center = (size.saturating_sub(1) as f32) * 0.5;
    let inv_radius = 1.0 / center.max(1.0);

    for y in 0..size {
        for x in 0..size {
            let dx = (x as f32 - center) * inv_radius;
            let dy = (y as f32 - center) * inv_radius;
            let radius = (dx * dx + dy * dy).sqrt();
            let angle = dy.atan2(dx);
            let falloff = (1.0 - radius).clamp(0.0, 1.0);
            let lobe_noise = 0.76
                + (angle * 3.0 + radius * 7.5).sin() * 0.18
                + (angle * 5.0 - radius * 11.0).cos() * 0.13
                + (angle * 9.0 + radius * 5.0).sin() * 0.07;
            let edge_breakup = (1.0 - (radius - 0.68).max(0.0) / 0.32).clamp(0.0, 1.0);
            let core_hollow = (radius / 0.18).clamp(0.0, 1.0);
            let alpha = (falloff.powf(1.55)
                * lobe_noise.clamp(0.35, 1.18)
                * edge_breakup.powf(0.9)
                * (0.72 + core_hollow * 0.28)
                * 255.0)
                .round()
                .clamp(0.0, 255.0) as u8;
            let idx = ((y * size + x) * 4) as usize;
            pixels[idx] = 255;
            pixels[idx + 1] = 255;
            pixels[idx + 2] = 255;
            pixels[idx + 3] = alpha;
        }
    }

    let handle = images.add(Image::new(
        Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        pixels,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    ));

    commands.insert_resource(MistSmokeSprite { handle });
}

fn update_mist_smoke_adaptive_budget(
    time: Res<Time>,
    budget: Res<MistSmokeBudget>,
    mut adaptive: ResMut<MistSmokeAdaptiveState>,
) {
    let dt = time.delta_secs().max(0.0);
    if dt <= f32::EPSILON {
        return;
    }

    let smoothing_window = budget.overload_duration_secs.max(0.1);
    let alpha = (dt / smoothing_window).clamp(0.0, 1.0);
    adaptive.avg_dt_secs += (dt - adaptive.avg_dt_secs) * alpha;

    let overload_threshold = budget.overload_avg_frame_ms.max(1.0) * 0.001;
    if adaptive.avg_dt_secs > overload_threshold {
        adaptive.overload_secs = (adaptive.overload_secs + dt).min(smoothing_window * 2.0);
    } else {
        adaptive.overload_secs = (adaptive.overload_secs - dt).max(0.0);
    }

    let overloaded = adaptive.overload_secs >= smoothing_window;
    if overloaded {
        if adaptive.spawn_scale > budget.min_spawn_scale + 1e-3 {
            adaptive.spawn_scale =
                (adaptive.spawn_scale - budget.degrade_rate * dt).max(budget.min_spawn_scale);
        } else {
            adaptive.lifetime_scale = (adaptive.lifetime_scale - budget.degrade_rate * 0.6 * dt)
                .max(budget.min_lifetime_scale);
        }
    } else if adaptive.lifetime_scale < 1.0 - 1e-3 {
        adaptive.lifetime_scale =
            (adaptive.lifetime_scale + budget.recovery_rate * 0.65 * dt).min(1.0);
    } else {
        adaptive.spawn_scale = (adaptive.spawn_scale + budget.recovery_rate * dt).min(1.0);
    }
}

fn adaptive_spawn_scale(adaptive: &MistSmokeAdaptiveState, budget: &MistSmokeBudget) -> f32 {
    adaptive
        .spawn_scale
        .clamp(budget.min_spawn_scale.clamp(0.05, 1.0), 1.0)
}

fn adaptive_lifetime_scale(adaptive: &MistSmokeAdaptiveState, budget: &MistSmokeBudget) -> f32 {
    adaptive
        .lifetime_scale
        .clamp(budget.min_lifetime_scale.clamp(0.1, 1.0), 1.0)
}

fn entity_and_ancestors_visible(
    entity: Entity,
    visibility_chain: &Query<(
        Option<&ChildOf>,
        Option<&Visibility>,
        Option<&InheritedVisibility>,
    )>,
) -> bool {
    let mut current = Some(entity);
    while let Some(current_entity) = current {
        let Ok((child_of, visibility, inherited_visibility)) = visibility_chain.get(current_entity)
        else {
            break;
        };
        if visibility.is_some_and(|visibility| *visibility == Visibility::Hidden) {
            return false;
        }
        if inherited_visibility.is_some_and(|visibility| !visibility.get()) {
            return false;
        }
        current = child_of.map(|child_of| child_of.parent());
    }
    true
}

fn emitter_spawn_interval(config: MistSmokeConfig, size_hint: f32) -> f32 {
    let hz = (17.0
        + config.intensity.clamp(1.0, 7.0) * 2.9
        + config.particle_density.clamp(0.4, 3.2) * 4.8
        + size_hint * 0.020)
        .clamp(17.0, 48.0);
    (1.0 / hz).max(1.0 / 120.0)
}

#[derive(Clone, Copy, Debug)]
struct ScreenSmokeClusterPattern {
    cluster_count: usize,
    particles_per_cluster: usize,
    tangential_spread_px: f32,
    radial_spread_px: f32,
    puff_spread_px: f32,
    orbit_speed_px: f32,
}

fn screen_smoke_cluster_pattern(size: Vec2, config: MistSmokeConfig) -> ScreenSmokeClusterPattern {
    let perimeter_scale = ((size.x + size.y) / 220.0).clamp(0.9, 2.1);
    let (base_clusters, particles_per_cluster, orbit_speed_px) = match config.preset {
        MistSmokePreset::StandardButton => (14.0, 8, 24.0),
        MistSmokePreset::PrimaryAction => (16.0, 9, 26.0),
        MistSmokePreset::ToolbarButton => (13.0, 8, 25.0),
        MistSmokePreset::DropdownOption => (13.0, 7, 23.0),
        MistSmokePreset::ScrollbarTrack => (10.0, 6, 20.0),
        MistSmokePreset::ScrollbarThumb => (11.0, 7, 22.0),
        MistSmokePreset::PanelFrame => (15.0, 8, 21.0),
        MistSmokePreset::DialogFrame => (16.0, 9, 22.0),
    };

    let cluster_count = ((base_clusters * perimeter_scale).round() as usize).clamp(12, 30);
    let density_scale = config.particle_density.clamp(0.8, 3.0);

    ScreenSmokeClusterPattern {
        cluster_count,
        particles_per_cluster: (particles_per_cluster as f32 + density_scale * 0.7).round() as usize,
        tangential_spread_px: (1.8 + density_scale * 1.2 + config.softness * 2.6).clamp(1.8, 6.0),
        radial_spread_px: (1.7 + config.softness * 3.4 + config.thickness * 8.8).clamp(1.6, 6.8),
        puff_spread_px: (0.8 + config.pulse_strength * 4.8).clamp(0.8, 3.4),
        orbit_speed_px: orbit_speed_px * config.flow_speed.clamp(0.5, 1.8),
    }
}

#[derive(Clone, Copy, Debug)]
struct SurfaceSmokeClusterPattern {
    cluster_count: usize,
    particles_per_cluster: usize,
    orbital_extent: Vec2,
    tangential_spread_px: f32,
    radial_spread_px: f32,
    puff_spread_px: f32,
    swirl_speed_px: f32,
}

fn surface_smoke_cluster_pattern(
    size: Vec2,
    config: MistSmokeConfig,
    placement: MistSmokePlacement,
) -> SurfaceSmokeClusterPattern {
    let body_scale = ((size.x * size.y).sqrt() / 180.0).clamp(0.8, 2.4);
    let (base_clusters, particles_per_cluster, orbital_scale, swirl_speed_px) =
        match (config.preset, placement) {
            (MistSmokePreset::PanelFrame, _) => (6.0, 4, 0.34, 16.0),
            (MistSmokePreset::DialogFrame, _) => (7.0, 4, 0.34, 17.0),
            (MistSmokePreset::DropdownOption, _) => (5.0, 4, 0.28, 15.0),
            (MistSmokePreset::ScrollbarTrack, _) => (4.0, 3, 0.24, 13.0),
            (MistSmokePreset::ScrollbarThumb, _) => (5.0, 4, 0.22, 15.0),
            (MistSmokePreset::PrimaryAction, _) => (6.0, 4, 0.25, 16.0),
            (MistSmokePreset::ToolbarButton, _) => (5.0, 4, 0.24, 15.0),
            (MistSmokePreset::StandardButton, _) => (5.0, 4, 0.24, 14.0),
        };

    let cluster_count = ((base_clusters * body_scale).round() as usize).clamp(5, 14);
    let density_scale = config.particle_density.clamp(0.8, 3.0);
    let orbital_extent = Vec2::new(
        clamp_with_dynamic_floor(size.x * orbital_scale, 12.0, size.x * 0.42),
        clamp_with_dynamic_floor(size.y * orbital_scale, 10.0, size.y * 0.40),
    );

    SurfaceSmokeClusterPattern {
        cluster_count,
        particles_per_cluster: (particles_per_cluster as f32 + density_scale * 0.6).round() as usize,
        orbital_extent,
        tangential_spread_px: (2.8 + density_scale * 2.1 + config.softness * 4.0).clamp(2.6, 8.8),
        radial_spread_px: (1.8 + config.softness * 3.1 + config.thickness * 6.6).clamp(1.6, 5.8),
        puff_spread_px: (1.0 + config.pulse_strength * 5.2).clamp(1.0, 4.0),
        swirl_speed_px: swirl_speed_px * config.flow_speed.clamp(0.45, 1.8),
    }
}

fn screen_border_orbit_band_extents(
    size: Vec2,
    config: MistSmokeConfig,
    pattern: ScreenSmokeClusterPattern,
) -> (Vec2, Vec2) {
    let half = size * 0.5;
    let outer_margin = (8.0
        + pattern.radial_spread_px
        + pattern.puff_spread_px * 0.45
        + config.softness * 2.6
        + config.particle_size_scale * 1.8)
        .clamp(10.0, 18.0);
    let band_width = (7.0
        + pattern.radial_spread_px
        + pattern.puff_spread_px * 0.5
        + config.thickness * 13.0
        + config.softness * 2.6
        + config.particle_density * 1.4)
        .clamp(7.0, 14.5);
    let inner_margin = (outer_margin - band_width).clamp(1.0, outer_margin - 1.0);
    let outer_half_extents = half + Vec2::splat(outer_margin);
    let inner_keepout_half_extents = (half + Vec2::splat(inner_margin))
        .min(outer_half_extents - Vec2::splat(2.0))
        .max(Vec2::ZERO);
    (outer_half_extents, inner_keepout_half_extents)
}

fn screen_surface_cloud_outer_extents(
    pattern: SurfaceSmokeClusterPattern,
    config: MistSmokeConfig,
) -> Vec2 {
    pattern.orbital_extent
        + Vec2::splat(
            (pattern.radial_spread_px
                + pattern.puff_spread_px
                + config.softness * 7.0
                + config.particle_size_scale * 2.4)
                .clamp(6.0, 18.0),
        )
}

fn border_alpha_peak_range(preset: MistSmokePreset) -> (f32, f32) {
    match preset {
        MistSmokePreset::StandardButton => (0.66, 0.86),
        MistSmokePreset::PrimaryAction => (0.72, 0.92),
        MistSmokePreset::ToolbarButton => (0.68, 0.88),
        MistSmokePreset::DropdownOption => (0.60, 0.80),
        MistSmokePreset::ScrollbarTrack => (0.54, 0.74),
        MistSmokePreset::ScrollbarThumb => (0.64, 0.84),
        MistSmokePreset::PanelFrame => (0.64, 0.86),
        MistSmokePreset::DialogFrame => (0.70, 0.90),
    }
}

fn surface_alpha_peak_range(preset: MistSmokePreset) -> (f32, f32) {
    match preset {
        MistSmokePreset::StandardButton => (0.09, 0.18),
        MistSmokePreset::PrimaryAction => (0.14, 0.26),
        MistSmokePreset::ToolbarButton => (0.10, 0.20),
        MistSmokePreset::DropdownOption => (0.08, 0.16),
        MistSmokePreset::ScrollbarTrack => (0.10, 0.20),
        MistSmokePreset::ScrollbarThumb => (0.14, 0.26),
        MistSmokePreset::PanelFrame => (0.10, 0.20),
        MistSmokePreset::DialogFrame => (0.12, 0.22),
    }
}

fn sync_mist_smoke_emitter_state(
    mut commands: Commands,
    backend: Option<Res<MistSmokeBackend>>,
    emitters: Query<
        (
            Entity,
            &MistSmokeConfig,
            &MistSmokeTarget,
            Option<&Node>,
            Option<&ComputedNode>,
            Option<&MistSmokeEmitterState>,
        ),
        Without<NoMistSmoke>,
    >,
) {
    let particles_enabled = !matches!(backend.as_deref(), Some(MistSmokeBackend::ShaderRing));

    for (entity, config, target, node, computed, state) in &emitters {
        if !particles_enabled || config.domain != target.domain || !config.supports_particles() {
            let mut entity_commands = commands.entity(entity);
            entity_commands.remove::<MistSmokeEmitter>();
            if state.is_some() {
                entity_commands.remove::<MistSmokeEmitterState>();
            }
            continue;
        }

        let Some(node) = node else {
            let mut entity_commands = commands.entity(entity);
            entity_commands.remove::<MistSmokeEmitter>();
            if state.is_some() {
                entity_commands.remove::<MistSmokeEmitterState>();
            }
            continue;
        };
        if node.display == Display::None {
            continue;
        }

        let size_hint = computed
            .map(|computed| computed.size())
            .map(|size| (size.x + size.y) * 0.5)
            .unwrap_or(64.0);
        let interval = emitter_spawn_interval(*config, size_hint);

        if let Some(existing) = state {
            if (existing.timer.duration().as_secs_f32() - interval).abs() > 1e-4 {
                commands.entity(entity).insert((
                    MistSmokeEmitter,
                    MistSmokeEmitterState {
                        timer: Timer::from_seconds(interval, TimerMode::Repeating),
                    },
                ));
            } else {
                commands.entity(entity).insert(MistSmokeEmitter);
            }
        } else {
            commands.entity(entity).insert((
                MistSmokeEmitter,
                MistSmokeEmitterState {
                    timer: Timer::from_seconds(interval, TimerMode::Repeating),
                },
            ));
        }
    }
}

#[derive(Default)]
struct MistSmokeSpawnScratch {
    per_emitter_live: HashMap<Entity, usize>,
}

struct MistSmokeSpawnBundle {
    image: Handle<Image>,
    size: Vec2,
    local_offset: Vec2,
    color: Color,
    particle: MistSmokeParticle,
}

fn screen_ui_particle_node(emitter_size: Vec2, local_offset: Vec2, particle_size: Vec2) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left: Val::Px(emitter_size.x * 0.5 + local_offset.x - particle_size.x * 0.5),
        top: Val::Px(emitter_size.y * 0.5 - local_offset.y - particle_size.y * 0.5),
        width: Val::Px(particle_size.x),
        height: Val::Px(particle_size.y),
        ..default()
    }
}

fn activate_mist_smoke_particle(
    commands: &mut Commands,
    pool: &mut MistSmokeParticlePool,
    emitter: Entity,
    emitter_size: Vec2,
    bundle: MistSmokeSpawnBundle,
) {
    if let Some(entity) = pool.free.pop() {
        let mut entity_commands = commands.entity(entity);
        entity_commands
            .remove::<InactiveMistSmokeParticle>()
            .insert((
                ImageNode {
                    image: bundle.image,
                    color: bundle.color,
                    ..default()
                },
                screen_ui_particle_node(emitter_size, bundle.local_offset, bundle.size),
                Visibility::Inherited,
                GlobalZIndex(40),
                bundle.particle,
                Name::new("MistSmokeParticle"),
            ));
        commands.entity(emitter).add_child(entity);
    } else {
        let particle_entity = commands
            .spawn((
                ImageNode {
                    image: bundle.image,
                    color: bundle.color,
                    ..default()
                },
                screen_ui_particle_node(emitter_size, bundle.local_offset, bundle.size),
                Visibility::Inherited,
                GlobalZIndex(40),
                bundle.particle,
                Name::new("MistSmokeParticle"),
            ))
            .id();
        commands.entity(emitter).add_child(particle_entity);
    }
}

fn recycle_mist_smoke_particle(
    commands: &mut Commands,
    pool: &mut MistSmokeParticlePool,
    entity: Entity,
) {
    pool.free.push(entity);
    commands
        .entity(entity)
        .remove::<MistSmokeParticle>()
        .remove::<ImageNode>()
        .insert((
            InactiveMistSmokeParticle,
            Visibility::Hidden,
            Name::new("MistSmokeParticlePoolSlot"),
        ));
}

#[allow(clippy::type_complexity)]
fn spawn_mist_smoke_particles(
    mut commands: Commands,
    time: Res<Time>,
    budget: Res<MistSmokeBudget>,
    adaptive: Res<MistSmokeAdaptiveState>,
    backend: Option<Res<MistSmokeBackend>>,
    sprite: Res<MistSmokeSprite>,
    mut pool: ResMut<MistSmokeParticlePool>,
    live_particles: Query<&MistSmokeParticle>,
    visibility_chain: Query<(
        Option<&ChildOf>,
        Option<&Visibility>,
        Option<&InheritedVisibility>,
    )>,
    mut emitters: Query<
        (
            Entity,
            &MistSmokeConfig,
            &MistSmokeTarget,
            Option<&Node>,
            Option<&ComputedNode>,
            Option<&Visibility>,
            Option<&InheritedVisibility>,
            Option<&MistSmokePlacement>,
            Option<&mut MistSmokeEmitterState>,
        ),
        (Without<NoMistSmoke>, Without<MistSmokeParticle>),
    >,
    mut scratch: Local<MistSmokeSpawnScratch>,
) {
    if matches!(backend.as_deref(), Some(MistSmokeBackend::ShaderRing)) {
        return;
    }

    scratch.per_emitter_live.clear();
    let mut live_global = 0usize;
    for particle in &live_particles {
        live_global += 1;
        *scratch
            .per_emitter_live
            .entry(particle.emitter)
            .or_insert(0usize) += 1;
    }

    let mut remaining_live_global = budget.max_live_global.saturating_sub(live_global);
    if remaining_live_global == 0 {
        return;
    }

    let spawn_scale = adaptive_spawn_scale(&adaptive, &budget);
    let lifetime_scale = adaptive_lifetime_scale(&adaptive, &budget);
    let mut remaining_spawn_global =
        ((budget.max_spawn_global_per_frame.max(1) as f32) * spawn_scale).round() as usize;
    remaining_spawn_global = remaining_spawn_global
        .max(1)
        .min(budget.max_spawn_global_per_frame.max(1));
    let mut per_emitter_spawn_limit =
        ((budget.max_spawn_per_emitter_per_tick.max(1) as f32) * spawn_scale).round() as usize;
    per_emitter_spawn_limit = per_emitter_spawn_limit
        .max(1)
        .min(budget.max_spawn_per_emitter_per_tick.max(1));

    let mut rng = rand::rng();
    let elapsed = time.elapsed_secs();

    for (
        entity,
        config,
        target,
        node,
        computed,
        visibility,
        inherited_visibility,
        placement,
        state,
    ) in &mut emitters
    {
        if remaining_live_global == 0 || remaining_spawn_global == 0 {
            break;
        }
        if config.domain != target.domain || !config.supports_particles() {
            continue;
        }
        if !entity_and_ancestors_visible(entity, &visibility_chain) {
            continue;
        }
        if visibility.is_some_and(|visibility| *visibility == Visibility::Hidden) {
            continue;
        }
        if inherited_visibility.is_some_and(|vis| !vis.get()) {
            continue;
        }

        let Some(node) = node else {
            continue;
        };
        if node.display == Display::None {
            continue;
        }
        let Some(computed) = computed else {
            continue;
        };
        let Some(mut timer_state) = state else {
            continue;
        };

        timer_state.timer.tick(time.delta());
        if !timer_state.timer.just_finished() {
            continue;
        }

        let emitter_live = *scratch.per_emitter_live.get(&entity).unwrap_or(&0usize);
        if emitter_live >= budget.max_live_per_emitter {
            continue;
        }

        let size = computed.size();
        if !(size.x.is_finite() && size.y.is_finite() && size.x > 1.0 && size.y > 1.0) {
            continue;
        }

        let base_color = derived_screen_ring(*config).color;
        let start_hsla: bevy::color::Hsla = base_color.into();
        let placement = placement
            .copied()
            .unwrap_or(MistSmokePlacement::BorderOrbit);
        let cluster_pattern = screen_smoke_cluster_pattern(size, *config);
        let surface_pattern = surface_smoke_cluster_pattern(size, *config, placement);
        let (outer_half_extents, inner_keepout_half_extents) = match placement {
            MistSmokePlacement::BorderOrbit => {
                screen_border_orbit_band_extents(size, *config, cluster_pattern)
            }
            MistSmokePlacement::SurfaceCloud => (
                screen_surface_cloud_outer_extents(surface_pattern, *config),
                Vec2::ZERO,
            ),
        };
        let requested_count = match placement {
            MistSmokePlacement::BorderOrbit => {
                (cluster_pattern.cluster_count * cluster_pattern.particles_per_cluster).max(3)
            }
            MistSmokePlacement::SurfaceCloud => {
                (surface_pattern.cluster_count * surface_pattern.particles_per_cluster).max(4)
            }
        };
        let mut count = requested_count;
        let emitter_capacity = budget.max_live_per_emitter.saturating_sub(emitter_live);
        count = count
            .min(per_emitter_spawn_limit)
            .min(28)
            .min(emitter_capacity)
            .min(remaining_live_global)
            .min(remaining_spawn_global);
        if count == 0 {
            continue;
        }

        let cluster_count = match placement {
            MistSmokePlacement::BorderOrbit => cluster_pattern.cluster_count,
            MistSmokePlacement::SurfaceCloud => surface_pattern.cluster_count,
        }
        .min(count.max(1));
        let swirl_direction = if (entity.to_bits() >> 2) & 1 == 0 {
            1.0_f32
        } else {
            -1.0_f32
        };
        let base_orbit_phase = elapsed * config.flow_speed.clamp(0.4, 2.0) * 0.85 * swirl_direction
            + (entity.to_bits() as f32 * 0.000_071_3).fract() * std::f32::consts::TAU;
        let mut spawned = 0usize;
        for particle_index in 0..count {
            if spawned >= per_emitter_spawn_limit
                || remaining_live_global == 0
                || remaining_spawn_global == 0
            {
                break;
            }

            let cluster_index = particle_index % cluster_count;
            let cluster_angle = base_orbit_phase
                + cluster_index as f32 * std::f32::consts::TAU / cluster_count as f32;

            let (spawn_offset, velocity, lifetime, base_size_px) = match placement {
                MistSmokePlacement::BorderOrbit => {
                    let ring_half_extents = inner_keepout_half_extents.lerp(outer_half_extents, 0.58);
                    let cluster_breath = 1.0
                        + (elapsed * (0.9 + cluster_index as f32 * 0.11)
                            + cluster_index as f32 * 0.73)
                            .sin()
                            * (0.04 + config.pulse_strength * 0.12);
                    let geometric_distortion = ((cluster_angle * 2.6 + elapsed * 1.3).sin() * 0.09
                        + (cluster_angle * 5.2 - elapsed * 0.9).cos() * 0.05
                        + rng.random_range(-0.025..0.025))
                        * config.intensity.clamp(0.5, 2.5);
                    let rx = ring_half_extents.x
                        * cluster_breath
                        * (1.0 + geometric_distortion.clamp(-0.14, 0.18));
                    let ry = ring_half_extents.y
                        * cluster_breath
                        * (1.0 + geometric_distortion.clamp(-0.14, 0.18));

                    let cos_a = cluster_angle.cos();
                    let sin_a = cluster_angle.sin();
                    let cluster_center = Vec2::new(cos_a * rx, sin_a * ry);
                    let edge_normal =
                        Vec2::new(cos_a / rx.max(0.001), sin_a / ry.max(0.001)).normalize_or_zero();
                    let tangent = Vec2::new(-edge_normal.y, edge_normal.x) * swirl_direction;

                    let cluster_tangent_offset = tangent
                        * rng.random_range(
                            -cluster_pattern.tangential_spread_px
                                ..cluster_pattern.tangential_spread_px,
                        );
                    let cluster_radial_offset = edge_normal
                        * rng.random_range(
                            -cluster_pattern.radial_spread_px..cluster_pattern.radial_spread_px,
                        );
                    let puff_angle = rng.random_range(0.0..std::f32::consts::TAU);
                    let puff_dir = Vec2::new(puff_angle.cos(), puff_angle.sin());
                    let puff_offset =
                        puff_dir * rng.random_range(0.0..cluster_pattern.puff_spread_px);
                    let edge_offset = cluster_center
                        + cluster_tangent_offset
                        + cluster_radial_offset
                        + puff_offset;

                    let outward_speed =
                        rng.random_range(0.18..0.90) * config.intensity.clamp(0.8, 2.0);
                    let swirl_speed = rng.random_range(
                        cluster_pattern.orbit_speed_px * 0.92
                            ..cluster_pattern.orbit_speed_px * 1.20,
                    );
                    let puff_speed = rng.random_range(0.6..2.4) * (0.5 + config.softness * 0.4);
                    let puff_velocity_dir =
                        (puff_offset.normalize_or_zero() + edge_normal * 0.65).normalize_or_zero();
                    let ui_velocity = Vec2::new(
                        edge_normal.x * outward_speed
                            + tangent.x * swirl_speed
                            + puff_velocity_dir.x * puff_speed,
                        edge_normal.y * outward_speed
                            + tangent.y * swirl_speed
                            + puff_velocity_dir.y * puff_speed,
                    );
                    let lifetime = (rng.random_range(1.2..2.2) * lifetime_scale).max(0.75);
                    let size_scale =
                        config.particle_size_scale.clamp(0.7, 1.4) * (0.98 + config.softness * 0.26);
                    let base_size_px = (12.0 + config.intensity.clamp(0.8, 4.0) * 1.5) * size_scale;
                    (
                        edge_offset,
                        ui_velocity.extend(0.0),
                        lifetime,
                        base_size_px,
                    )
                }
                MistSmokePlacement::SurfaceCloud => {
                    let orbit_extent = surface_pattern.orbital_extent;
                    let local_breath = 1.0
                        + (elapsed * (0.6 + cluster_index as f32 * 0.09)
                            + cluster_index as f32 * 0.58)
                            .sin()
                            * (0.12 + config.pulse_strength * 0.28);
                    let cos_a = cluster_angle.cos();
                    let sin_a = cluster_angle.sin();
                    let cluster_center = Vec2::new(
                        cos_a * orbit_extent.x * local_breath,
                        sin_a * orbit_extent.y * local_breath,
                    );
                    let orbit_normal = Vec2::new(
                        cos_a / orbit_extent.x.max(0.001),
                        sin_a / orbit_extent.y.max(0.001),
                    )
                    .normalize_or_zero();
                    let tangent = Vec2::new(-orbit_normal.y, orbit_normal.x) * swirl_direction;
                    let cluster_tangent_offset = tangent
                        * rng.random_range(
                            -surface_pattern.tangential_spread_px
                                ..surface_pattern.tangential_spread_px,
                        );
                    let cluster_radial_offset = orbit_normal
                        * rng.random_range(
                            -surface_pattern.radial_spread_px..surface_pattern.radial_spread_px,
                        );
                    let puff_angle = rng.random_range(0.0..std::f32::consts::TAU);
                    let puff_dir = Vec2::new(puff_angle.cos(), puff_angle.sin());
                    let puff_offset =
                        puff_dir * rng.random_range(0.0..surface_pattern.puff_spread_px);
                    let surface_offset = cluster_center
                        + cluster_tangent_offset
                        + cluster_radial_offset
                        + puff_offset;

                    let core_pull = (-surface_offset).normalize_or_zero();
                    let swirl_speed = rng.random_range(
                        surface_pattern.swirl_speed_px * 0.72
                            ..surface_pattern.swirl_speed_px * 1.26,
                    );
                    let drift_speed = rng.random_range(1.2..4.8)
                        * (0.55 + config.softness + config.pulse_strength);
                    let puff_speed = rng.random_range(0.8..3.4) * (0.75 + config.softness * 0.6);
                    let ui_velocity = Vec2::new(
                        tangent.x * swirl_speed
                            + core_pull.x * drift_speed * 0.35
                            + puff_dir.x * puff_speed,
                        tangent.y * swirl_speed
                            + core_pull.y * drift_speed * 0.35
                            + puff_dir.y * puff_speed,
                    );
                    let lifetime = (rng.random_range(1.45..2.7) * lifetime_scale).max(0.95);
                    let size_scale = config.particle_size_scale.clamp(0.7, 1.8)
                        * (1.28 + config.softness * 0.62);
                    let base_size_px = (18.0 + config.intensity.clamp(0.8, 4.2) * 3.0) * size_scale;
                    (
                        surface_offset,
                        ui_velocity.extend(0.0),
                        lifetime,
                        base_size_px,
                    )
                }
            };

            let stretch = rng.random_range(0.8..1.25);
            let start_size = clamp_screen_particle_size(Vec2::new(
                base_size_px
                    * match placement {
                        MistSmokePlacement::BorderOrbit => 0.42,
                        MistSmokePlacement::SurfaceCloud => 0.46,
                    }
                    * stretch,
                base_size_px
                    * match placement {
                        MistSmokePlacement::BorderOrbit => 0.42,
                        MistSmokePlacement::SurfaceCloud => 0.46,
                    }
                    / stretch,
            ));
            let end_size = clamp_screen_particle_size(Vec2::new(
                base_size_px
                    * match placement {
                        MistSmokePlacement::BorderOrbit => 1.02,
                        MistSmokePlacement::SurfaceCloud => 1.12,
                    }
                    * stretch,
                base_size_px
                    * match placement {
                        MistSmokePlacement::BorderOrbit => 1.02,
                        MistSmokePlacement::SurfaceCloud => 1.12,
                    }
                    / stretch,
            ));
            let hsv_shift = rng.random_range(-0.05..0.05);
            let end_hsla = start_hsla.with_hue(start_hsla.hue + hsv_shift * 360.0);
            let alpha_peak = match placement {
                MistSmokePlacement::BorderOrbit => {
                    let (min_alpha, max_alpha) = border_alpha_peak_range(config.preset);
                    (rng.random_range(min_alpha..max_alpha)
                        * (config.intensity / 3.1).clamp(0.92, 1.46))
                    .clamp(min_alpha, (max_alpha + 0.08).min(0.92))
                }
                MistSmokePlacement::SurfaceCloud => {
                    let (min_alpha, max_alpha) = surface_alpha_peak_range(config.preset);
                    (rng.random_range(min_alpha..max_alpha)
                        * (config.intensity / 3.4).clamp(0.72, 1.12))
                    .clamp(min_alpha, (max_alpha + 0.03).min(0.18))
                }
            };
            let start_linear = base_color.to_linear();
            let end_linear = Color::from(end_hsla).to_linear();

            activate_mist_smoke_particle(
                &mut commands,
                &mut pool,
                entity,
                size,
                MistSmokeSpawnBundle {
                    image: sprite.handle.clone(),
                    size: start_size,
                    local_offset: spawn_offset,
                    color: Color::LinearRgba(LinearRgba::new(
                        start_linear.red,
                        start_linear.green,
                        start_linear.blue,
                        alpha_peak,
                    )),
                    particle: MistSmokeParticle {
                        emitter: entity,
                        domain: MistSmokeDomain::ScreenUi,
                        placement,
                        velocity,
                        acceleration: Vec3::ZERO,
                        local_offset: Vec3::new(spawn_offset.x, spawn_offset.y, 0.0),
                        outer_half_extents,
                        inner_keepout_half_extents,
                        age_secs: 0.0,
                        lifetime_secs: lifetime,
                        rotation_speed: rng.random_range(-3.0..3.0),
                        start_size,
                        end_size,
                        start_color: LinearRgba::new(
                            start_linear.red,
                            start_linear.green,
                            start_linear.blue,
                            alpha_peak,
                        ),
                        end_color: LinearRgba::new(
                            end_linear.red,
                            end_linear.green,
                            end_linear.blue,
                            0.0,
                        ),
                        follow_emitter: true,
                    },
                },
            );

            spawned += 1;
            remaining_spawn_global = remaining_spawn_global.saturating_sub(1);
            remaining_live_global = remaining_live_global.saturating_sub(1);
            *scratch.per_emitter_live.entry(entity).or_insert(0) += 1;
        }
    }
}

fn update_mist_smoke_particles(
    mut commands: Commands,
    time: Res<Time>,
    backend: Option<Res<MistSmokeBackend>>,
    mut pool: ResMut<MistSmokeParticlePool>,
    emitters: Query<
        (
            Option<&Node>,
            Option<&ComputedNode>,
            Option<&MistSmokeTarget>,
            Option<&MistSmokeConfig>,
            Option<&Visibility>,
            Option<&InheritedVisibility>,
        ),
        (Without<NoMistSmoke>, Without<MistSmokeParticle>),
    >,
    mut particles: Query<(Entity, &mut Node, &mut ImageNode, &mut MistSmokeParticle)>,
) {
    if matches!(backend.as_deref(), Some(MistSmokeBackend::ShaderRing)) {
        for (entity, _, _, _) in &mut particles {
            recycle_mist_smoke_particle(&mut commands, &mut pool, entity);
        }
        return;
    }

    let dt = time.delta_secs().max(0.0);
    if dt <= f32::EPSILON {
        return;
    }

    for (entity, mut node, mut image, mut particle) in &mut particles {
        particle.age_secs += dt;
        if particle.age_secs >= particle.lifetime_secs {
            recycle_mist_smoke_particle(&mut commands, &mut pool, entity);
            continue;
        }

        let acceleration = particle.acceleration;
        particle.velocity += acceleration * dt;

        if particle.follow_emitter {
            let Ok((
                Some(emitter_node),
                Some(computed),
                Some(target),
                Some(config),
                visibility,
                inherited_visibility,
            )) = emitters.get(particle.emitter)
            else {
                recycle_mist_smoke_particle(&mut commands, &mut pool, entity);
                continue;
            };

            if emitter_node.display == Display::None
                || visibility.is_some_and(|visibility| *visibility == Visibility::Hidden)
                || inherited_visibility.is_some_and(|visibility| !visibility.get())
            {
                recycle_mist_smoke_particle(&mut commands, &mut pool, entity);
                continue;
            }

            let size = computed.size();
            if !(size.x.is_finite() && size.y.is_finite() && size.x > 1.0 && size.y > 1.0) {
                recycle_mist_smoke_particle(&mut commands, &mut pool, entity);
                continue;
            }
            if target.domain != particle.domain || !config.supports_particles() {
                recycle_mist_smoke_particle(&mut commands, &mut pool, entity);
                continue;
            }

            particle.velocity *= 0.958_f32.powf(dt * 60.0);
            let velocity = particle.velocity;
            particle.local_offset += velocity * dt;
            let clamped_xy = match particle.placement {
                MistSmokePlacement::BorderOrbit => clamp_offset_to_ring_band(
                    particle.local_offset.truncate(),
                    particle.outer_half_extents,
                    particle.inner_keepout_half_extents,
                ),
                MistSmokePlacement::SurfaceCloud => {
                    clamp_offset_to_half_extents(particle.local_offset.truncate(), particle.outer_half_extents)
                }
            };
            particle.local_offset.x = clamped_xy.x;
            particle.local_offset.y = clamped_xy.y;
        } else {
            let velocity = particle.velocity;
            particle.local_offset += velocity * dt;
        }

        let t = (particle.age_secs / particle.lifetime_secs.max(f32::EPSILON)).clamp(0.0, 1.0);
        let grow_t = 1.0 - (1.0 - t) * (1.0 - t);
        let fade_t = t * t;
        let draw_size =
            clamp_screen_particle_size(particle.start_size.lerp(particle.end_size, grow_t));
        if let Ok((Some(_), Some(computed), _, _, _, _)) = emitters.get(particle.emitter) {
            *node = screen_ui_particle_node(
                computed.size(),
                particle.local_offset.truncate(),
                draw_size,
            );
        }
        image.color = Color::LinearRgba(LinearRgba::new(
            particle.start_color.red + (particle.end_color.red - particle.start_color.red) * fade_t,
            particle.start_color.green
                + (particle.end_color.green - particle.start_color.green) * fade_t,
            particle.start_color.blue
                + (particle.end_color.blue - particle.start_color.blue) * fade_t,
            particle.start_color.alpha
                + (particle.end_color.alpha - particle.start_color.alpha) * fade_t,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::asset::AssetPlugin;

    #[test]
    fn smoke_particles_plugin_registers_runtime_resources() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));
        app.add_plugins(SmokeParticlesPlugin);

        assert!(app.world().contains_resource::<MistSmokeBudget>());
        assert!(app.world().contains_resource::<MistSmokeParticlePool>());
    }

    #[test]
    fn screen_presets_default_to_particles_only() {
        let button = MistSmokeConfig::screen_preset(MistSmokePreset::StandardButton);
        let panel = MistSmokeConfig::screen_preset(MistSmokePreset::PanelFrame);

        assert!(matches!(button.domain, MistSmokeDomain::ScreenUi));
        assert!(matches!(
            button.overlay_mode,
            MistSmokeOverlayMode::ParticlesOnly
        ));
        assert!(panel.particle_density > 0.0);
        assert!(panel.supports_particles());
        assert!(!panel.supports_ring());
    }

    #[test]
    fn derived_ring_tracks_particle_tuning() {
        let config = MistSmokeConfig::screen_preset(MistSmokePreset::ToolbarButton)
            .with_thickness(0.17)
            .with_intensity(3.7)
            .with_softness(0.42);
        let ring = derived_screen_ring(config);

        assert!((ring.thickness - 0.17).abs() < 1e-5);
        assert!((ring.intensity - 3.7).abs() < 1e-5);
        assert!((ring.softness - 0.42).abs() < 1e-5);
    }

    #[test]
    fn border_patterns_prioritize_dense_ring_coverage() {
        let config = MistSmokeConfig::screen_preset(MistSmokePreset::StandardButton);
        let pattern = screen_smoke_cluster_pattern(Vec2::new(180.0, 52.0), config);
        let (outer, inner) = screen_border_orbit_band_extents(Vec2::new(180.0, 52.0), config, pattern);

        assert!(pattern.cluster_count >= 14);
        assert!(pattern.particles_per_cluster >= 8);
        assert!(outer.x - inner.x >= 7.0);
        assert!(outer.y - inner.y >= 7.0);
    }

    #[test]
    fn surface_patterns_and_alpha_ranges_keep_internal_components_visible() {
        let config = MistSmokeConfig::screen_preset(MistSmokePreset::PrimaryAction);
        let pattern = surface_smoke_cluster_pattern(
            Vec2::new(120.0, 42.0),
            config,
            MistSmokePlacement::SurfaceCloud,
        );
        let (min_alpha, max_alpha) = surface_alpha_peak_range(MistSmokePreset::PrimaryAction);

        assert!(pattern.cluster_count >= 5);
        assert!(pattern.particles_per_cluster >= 4);
        assert!(min_alpha >= 0.10);
        assert!(max_alpha >= 0.20);
    }
}
