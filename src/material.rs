use bevy::{
    asset::{Asset, Assets},
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::{Shader, ShaderRef},
    sprite_render::{AlphaMode2d, Material2d},
};

#[derive(AsBindGroup, TypePath, Debug, Clone, Asset)]
pub struct SmokeRingMaterial {
    #[uniform(0)]
    pub params: SmokeRingParams,
}

#[derive(ShaderType, Debug, Clone, Copy)]
pub struct SmokeRingParams {
    pub color: Vec4,
    pub rect_size: Vec2,
    pub time: f32,
    pub thickness: f32,
    pub noise_scale: f32,
    pub flow_speed: f32,
    pub breakup: f32,
    pub softness: f32,
}

impl Default for SmokeRingParams {
    fn default() -> Self {
        Self {
            color: Vec4::new(0.85, 0.92, 1.0, 0.45),
            rect_size: Vec2::new(100.0, 100.0),
            time: 0.0,
            thickness: 20.0,
            noise_scale: 42.0,
            flow_speed: 0.75,
            breakup: 0.35,
            softness: 3.2,
        }
    }
}

impl Default for SmokeRingMaterial {
    fn default() -> Self {
        Self {
            params: SmokeRingParams::default(),
        }
    }
}

impl Material2d for SmokeRingMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Handle(SMOKE_RING_SHADER_HANDLE.clone())
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

impl UiMaterial for SmokeRingMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Handle(SMOKE_RING_UI_SHADER_HANDLE.clone())
    }
}

pub const SMOKE_RING_SHADER_HANDLE: Handle<Shader> =
    bevy::asset::uuid_handle!("8192aff0-e1e0-43ce-a4db-912808c32493");
pub const SMOKE_RING_UI_SHADER_HANDLE: Handle<Shader> =
    bevy::asset::uuid_handle!("aef98f65-1b20-4d3e-8a8b-66f4c8a6ca12");

const SMOKE_RING_SHADER_SRC: &str = include_str!("../assets/shaders/smoke_ring.wgsl");
const SMOKE_RING_UI_SHADER_SRC: &str = include_str!("../assets/shaders/smoke_ring_ui.wgsl");

pub fn init_smoke_ring_shader(shaders: &mut Assets<Shader>) {
    let _ = shaders.insert(
        SMOKE_RING_SHADER_HANDLE.id(),
        Shader::from_wgsl(SMOKE_RING_SHADER_SRC, "smoke_ring.wgsl"),
    );
    let _ = shaders.insert(
        SMOKE_RING_UI_SHADER_HANDLE.id(),
        Shader::from_wgsl(SMOKE_RING_UI_SHADER_SRC, "smoke_ring_ui.wgsl"),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::sprite_render::Material2d;

    #[test]
    fn smoke_ring_material_blends_alpha() {
        let material = SmokeRingMaterial::default();
        assert!(matches!(
            Material2d::alpha_mode(&material),
            AlphaMode2d::Blend
        ));
    }

    #[test]
    fn smoke_ring_shader_is_registered() {
        let mut shaders = Assets::<Shader>::default();
        init_smoke_ring_shader(&mut shaders);
        assert!(shaders.get(&SMOKE_RING_SHADER_HANDLE).is_some());
        assert!(shaders.get(&SMOKE_RING_UI_SHADER_HANDLE).is_some());
    }
}
