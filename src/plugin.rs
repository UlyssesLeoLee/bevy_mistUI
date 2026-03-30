use crate::{init_smoke_ring_shader, SmokeBorder, SmokeRingMaterial, SmokeRingParams};
use bevy::ecs::hierarchy::ChildOf;
use bevy::math::primitives::Rectangle;
use bevy::prelude::*;
use bevy::sprite_render::{Material2dPlugin, MeshMaterial2d};
use bevy::ui::ComputedNode;
use bevy_camera::prelude::ViewVisibility;
use bevy_camera::visibility::RenderLayers;
use bevy_mesh::Mesh2d;

#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq, Reflect)]
pub enum MistSmokeBackend {
    #[default]
    Particles,
    ShaderRing,
}

#[derive(Resource, Clone, Debug)]
pub struct SmokeRingSettings {
    pub enabled: bool,
    pub render_layers: Option<RenderLayers>,
    pub z_offset: f32,
    pub base_band_px: f32,
    pub irregularity_px: f32,
    pub min_half_extent_px: f32,
}

impl Default for SmokeRingSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            render_layers: None,
            z_offset: -0.5,
            base_band_px: 12.0,
            irregularity_px: 4.0,
            min_half_extent_px: 24.0,
        }
    }
}

#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SmokeRingPadding {
    pub horizontal: f32,
    pub vertical: f32,
}

impl SmokeRingPadding {
    pub const fn all(value: f32) -> Self {
        Self {
            horizontal: value,
            vertical: value,
        }
    }

    pub const fn symmetric(horizontal: f32, vertical: f32) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }
}

#[derive(Bundle, Default)]
pub struct SmokeRingBundle {
    pub border: SmokeBorder,
    pub padding: SmokeRingPadding,
}

#[derive(Component, Debug, Clone, Copy)]
struct SmokeRingShell {
    ring_entity: Entity,
    mode: SmokeRingShellMode,
}

#[derive(Component, Debug, Clone, Copy, Default)]
struct SmokeRingMesh;

#[derive(Component, Debug, Clone, Copy, Default)]
struct SmokeRingUiNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SmokeRingShellMode {
    UiNode,
    Mesh2d,
}

pub struct SmokeRingPlugin;

impl Plugin for SmokeRingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SmokeRingSettings>();
        if !app.is_plugin_added::<Material2dPlugin<SmokeRingMaterial>>() {
            app.add_plugins(Material2dPlugin::<SmokeRingMaterial>::default());
        }
        if !app.is_plugin_added::<UiMaterialPlugin<SmokeRingMaterial>>() {
            app.add_plugins(UiMaterialPlugin::<SmokeRingMaterial>::default());
        }
        app.add_systems(Startup, init_smoke_ring_assets)
            .add_systems(PostStartup, sync_smoke_rings)
            .add_systems(PreUpdate, sync_smoke_rings);
    }
}

fn init_smoke_ring_assets(mut shaders: ResMut<Assets<Shader>>) {
    init_smoke_ring_shader(&mut shaders);
}

fn has_valid_size(size: Vec2) -> bool {
    size.x.is_finite() && size.y.is_finite() && size.x > 1.0 && size.y > 1.0
}

fn ring_rect_size(
    node_size: Vec2,
    padding: SmokeRingPadding,
    settings: &SmokeRingSettings,
) -> Vec2 {
    let band = (settings.base_band_px + settings.irregularity_px) * 2.0;
    let min_extent = settings.min_half_extent_px * 2.0;
    let padded = Vec2::new(
        node_size.x + padding.horizontal * 2.0,
        node_size.y + padding.vertical * 2.0,
    );
    Vec2::new(
        (padded.x + band).max(min_extent),
        (padded.y + band).max(min_extent),
    )
}

fn ring_transform(node_size: Vec2, rect_size: Vec2, z_offset: f32) -> Transform {
    let mut transform =
        Transform::from_translation(Vec3::new(node_size.x * 0.5, -node_size.y * 0.5, z_offset));
    transform.scale = Vec3::new(rect_size.x, rect_size.y, 1.0);
    transform
}

fn ring_ui_node(node_size: Vec2, rect_size: Vec2) -> Node {
    let inset = (rect_size - node_size) * 0.5;
    Node {
        position_type: PositionType::Absolute,
        left: Val::Px(-inset.x),
        top: Val::Px(-inset.y),
        width: Val::Px(rect_size.x),
        height: Val::Px(rect_size.y),
        ..default()
    }
}

fn ring_corner_radius(
    computed: Option<&ComputedNode>,
    padding: SmokeRingPadding,
    settings: &SmokeRingSettings,
) -> Vec4 {
    let expansion = padding.horizontal.max(padding.vertical)
        + (settings.base_band_px + settings.irregularity_px) * 0.5;

    if let Some(computed) = computed {
        let radii: [f32; 4] = computed.border_radius().into();
        Vec4::new(radii[0], radii[1], radii[2], radii[3]) + Vec4::splat(expansion)
    } else {
        Vec4::ZERO
    }
}

fn material_from_smoke_border(
    smoke: &SmokeBorder,
    rect_size: Vec2,
    corner_radius: Vec4,
    time: f32,
) -> SmokeRingMaterial {
    let base = smoke.color.to_linear();
    let pulse = smoke.pulse_color.to_linear();
    let pulse_mix = (0.18 + smoke.pulse_strength * 0.55).clamp(0.0, 0.92);
    let rgb = Vec3::new(base.red, base.green, base.blue)
        .lerp(Vec3::new(pulse.red, pulse.green, pulse.blue), pulse_mix);
    let alpha = (0.18 + smoke.intensity * 0.16).clamp(0.26, 0.94);

    SmokeRingMaterial {
        params: SmokeRingParams {
            color: Vec4::new(rgb.x, rgb.y, rgb.z, alpha),
            rect_size,
            corner_radius,
            time,
            thickness: (10.0 + smoke.thickness.clamp(0.05, 0.8) * 22.0).clamp(10.0, 30.0),
            noise_scale: smoke.noise_scale.clamp(10.0, 72.0),
            flow_speed: smoke.flow_speed.clamp(0.08, 2.2),
            breakup: (0.28
                + smoke.pulse_strength.clamp(0.0, 1.0) * 0.34
                + smoke.softness.clamp(0.0, 1.0) * 0.12)
                .clamp(0.2, 0.8),
            softness: (6.0 + smoke.softness.clamp(0.0, 1.0) * 24.0).clamp(6.0, 32.0),
        },
    }
}

fn remove_ring(commands: &mut Commands, parent: Entity, shell: SmokeRingShell) {
    if let Ok(mut ring_entity) = commands.get_entity(shell.ring_entity) {
        ring_entity.despawn();
    }
    if let Ok(mut parent_entity) = commands.get_entity(parent) {
        parent_entity.remove::<SmokeRingShell>();
    }
}

fn apply_layers(
    commands: &mut Commands,
    entity: Entity,
    render_layers: Option<&RenderLayers>,
    settings: &SmokeRingSettings,
) {
    match (&settings.render_layers, render_layers) {
        (Some(desired), Some(current)) if current == desired => {}
        (Some(desired), _) => {
            commands.entity(entity).insert(desired.clone());
        }
        (None, Some(_)) => {
            commands.entity(entity).remove::<RenderLayers>();
        }
        (None, None) => {}
    }
}

fn shell_mode_for_parent(
    computed: Option<&ComputedNode>,
    sprite: Option<&Sprite>,
) -> Option<SmokeRingShellMode> {
    if computed.is_some() {
        Some(SmokeRingShellMode::UiNode)
    } else if sprite.is_some() {
        Some(SmokeRingShellMode::Mesh2d)
    } else {
        None
    }
}

fn sync_smoke_rings(
    mut commands: Commands,
    time: Res<Time>,
    settings: Res<SmokeRingSettings>,
    backend: Option<Res<MistSmokeBackend>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SmokeRingMaterial>>,
    mut shared_ring_mesh: Local<Option<Handle<Mesh>>>,
    parents: Query<
        (
            Entity,
            Option<&SmokeBorder>,
            Option<&SmokeRingPadding>,
            Option<&ComputedNode>,
            Option<&Sprite>,
            Option<&SmokeRingShell>,
        ),
        Or<(With<SmokeBorder>, With<SmokeRingShell>)>,
    >,
    mut ui_rings: Query<
        (
            &ChildOf,
            &mut Node,
            &mut Visibility,
            &mut MaterialNode<SmokeRingMaterial>,
        ),
        (With<SmokeRingUiNode>, Without<SmokeRingMesh>),
    >,
    mut mesh_rings: Query<
        (
            &ChildOf,
            &mut Transform,
            &mut Visibility,
            &mut MeshMaterial2d<SmokeRingMaterial>,
            Option<&RenderLayers>,
        ),
        (With<SmokeRingMesh>, Without<SmokeRingUiNode>),
    >,
) {
    let ring_backend_active = !matches!(backend.as_deref(), Some(MistSmokeBackend::Particles));
    let ring_mesh = shared_ring_mesh
        .get_or_insert_with(|| meshes.add(Mesh::from(Rectangle::new(1.0, 1.0))))
        .clone();

    for (entity, smoke, padding, computed, sprite, shell) in &parents {
        let mut shell = shell.copied();
        let Some(smoke) = smoke else {
            if let Some(shell) = shell {
                remove_ring(&mut commands, entity, shell);
            }
            continue;
        };

        if !ring_backend_active || !settings.enabled || smoke.intensity <= f32::EPSILON {
            if let Some(shell) = shell {
                remove_ring(&mut commands, entity, shell);
            }
            continue;
        }

        let Some(mode) = shell_mode_for_parent(computed, sprite) else {
            if let Some(shell) = shell {
                remove_ring(&mut commands, entity, shell);
            }
            continue;
        };

        if shell.is_some_and(|shell| shell.mode != mode) {
            remove_ring(&mut commands, entity, shell.expect("checked above"));
            shell = None;
        }

        let node_size = if let Some(computed) = computed {
            computed.size()
        } else if let Some(sprite) = sprite {
            sprite.custom_size.unwrap_or(Vec2::new(120.0, 120.0))
        } else {
            Vec2::ZERO
        };

        if !has_valid_size(node_size) {
            if let Some(shell) = shell {
                match shell.mode {
                    SmokeRingShellMode::UiNode => {
                        if let Ok((_, _, mut visibility, _)) = ui_rings.get_mut(shell.ring_entity) {
                            *visibility = Visibility::Hidden;
                        } else {
                            commands.entity(entity).remove::<SmokeRingShell>();
                        }
                    }
                    SmokeRingShellMode::Mesh2d => {
                        if let Ok((_, _, mut visibility, _, render_layers)) =
                            mesh_rings.get_mut(shell.ring_entity)
                        {
                            *visibility = Visibility::Hidden;
                            apply_layers(
                                &mut commands,
                                shell.ring_entity,
                                render_layers,
                                &settings,
                            );
                        } else {
                            commands.entity(entity).remove::<SmokeRingShell>();
                        }
                    }
                }
            }
            continue;
        }

        let padding = padding.copied().unwrap_or_default();
        let rect_size = ring_rect_size(node_size, padding, &settings);
        let corner_radius = ring_corner_radius(computed, padding, &settings);
        let next_material =
            material_from_smoke_border(smoke, rect_size, corner_radius, time.elapsed_secs());

        match mode {
            SmokeRingShellMode::UiNode => {
                let next_node = ring_ui_node(node_size, rect_size);

                if let Some(shell) = shell {
                    let Ok((relationship, mut node, mut visibility, mut material_handle)) =
                        ui_rings.get_mut(shell.ring_entity)
                    else {
                        commands.entity(entity).remove::<SmokeRingShell>();
                        continue;
                    };

                    if relationship.parent() != entity {
                        commands.entity(entity).remove::<SmokeRingShell>();
                        continue;
                    }

                    *node = next_node;
                    *visibility = Visibility::Inherited;
                    if let Some(material) = materials.get_mut(&material_handle.0) {
                        material.params = next_material.params;
                    } else {
                        material_handle.0 = materials.add(next_material.clone());
                    }
                    continue;
                }

                let ring_entity = commands
                    .spawn((
                        next_node,
                        MaterialNode(materials.add(next_material.clone())),
                        Visibility::Inherited,
                        SmokeRingUiNode,
                        Name::new("Smoke Ring"),
                    ))
                    .id();
                commands.entity(entity).add_child(ring_entity);
                commands
                    .entity(entity)
                    .insert(SmokeRingShell { ring_entity, mode });
            }
            SmokeRingShellMode::Mesh2d => {
                let next_transform = ring_transform(node_size, rect_size, settings.z_offset);

                if let Some(shell) = shell {
                    let Ok((
                        relationship,
                        mut transform,
                        mut visibility,
                        mut material_handle,
                        render_layers,
                    )) = mesh_rings.get_mut(shell.ring_entity)
                    else {
                        commands.entity(entity).remove::<SmokeRingShell>();
                        continue;
                    };

                    if relationship.parent() != entity {
                        commands.entity(entity).remove::<SmokeRingShell>();
                        continue;
                    }

                    *transform = next_transform;
                    *visibility = Visibility::Inherited;
                    apply_layers(&mut commands, shell.ring_entity, render_layers, &settings);
                    if let Some(material) = materials.get_mut(&material_handle.0) {
                        material.params = next_material.params;
                    } else {
                        material_handle.0 = materials.add(next_material.clone());
                    }
                    continue;
                }

                let ring_entity = commands
                    .spawn((
                        Mesh2d(ring_mesh.clone()),
                        MeshMaterial2d(materials.add(next_material)),
                        next_transform,
                        GlobalTransform::default(),
                        Visibility::Inherited,
                        InheritedVisibility::default(),
                        ViewVisibility::default(),
                        SmokeRingMesh,
                        Name::new("Smoke Ring"),
                    ))
                    .id();
                apply_layers(&mut commands, ring_entity, None, &settings);
                commands.entity(entity).add_child(ring_entity);
                commands
                    .entity(entity)
                    .insert(SmokeRingShell { ring_entity, mode });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::shader::Shader;

    #[test]
    fn ring_layout_respects_padding_and_min_extent() {
        let settings = SmokeRingSettings::default();
        let rect = ring_rect_size(
            Vec2::new(100.0, 40.0),
            SmokeRingPadding::all(6.0),
            &settings,
        );
        assert!(rect.x > 100.0);
        assert!(rect.y > 40.0);

        let tiny = ring_rect_size(Vec2::new(4.0, 4.0), SmokeRingPadding::default(), &settings);
        assert_eq!(tiny, Vec2::splat(settings.min_half_extent_px * 2.0));
    }

    #[test]
    fn plugin_registers_settings_resource() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(AssetPlugin::default());
        app.init_asset::<Shader>();
        app.add_plugins(SmokeRingPlugin);
        assert!(app.world().contains_resource::<SmokeRingSettings>());
    }

    #[test]
    fn shader_ring_backend_spawns_ui_material_shell_for_ui_nodes() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(AssetPlugin::default());
        app.init_asset::<Mesh>();
        app.init_asset::<Shader>();
        app.add_plugins(SmokeRingPlugin);
        app.insert_resource(MistSmokeBackend::ShaderRing);

        let host = app
            .world_mut()
            .spawn((
                Node {
                    width: Val::Px(120.0),
                    height: Val::Px(52.0),
                    ..default()
                },
                ComputedNode {
                    size: Vec2::new(120.0, 52.0),
                    unrounded_size: Vec2::new(120.0, 52.0),
                    ..default()
                },
                SmokeBorder::gaseous_idle(7),
                SmokeRingPadding::all(6.0),
            ))
            .id();

        app.update();

        let shell = *app
            .world()
            .entity(host)
            .get::<SmokeRingShell>()
            .expect("host should own a smoke ring shell");
        assert_eq!(shell.mode, SmokeRingShellMode::UiNode);

        let ring = app.world().entity(shell.ring_entity);
        assert!(ring.contains::<SmokeRingUiNode>());
        assert!(ring.contains::<MaterialNode<SmokeRingMaterial>>());

        let node = ring.get::<Node>().expect("ring should have a UI node");
        assert_eq!(node.position_type, PositionType::Absolute);
        assert!(matches!(node.width, Val::Px(width) if width > 120.0));
        assert!(matches!(node.height, Val::Px(height) if height > 52.0));
    }
}
