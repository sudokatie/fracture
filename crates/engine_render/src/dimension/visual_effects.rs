//! Dimension visual effects.
//!
//! Provides fog, lighting, and color tinting for each dimension.

use engine_physics::dimension::Dimension;

/// Visual effects handler for dimensions.
#[derive(Clone, Debug, Default)]
pub struct DimensionVisuals;

impl DimensionVisuals {
    /// Create a new dimension visuals handler.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Get the fog color for a dimension.
    ///
    /// Returns RGBA color values (0.0 to 1.0).
    #[must_use]
    pub fn get_fog_color(&self, dim: Dimension) -> [f32; 4] {
        match dim {
            Dimension::Prime => [0.7, 0.85, 1.0, 1.0],     // Light blue sky
            Dimension::Inverted => [0.9, 0.4, 0.2, 1.0],  // Orange-red heat haze
            Dimension::Void => [0.3, 0.3, 0.35, 1.0],     // Gray void mist
            Dimension::Nexus => [0.5, 0.3, 0.8, 1.0],     // Prismatic purple
        }
    }

    /// Get the fog density for a dimension.
    ///
    /// Returns 0.0 (clear) to 1.0 (opaque).
    #[must_use]
    pub fn get_fog_density(&self, dim: Dimension) -> f32 {
        match dim {
            Dimension::Prime => 0.0,
            Dimension::Inverted => 0.3,
            Dimension::Void => 0.9,
            Dimension::Nexus => 0.5,
        }
    }

    /// Get the ambient light level for a dimension.
    ///
    /// Returns 0.0 (dark) to 1.0 (full brightness).
    #[must_use]
    pub fn get_light_level(&self, dim: Dimension) -> f32 {
        match dim {
            Dimension::Prime => 1.0,
            Dimension::Inverted => 0.3,
            Dimension::Void => 0.05,
            Dimension::Nexus => 0.6,
        }
    }

    /// Get the color tint for a dimension.
    ///
    /// Returns RGB color multipliers (0.0 to 1.0).
    #[must_use]
    pub fn get_color_tint(&self, dim: Dimension) -> [f32; 3] {
        match dim {
            Dimension::Prime => [1.0, 1.0, 1.0],       // No tint (neutral)
            Dimension::Inverted => [1.2, 0.8, 0.6],   // Warm orange tint
            Dimension::Void => [0.6, 0.6, 0.8],       // Cool blue-gray tint
            Dimension::Nexus => [0.9, 0.7, 1.1],      // Purple tint
        }
    }

    /// Get the sky color for a dimension.
    ///
    /// Returns RGB color values (0.0 to 1.0).
    #[must_use]
    pub fn get_sky_color(&self, dim: Dimension) -> [f32; 3] {
        match dim {
            Dimension::Prime => [0.4, 0.6, 0.9],       // Clear blue sky
            Dimension::Inverted => [0.7, 0.3, 0.1],   // Burning orange sky
            Dimension::Void => [0.05, 0.05, 0.1],     // Near-black void
            Dimension::Nexus => [0.3, 0.2, 0.5],      // Deep purple
        }
    }

    /// Get the horizon color for a dimension.
    ///
    /// Returns RGB color values (0.0 to 1.0).
    #[must_use]
    pub fn get_horizon_color(&self, dim: Dimension) -> [f32; 3] {
        match dim {
            Dimension::Prime => [0.8, 0.85, 0.95],    // Light horizon
            Dimension::Inverted => [0.9, 0.5, 0.2],   // Orange glow
            Dimension::Void => [0.1, 0.1, 0.15],      // Dark gray
            Dimension::Nexus => [0.5, 0.3, 0.6],      // Purple haze
        }
    }

    /// Check if stars should be visible in this dimension.
    #[must_use]
    pub fn has_stars(&self, dim: Dimension) -> bool {
        matches!(dim, Dimension::Void | Dimension::Nexus)
    }

    /// Get the star brightness for a dimension.
    #[must_use]
    pub fn get_star_brightness(&self, dim: Dimension) -> f32 {
        match dim {
            Dimension::Prime => 0.0,       // No stars
            Dimension::Inverted => 0.0,    // Too bright for stars
            Dimension::Void => 0.9,        // Very bright stars
            Dimension::Nexus => 0.5,       // Dim stars
        }
    }

    /// Get the ambient occlusion strength for a dimension.
    #[must_use]
    pub fn get_ao_strength(&self, dim: Dimension) -> f32 {
        match dim {
            Dimension::Prime => 0.5,
            Dimension::Inverted => 0.3,    // Less AO due to heat glow
            Dimension::Void => 0.8,        // Strong shadows
            Dimension::Nexus => 0.6,
        }
    }

    /// Get the bloom intensity for a dimension.
    #[must_use]
    pub fn get_bloom_intensity(&self, dim: Dimension) -> f32 {
        match dim {
            Dimension::Prime => 0.3,
            Dimension::Inverted => 0.7,    // Heat bloom
            Dimension::Void => 0.1,        // Minimal bloom
            Dimension::Nexus => 0.5,       // Moderate bloom
        }
    }
}

/// Get fog color for a dimension (standalone function).
#[must_use]
pub fn get_fog_color(dim: Dimension) -> [f32; 4] {
    DimensionVisuals::new().get_fog_color(dim)
}

/// Get fog density for a dimension (standalone function).
#[must_use]
pub fn get_fog_density(dim: Dimension) -> f32 {
    DimensionVisuals::new().get_fog_density(dim)
}

/// Get light level for a dimension (standalone function).
#[must_use]
pub fn get_light_level(dim: Dimension) -> f32 {
    DimensionVisuals::new().get_light_level(dim)
}

/// Get color tint for a dimension (standalone function).
#[must_use]
pub fn get_color_tint(dim: Dimension) -> [f32; 3] {
    DimensionVisuals::new().get_color_tint(dim)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimension_visuals_fog_color() {
        let visuals = DimensionVisuals::new();

        let prime = visuals.get_fog_color(Dimension::Prime);
        assert!((prime[0] - 0.7).abs() < f32::EPSILON);

        let inverted = visuals.get_fog_color(Dimension::Inverted);
        assert!((inverted[0] - 0.9).abs() < f32::EPSILON);

        let void = visuals.get_fog_color(Dimension::Void);
        assert!((void[0] - 0.3).abs() < f32::EPSILON);

        let nexus = visuals.get_fog_color(Dimension::Nexus);
        assert!((nexus[0] - 0.5).abs() < f32::EPSILON);
        assert!((nexus[1] - 0.3).abs() < f32::EPSILON);
        assert!((nexus[2] - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn test_dimension_visuals_fog_density() {
        let visuals = DimensionVisuals::new();

        assert!((visuals.get_fog_density(Dimension::Prime) - 0.0).abs() < f32::EPSILON);
        assert!((visuals.get_fog_density(Dimension::Inverted) - 0.3).abs() < f32::EPSILON);
        assert!((visuals.get_fog_density(Dimension::Void) - 0.9).abs() < f32::EPSILON);
        assert!((visuals.get_fog_density(Dimension::Nexus) - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_dimension_visuals_light_level() {
        let visuals = DimensionVisuals::new();

        assert!((visuals.get_light_level(Dimension::Prime) - 1.0).abs() < f32::EPSILON);
        assert!((visuals.get_light_level(Dimension::Inverted) - 0.3).abs() < f32::EPSILON);
        assert!((visuals.get_light_level(Dimension::Void) - 0.05).abs() < f32::EPSILON);
        assert!((visuals.get_light_level(Dimension::Nexus) - 0.6).abs() < f32::EPSILON);
    }

    #[test]
    fn test_dimension_visuals_color_tint() {
        let visuals = DimensionVisuals::new();

        let prime = visuals.get_color_tint(Dimension::Prime);
        assert!((prime[0] - 1.0).abs() < f32::EPSILON);
        assert!((prime[1] - 1.0).abs() < f32::EPSILON);
        assert!((prime[2] - 1.0).abs() < f32::EPSILON);

        let inverted = visuals.get_color_tint(Dimension::Inverted);
        assert!(inverted[0] > 1.0); // Warm tint boosted
    }

    #[test]
    fn test_dimension_visuals_sky_color() {
        let visuals = DimensionVisuals::new();

        let prime = visuals.get_sky_color(Dimension::Prime);
        assert!(prime[2] > prime[0]); // Blue dominant

        let void = visuals.get_sky_color(Dimension::Void);
        assert!(void[0] < 0.1); // Very dark
    }

    #[test]
    fn test_dimension_visuals_has_stars() {
        let visuals = DimensionVisuals::new();

        assert!(!visuals.has_stars(Dimension::Prime));
        assert!(!visuals.has_stars(Dimension::Inverted));
        assert!(visuals.has_stars(Dimension::Void));
        assert!(visuals.has_stars(Dimension::Nexus));
    }

    #[test]
    fn test_dimension_visuals_star_brightness() {
        let visuals = DimensionVisuals::new();

        assert!((visuals.get_star_brightness(Dimension::Prime) - 0.0).abs() < f32::EPSILON);
        assert!(visuals.get_star_brightness(Dimension::Void) > 0.5);
    }

    #[test]
    fn test_dimension_visuals_ao_strength() {
        let visuals = DimensionVisuals::new();

        assert!(visuals.get_ao_strength(Dimension::Void) > visuals.get_ao_strength(Dimension::Prime));
    }

    #[test]
    fn test_dimension_visuals_bloom_intensity() {
        let visuals = DimensionVisuals::new();

        assert!(visuals.get_bloom_intensity(Dimension::Inverted) > visuals.get_bloom_intensity(Dimension::Prime));
    }

    #[test]
    fn test_standalone_functions() {
        assert!((get_fog_density(Dimension::Prime) - 0.0).abs() < f32::EPSILON);
        assert!((get_light_level(Dimension::Prime) - 1.0).abs() < f32::EPSILON);

        let fog = get_fog_color(Dimension::Nexus);
        assert!((fog[0] - 0.5).abs() < f32::EPSILON);

        let tint = get_color_tint(Dimension::Prime);
        assert!((tint[0] - 1.0).abs() < f32::EPSILON);
    }
}
