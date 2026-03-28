#![allow(non_snake_case)]

//! bevy_mistUI: community-facing mist / smoke-ring UI crate incubated inside ROPE.
//!
//! The public surface stays intentionally small:
//! - [`SmokeBorder`] for procedural ring styling
//! - [`SmokeRingMaterial`] / [`SmokeRingParams`] for the GPU ring material
//! - [`SmokeRingPlugin`] for registration and automatic size syncing
//! - [`SmokeRingPadding`] / [`SmokeRingBundle`] for common attachment cases

mod border;
mod material;
mod plugin;

pub use border::SmokeBorder;
pub use material::{
    init_smoke_ring_shader, SmokeRingMaterial, SmokeRingParams, SMOKE_RING_SHADER_HANDLE,
};
pub use plugin::{SmokeRingBundle, SmokeRingPadding, SmokeRingPlugin, SmokeRingSettings};
