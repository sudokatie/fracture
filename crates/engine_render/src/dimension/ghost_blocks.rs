//! Ghost block rendering for cross-dimension visibility.
//!
//! Renders semi-transparent "ghost" versions of blocks from adjacent dimensions,
//! allowing players to see nearby dimension boundaries.

use engine_physics::dimension::Dimension;

/// Maximum distance for ghost block rendering.
pub const MAX_GHOST_DISTANCE: f32 = 8.0;

/// Base alpha for ghost blocks.
pub const BASE_GHOST_ALPHA: f32 = 0.3;

/// Ghost block renderer.
#[derive(Clone, Debug, Default)]
pub struct GhostBlockRenderer;

impl GhostBlockRenderer {
    /// Create a new ghost block renderer.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Get the alpha value for a ghost block based on distance.
    ///
    /// Returns 0.0 to 0.3 based on distance (closer = more visible).
    #[must_use]
    pub fn get_ghost_alpha(&self, distance: f32) -> f32 {
        if distance <= 0.0 {
            return BASE_GHOST_ALPHA;
        }

        if distance >= MAX_GHOST_DISTANCE {
            return 0.0;
        }

        // Linear falloff
        let t = distance / MAX_GHOST_DISTANCE;
        BASE_GHOST_ALPHA * (1.0 - t)
    }

    /// Check if ghost blocks should be rendered between two dimensions.
    #[must_use]
    pub fn should_render_ghost(&self, dim: Dimension, neighbor_dim: Dimension) -> bool {
        // Only render ghosts across dimension boundaries
        dim != neighbor_dim
    }

    /// Get the tint color for ghost blocks from a dimension.
    #[must_use]
    pub fn get_ghost_tint(&self, from_dim: Dimension) -> [f32; 4] {
        match from_dim {
            Dimension::Prime => [0.9, 0.95, 1.0, 1.0],    // White-blue tint
            Dimension::Inverted => [1.0, 0.6, 0.3, 1.0],  // Orange tint
            Dimension::Void => [0.4, 0.4, 0.5, 1.0],      // Gray-blue tint
            Dimension::Nexus => [0.7, 0.4, 1.0, 1.0],     // Purple tint
        }
    }

    /// Get the outline color for ghost blocks.
    #[must_use]
    pub fn get_ghost_outline_color(&self, from_dim: Dimension) -> [f32; 4] {
        match from_dim {
            Dimension::Prime => [0.5, 0.7, 1.0, 0.5],     // Light blue outline
            Dimension::Inverted => [1.0, 0.4, 0.1, 0.5],  // Orange outline
            Dimension::Void => [0.2, 0.2, 0.3, 0.5],      // Dark outline
            Dimension::Nexus => [0.6, 0.2, 0.9, 0.5],     // Purple outline
        }
    }

    /// Check if a ghost block should have a glowing effect.
    #[must_use]
    pub fn should_glow(&self, dim: Dimension) -> bool {
        matches!(dim, Dimension::Inverted | Dimension::Nexus)
    }

    /// Get the glow intensity for ghost blocks.
    #[must_use]
    pub fn get_glow_intensity(&self, dim: Dimension) -> f32 {
        match dim {
            Dimension::Prime => 0.0,
            Dimension::Inverted => 0.4,
            Dimension::Void => 0.1,
            Dimension::Nexus => 0.6,
        }
    }

    /// Get the shimmer frequency for animated ghost blocks.
    #[must_use]
    pub fn get_shimmer_frequency(&self, dim: Dimension) -> f32 {
        match dim {
            Dimension::Prime => 0.0,   // No shimmer
            Dimension::Inverted => 1.0,
            Dimension::Void => 0.3,    // Slow pulse
            Dimension::Nexus => 2.0,   // Fast shimmer
        }
    }

    /// Calculate the final alpha for a ghost block.
    #[must_use]
    pub fn calculate_final_alpha(
        &self,
        distance: f32,
        from_dim: Dimension,
        time: f32,
    ) -> f32 {
        let base = self.get_ghost_alpha(distance);
        let shimmer_freq = self.get_shimmer_frequency(from_dim);

        if shimmer_freq > 0.0 {
            // Add subtle shimmer
            let shimmer = (time * shimmer_freq * std::f32::consts::TAU).sin() * 0.1 + 1.0;
            (base * shimmer).clamp(0.0, BASE_GHOST_ALPHA)
        } else {
            base
        }
    }

    /// Check if blocks should be rendered as wireframe ghosts.
    #[must_use]
    pub fn use_wireframe(&self, dim: Dimension) -> bool {
        matches!(dim, Dimension::Void)
    }

    /// Get the maximum render distance for ghost blocks in a dimension.
    #[must_use]
    pub fn get_render_distance(&self, dim: Dimension) -> f32 {
        match dim {
            Dimension::Prime => MAX_GHOST_DISTANCE,
            Dimension::Inverted => MAX_GHOST_DISTANCE * 0.8, // Heat haze reduces visibility
            Dimension::Void => MAX_GHOST_DISTANCE * 1.5,     // Can see further in void
            Dimension::Nexus => MAX_GHOST_DISTANCE * 1.2,
        }
    }
}

/// Get ghost alpha for a distance (standalone function).
#[must_use]
pub fn get_ghost_alpha(distance: f32) -> f32 {
    GhostBlockRenderer::new().get_ghost_alpha(distance)
}

/// Check if ghost should render between dimensions (standalone function).
#[must_use]
pub fn should_render_ghost(dim: Dimension, neighbor_dim: Dimension) -> bool {
    GhostBlockRenderer::new().should_render_ghost(dim, neighbor_dim)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ghost_alpha_at_zero() {
        let renderer = GhostBlockRenderer::new();
        assert!((renderer.get_ghost_alpha(0.0) - BASE_GHOST_ALPHA).abs() < f32::EPSILON);
    }

    #[test]
    fn test_ghost_alpha_at_max_distance() {
        let renderer = GhostBlockRenderer::new();
        assert!((renderer.get_ghost_alpha(MAX_GHOST_DISTANCE) - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_ghost_alpha_beyond_max() {
        let renderer = GhostBlockRenderer::new();
        assert!((renderer.get_ghost_alpha(100.0) - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_ghost_alpha_midpoint() {
        let renderer = GhostBlockRenderer::new();
        let mid_alpha = renderer.get_ghost_alpha(MAX_GHOST_DISTANCE / 2.0);
        assert!(mid_alpha > 0.0);
        assert!(mid_alpha < BASE_GHOST_ALPHA);
    }

    #[test]
    fn test_should_render_ghost_different_dims() {
        let renderer = GhostBlockRenderer::new();
        assert!(renderer.should_render_ghost(Dimension::Prime, Dimension::Void));
        assert!(renderer.should_render_ghost(Dimension::Inverted, Dimension::Nexus));
    }

    #[test]
    fn test_should_render_ghost_same_dim() {
        let renderer = GhostBlockRenderer::new();
        assert!(!renderer.should_render_ghost(Dimension::Prime, Dimension::Prime));
        assert!(!renderer.should_render_ghost(Dimension::Void, Dimension::Void));
    }

    #[test]
    fn test_ghost_tint_colors() {
        let renderer = GhostBlockRenderer::new();

        let prime = renderer.get_ghost_tint(Dimension::Prime);
        assert_eq!(prime.len(), 4);

        let inverted = renderer.get_ghost_tint(Dimension::Inverted);
        assert!(inverted[0] > inverted[2]); // More red than blue
    }

    #[test]
    fn test_ghost_outline_colors() {
        let renderer = GhostBlockRenderer::new();

        let color = renderer.get_ghost_outline_color(Dimension::Nexus);
        assert_eq!(color.len(), 4);
        assert!(color[2] > color[0]); // More blue/purple than red
    }

    #[test]
    fn test_should_glow() {
        let renderer = GhostBlockRenderer::new();

        assert!(!renderer.should_glow(Dimension::Prime));
        assert!(renderer.should_glow(Dimension::Inverted));
        assert!(!renderer.should_glow(Dimension::Void));
        assert!(renderer.should_glow(Dimension::Nexus));
    }

    #[test]
    fn test_glow_intensity() {
        let renderer = GhostBlockRenderer::new();

        assert!((renderer.get_glow_intensity(Dimension::Prime) - 0.0).abs() < f32::EPSILON);
        assert!(renderer.get_glow_intensity(Dimension::Nexus) > renderer.get_glow_intensity(Dimension::Inverted));
    }

    #[test]
    fn test_shimmer_frequency() {
        let renderer = GhostBlockRenderer::new();

        assert!((renderer.get_shimmer_frequency(Dimension::Prime) - 0.0).abs() < f32::EPSILON);
        assert!(renderer.get_shimmer_frequency(Dimension::Nexus) > 0.0);
    }

    #[test]
    fn test_calculate_final_alpha() {
        let renderer = GhostBlockRenderer::new();

        // No shimmer in Prime
        let alpha1 = renderer.calculate_final_alpha(2.0, Dimension::Prime, 0.0);
        let alpha2 = renderer.calculate_final_alpha(2.0, Dimension::Prime, 1.0);
        assert!((alpha1 - alpha2).abs() < f32::EPSILON);

        // Shimmer in Nexus
        // Note: may or may not be different depending on time value
        let _alpha3 = renderer.calculate_final_alpha(2.0, Dimension::Nexus, 0.0);
    }

    #[test]
    fn test_use_wireframe() {
        let renderer = GhostBlockRenderer::new();

        assert!(!renderer.use_wireframe(Dimension::Prime));
        assert!(!renderer.use_wireframe(Dimension::Inverted));
        assert!(renderer.use_wireframe(Dimension::Void));
        assert!(!renderer.use_wireframe(Dimension::Nexus));
    }

    #[test]
    fn test_render_distance() {
        let renderer = GhostBlockRenderer::new();

        assert!(renderer.get_render_distance(Dimension::Void) > renderer.get_render_distance(Dimension::Prime));
        assert!(renderer.get_render_distance(Dimension::Inverted) < MAX_GHOST_DISTANCE);
    }

    #[test]
    fn test_standalone_functions() {
        assert!((get_ghost_alpha(0.0) - BASE_GHOST_ALPHA).abs() < f32::EPSILON);
        assert!(should_render_ghost(Dimension::Prime, Dimension::Void));
        assert!(!should_render_ghost(Dimension::Prime, Dimension::Prime));
    }
}
