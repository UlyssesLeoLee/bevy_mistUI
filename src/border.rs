use bevy::prelude::*;

/// Component that turns a UI node's border into a procedural smoke ring.
#[derive(Component, Clone, Reflect, Debug, PartialEq)]
pub struct SmokeBorder {
    /// Base color of the smoke.
    pub color: Color,
    /// Width of the smoke ring relative to the element size.
    pub thickness: f32,
    /// Overall brightness/opacity multiplier.
    pub intensity: f32,
    /// Speed of the smoke flow animation.
    pub flow_speed: f32,
    /// Scale of the noise texture (affects detail density).
    pub noise_scale: f32,
    /// Softness of the smoke edges.
    pub softness: f32,
    /// Strength of the rhythmic pulsing effect.
    pub pulse_strength: f32,
    /// Color of the pulsing glow.
    pub pulse_color: Color,
}

impl Default for SmokeBorder {
    fn default() -> Self {
        Self {
            color: Color::srgb(0.9, 0.95, 1.0),
            thickness: 0.45,
            intensity: 1.8,
            flow_speed: 1.0,
            noise_scale: 28.0,
            softness: 0.15,
            pulse_strength: 0.45,
            pulse_color: Color::srgb(0.6, 0.8, 1.0),
        }
    }
}

impl SmokeBorder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
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

    pub fn neon_tube(primary: Color, secondary: Color, _seed: u32) -> Self {
        Self {
            color: primary,
            pulse_color: secondary,
            intensity: 1.5,
            thickness: 0.18,
            softness: 0.2,
            flow_speed: 1.0,
            noise_scale: 40.0,
            pulse_strength: 0.6,
        }
    }

    pub fn gaseous_idle(_seed: u64) -> Self {
        Self {
            color: Color::srgb(1.0, 1.0, 1.0),
            pulse_color: Color::srgb(0.92, 0.96, 1.0),
            intensity: 2.6,
            thickness: 0.1,
            softness: 0.4,
            flow_speed: 0.4,
            noise_scale: 36.0,
            pulse_strength: 0.15,
        }
    }

    pub fn disabled() -> Self {
        Self {
            intensity: 0.0,
            ..Default::default()
        }
    }

    pub fn particle_border(seed: u64) -> Self {
        Self::gaseous_thick_border(seed)
    }

    pub fn gaseous_thick_border(seed: u64) -> Self {
        let mut border = Self::gaseous_idle(seed);
        border.thickness = 0.35;
        border.intensity = 5.2;
        border.softness = 0.12;
        border.noise_scale = 58.0;
        border
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_border_presets_keep_expected_ordering() {
        let idle = SmokeBorder::gaseous_idle(0);
        let thick = SmokeBorder::gaseous_thick_border(1);

        assert!(idle.intensity > 0.0);
        assert!(idle.thickness > 0.0);
        assert!(thick.intensity > idle.intensity);
        assert!(thick.thickness > idle.thickness);
        assert_eq!(SmokeBorder::disabled().intensity, 0.0);
    }
}

