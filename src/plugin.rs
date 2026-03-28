use crate::{init_smoke_ring_shader, SmokeBorder, SmokeRingMaterial, SmokeRingParams};
use bevy::ecs::hierarchy::ChildOf;
use bevy::math::primitives::Rectangle;
use bevy::prelude::*;
use bevy::sprite_render::{Material2dPlugin, MeshMaterial2d};
use bevy::transform::TransformSystems;
use bevy::ui::{ComputedNode, UiSystems};
use bevy_camera::prelude::ViewVisibility;
use bevy_camera::visibility::RenderLayers;
use bevy_mesh::Mesh2d;

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
            base_band_px: 8.0,
            irregularity_px: 2.0,
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
}

#[derive(Component, Debug, Clone, Copy, Default)]
struct SmokeRingMesh;

pub struct SmokeRingPlugin;

impl Plugin for SmokeRingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SmokeRingSettings>();
        if !app.is_plugin_added::<Material2dPlugin<SmokeRingMaterial>>() {
            app.add_plugins(Material2dPlugin::<SmokeRingMaterial>::default());
        }
        app.add_systems(Startup, init_smoke_ring_assets).add_systems(
            PostUpdate,
            sync_smoke_rings
                .after(UiSystems::Layout)
                .before(TransformSystems::Propagate),
        );
    }
}

fn init_smoke_ring_assets(mut shaders: ResMut<Assets<Shader>>) {
    init_smoke_ring_shader(&mut shaders);
}

fn has_valid_size(size: Vec2) -> bool {
    size.x.is_finite() && size.y.is_finite() && size.x > 1.0 && size.y > 1.0
}

fn ring_rect_size(node_size: Vec2, padding: SmokeRingPadding, settings: &SmokeRingSettings) -> Vec2 {
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

fn material_from_smoke_border(smoke: &SmokeBorder, rect_size: Vec2, time: f32) -> SmokeRingMaterial {
    let base = smoke.color.to_linear();
    let pulse = smoke.pulse_color.to_linear();
    let pulse_mix = (smoke.pulse_strength * 0.55).clamp(0.0, 0.9);
    let rgb = Vec3::new(base.red, base.green, base.blue)
        .lerp(Vec3::new(pulse.red, pulse.green, pulse.blue), pulse_mix);
    let alpha = (0.35 + smoke.intensity * 0.28).clamp(0.2, 0.98);

    SmokeRingMaterial {
        params: SmokeRingParams {
            color: Vec4::new(rgb.x, rgb.y, rgb.z, alpha),
            rect_size,
            time,
            thickness: (8.0 * (1.0 + smoke.thickness.clamp(0.05, 0.6) * 1.3)).clamp(8.0, 20.0),
            noise_scale: smoke.noise_scale.clamp(12.0, 84.0),
            flow_speed: smoke.flow_speed.clamp(0.1, 3.5),
            breakup: (0.15 + smoke.pulse_strength.clamp(0.0, 1.0) * 0.25).clamp(0.05, 0.45),
            softness: (3.0 + smoke.softness.clamp(0.0, 1.0) * 18.0).clamp(2.0, 24.0),
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

fn sync_smoke_rings(
    mut commands: Commands,
    time: Res<Time>,
    settings: Res<SmokeRingSettings>,
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
    mut rings: Query<
        (
            &ChildOf,
            &mut Transform,
            &mut Visibility,
            &mut MeshMaterial2d<SmokeRingMaterial>,
            Option<&RenderLayers>,
        ),
        With<SmokeRingMesh>,
    >,
) {
    let ring_mesh = shared_ring_mesh
        .get_or_insert_with(|| meshes.add(Mesh::from(Rectangle::new(1.0, 1.0))))
        .clone();

    for (entity, smoke, padding, computed, sprite, shell) in &parents {
        let Some(smoke) = smoke else {
            if let Some(shell) = shell {
                remove_ring(&mut commands, entity, *shell);
            }
            continue;
        };

        if !settings.enabled || smoke.intensity <= f32::EPSILON {
            if let Some(shell) = shell {
                remove_ring(&mut commands, entity, *shell);
            }
            continue;
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
                if let Ok((_, _, mut visibility, _, render_layers)) = rings.get_mut(shell.ring_entity)
                {
                    *visibility = Visibility::Hidden;
                    apply_layers(&mut commands, shell.ring_entity, render_layers, &settings);
                } else {
                    commands.entity(entity).remove::<SmokeRingShell>();
                }
            }
            continue;
        }

        let padding = padding.copied().unwrap_or_default();
        let rect_size = ring_rect_size(node_size, padding, &settings);
        let next_material = material_from_smoke_border(smoke, rect_size, time.elapsed_secs());
        let next_transform = ring_transform(node_size, rect_size, settings.z_offset);

        if let Some(shell) = shell {
            let Ok((relationship, mut transform, mut visibility, mut material_handle, render_layers)) =
                rings.get_mut(shell.ring_entity)
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
                material_handle.0 = materials.add(next_material);
            }
            continue;
        }

        let material = materials.add(next_material);
        let ring_entity = commands
            .spawn((
                Mesh2d(ring_mesh.clone()),
                MeshMaterial2d(material),
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
        commands.entity(entity).insert(SmokeRingShell { ring_entity });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_layout_respects_padding_and_min_extent() {
        let settings = SmokeRingSettings::default();
        let rect = ring_rect_size(Vec2::new(100.0, 40.0), SmokeRingPadding::all(6.0), &settings);
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
        app.add_plugins(SmokeRingPlugin);
        assert!(app.world().contains_resource::<SmokeRingSettings>());
    }
}

