//! Dimension transition visual effects.
//!
//! Handles visual effects for phase shifts and fractures.

use engine_physics::dimension::{Dimension, FractureType};

/// Visual effect data for a fracture event.
#[derive(Clone, Debug)]
pub struct FractureVisual {
    /// Ripple effect radius.
    pub ripple_radius: f32,
    /// Effect duration in seconds.
    pub duration: f32,
    /// Effect intensity (0.0 to 1.0).
    pub intensity: f32,
}

impl FractureVisual {
    /// Create a new fracture visual.
    #[must_use]
    pub fn new(ripple_radius: f32, duration: f32, intensity: f32) -> Self {
        Self {
            ripple_radius,
            duration,
            intensity: intensity.clamp(0.0, 1.0),
        }
    }
}

/// Phase shift visual effects handler.
#[derive(Clone, Debug, Default)]
pub struct PhaseShiftVisuals;

impl PhaseShiftVisuals {
    /// Create a new phase shift visuals handler.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Get the duration of a phase shift effect.
    ///
    /// Returns duration in seconds (0.5 to 2.0).
    #[must_use]
    pub fn get_shift_duration(&self) -> f32 {
        1.0 // Base duration
    }

    /// Get the shift duration based on dimensions.
    #[must_use]
    pub fn get_shift_duration_for(&self, _from: Dimension, to: Dimension) -> f32 {
        match to {
            Dimension::Prime => 0.5,     // Quick return to Prime
            Dimension::Inverted => 1.0,  // Standard
            Dimension::Void => 1.5,      // Slower, more disorienting
            Dimension::Nexus => 2.0,     // Longest, most dramatic
        }
    }

    /// Get the color of the phase shift effect.
    ///
    /// Returns RGBA color values (0.0 to 1.0).
    #[must_use]
    pub fn get_shift_color(&self, from: Dimension, to: Dimension) -> [f32; 4] {
        // Blend between dimension colors
        let from_color = self.dimension_base_color(from);
        let to_color = self.dimension_base_color(to);

        // Return a blend that transitions from -> to
        [
            (from_color[0] + to_color[0]) / 2.0,
            (from_color[1] + to_color[1]) / 2.0,
            (from_color[2] + to_color[2]) / 2.0,
            0.8, // Semi-transparent
        ]
    }

    /// Get the base color for a dimension.
    fn dimension_base_color(&self, dim: Dimension) -> [f32; 3] {
        match dim {
            Dimension::Prime => [0.9, 0.95, 1.0],     // White-blue
            Dimension::Inverted => [1.0, 0.5, 0.2],   // Orange
            Dimension::Void => [0.2, 0.2, 0.3],       // Dark gray
            Dimension::Nexus => [0.6, 0.3, 0.9],      // Purple
        }
    }

    /// Get the distortion strength during a shift.
    #[must_use]
    pub fn get_distortion_strength(&self, progress: f32) -> f32 {
        // Peak distortion at 50% progress
        let centered = (progress - 0.5).abs();
        (1.0 - centered * 2.0).max(0.0)
    }

    /// Get the screen fade amount during a shift.
    #[must_use]
    pub fn get_fade_amount(&self, progress: f32) -> f32 {
        // Fade out then in
        if progress < 0.5 {
            progress * 2.0
        } else {
            (1.0 - progress) * 2.0
        }
    }

    /// Check if particle effects should spawn during shift.
    #[must_use]
    pub fn should_spawn_particles(&self, progress: f32) -> bool {
        // Spawn particles in the middle of the effect
        (0.3..0.7).contains(&progress)
    }
}

/// Fracture visual effects handler.
#[derive(Clone, Debug, Default)]
pub struct FractureVisuals;

impl FractureVisuals {
    /// Create a new fracture visuals handler.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Get the visual effect for a fracture type.
    #[must_use]
    pub fn get_fracture_effect(&self, ftype: FractureType) -> FractureVisual {
        match ftype {
            FractureType::Micro => FractureVisual::new(2.0, 0.5, 0.3),
            FractureType::Meso => FractureVisual::new(5.0, 1.0, 0.5),
            FractureType::Macro => FractureVisual::new(10.0, 2.0, 0.7),
            FractureType::Cascade => FractureVisual::new(15.0, 3.0, 1.0),
        }
    }

    /// Get the crack pattern density for a fracture type.
    #[must_use]
    pub fn get_crack_density(&self, ftype: FractureType) -> f32 {
        match ftype {
            FractureType::Micro => 0.1,
            FractureType::Meso => 0.3,
            FractureType::Macro => 0.6,
            FractureType::Cascade => 1.0,
        }
    }

    /// Get the glow color for a fracture type.
    #[must_use]
    pub fn get_fracture_glow_color(&self, ftype: FractureType) -> [f32; 4] {
        match ftype {
            FractureType::Micro => [0.5, 0.8, 1.0, 0.3],   // Light blue
            FractureType::Meso => [0.8, 0.5, 1.0, 0.5],    // Purple
            FractureType::Macro => [1.0, 0.3, 0.3, 0.7],   // Red
            FractureType::Cascade => [1.0, 0.9, 0.3, 1.0], // Golden yellow
        }
    }

    /// Get the sound radius for a fracture type.
    #[must_use]
    pub fn get_sound_radius(&self, ftype: FractureType) -> f32 {
        match ftype {
            FractureType::Micro => 5.0,
            FractureType::Meso => 15.0,
            FractureType::Macro => 30.0,
            FractureType::Cascade => 50.0,
        }
    }

    /// Check if screen shake should occur.
    #[must_use]
    pub fn has_screen_shake(&self, ftype: FractureType) -> bool {
        !matches!(ftype, FractureType::Micro)
    }

    /// Get the screen shake intensity.
    #[must_use]
    pub fn get_shake_intensity(&self, ftype: FractureType) -> f32 {
        match ftype {
            FractureType::Micro => 0.0,
            FractureType::Meso => 0.2,
            FractureType::Macro => 0.5,
            FractureType::Cascade => 1.0,
        }
    }

    /// Get ripple effect parameters.
    #[must_use]
    pub fn get_ripple_params(&self, ftype: FractureType) -> (f32, f32, f32) {
        // (speed, wavelength, amplitude)
        match ftype {
            FractureType::Micro => (10.0, 0.5, 0.1),
            FractureType::Meso => (15.0, 1.0, 0.2),
            FractureType::Macro => (20.0, 1.5, 0.4),
            FractureType::Cascade => (25.0, 2.0, 0.6),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // PhaseShiftVisuals tests
    #[test]
    fn test_phase_shift_duration() {
        let visuals = PhaseShiftVisuals::new();

        assert!((visuals.get_shift_duration() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_phase_shift_duration_for_dimensions() {
        let visuals = PhaseShiftVisuals::new();

        assert!(visuals.get_shift_duration_for(Dimension::Void, Dimension::Prime) < 1.0);
        assert!(visuals.get_shift_duration_for(Dimension::Prime, Dimension::Nexus) > 1.0);
    }

    #[test]
    fn test_phase_shift_color() {
        let visuals = PhaseShiftVisuals::new();

        let color = visuals.get_shift_color(Dimension::Prime, Dimension::Void);
        assert_eq!(color.len(), 4);
        assert!(color[3] > 0.0); // Has alpha
    }

    #[test]
    fn test_phase_shift_distortion() {
        let visuals = PhaseShiftVisuals::new();

        // Peak at 50%
        assert!(visuals.get_distortion_strength(0.5) > visuals.get_distortion_strength(0.0));
        assert!(visuals.get_distortion_strength(0.5) > visuals.get_distortion_strength(1.0));
    }

    #[test]
    fn test_phase_shift_fade() {
        let visuals = PhaseShiftVisuals::new();

        // Fade out then in
        assert!(visuals.get_fade_amount(0.0) < visuals.get_fade_amount(0.5));
        assert!(visuals.get_fade_amount(1.0) < visuals.get_fade_amount(0.5));
    }

    #[test]
    fn test_phase_shift_particles() {
        let visuals = PhaseShiftVisuals::new();

        assert!(!visuals.should_spawn_particles(0.1));
        assert!(visuals.should_spawn_particles(0.5));
        assert!(!visuals.should_spawn_particles(0.9));
    }

    // FractureVisuals tests
    #[test]
    fn test_fracture_visual_new() {
        let visual = FractureVisual::new(5.0, 1.0, 0.5);

        assert!((visual.ripple_radius - 5.0).abs() < f32::EPSILON);
        assert!((visual.duration - 1.0).abs() < f32::EPSILON);
        assert!((visual.intensity - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_fracture_visual_intensity_clamped() {
        let visual = FractureVisual::new(5.0, 1.0, 1.5);
        assert!((visual.intensity - 1.0).abs() < f32::EPSILON);

        let visual = FractureVisual::new(5.0, 1.0, -0.5);
        assert!((visual.intensity - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_fracture_effect_scaling() {
        let visuals = FractureVisuals::new();

        let micro = visuals.get_fracture_effect(FractureType::Micro);
        let cascade = visuals.get_fracture_effect(FractureType::Cascade);

        assert!(cascade.ripple_radius > micro.ripple_radius);
        assert!(cascade.duration > micro.duration);
        assert!(cascade.intensity > micro.intensity);
    }

    #[test]
    fn test_fracture_crack_density() {
        let visuals = FractureVisuals::new();

        assert!(visuals.get_crack_density(FractureType::Micro) < visuals.get_crack_density(FractureType::Cascade));
    }

    #[test]
    fn test_fracture_glow_color() {
        let visuals = FractureVisuals::new();

        let color = visuals.get_fracture_glow_color(FractureType::Micro);
        assert_eq!(color.len(), 4);

        // Cascade should be more intense (higher alpha)
        let micro_alpha = visuals.get_fracture_glow_color(FractureType::Micro)[3];
        let cascade_alpha = visuals.get_fracture_glow_color(FractureType::Cascade)[3];
        assert!(cascade_alpha > micro_alpha);
    }

    #[test]
    fn test_fracture_sound_radius() {
        let visuals = FractureVisuals::new();

        assert!(visuals.get_sound_radius(FractureType::Cascade) > visuals.get_sound_radius(FractureType::Micro));
    }

    #[test]
    fn test_fracture_screen_shake() {
        let visuals = FractureVisuals::new();

        assert!(!visuals.has_screen_shake(FractureType::Micro));
        assert!(visuals.has_screen_shake(FractureType::Meso));
        assert!(visuals.has_screen_shake(FractureType::Macro));
        assert!(visuals.has_screen_shake(FractureType::Cascade));
    }

    #[test]
    fn test_fracture_shake_intensity() {
        let visuals = FractureVisuals::new();

        assert!((visuals.get_shake_intensity(FractureType::Micro) - 0.0).abs() < f32::EPSILON);
        assert!(visuals.get_shake_intensity(FractureType::Cascade) > visuals.get_shake_intensity(FractureType::Meso));
    }

    #[test]
    fn test_fracture_ripple_params() {
        let visuals = FractureVisuals::new();

        let (speed, wavelength, amplitude) = visuals.get_ripple_params(FractureType::Micro);
        assert!(speed > 0.0);
        assert!(wavelength > 0.0);
        assert!(amplitude > 0.0);

        let (cascade_speed, _, _) = visuals.get_ripple_params(FractureType::Cascade);
        assert!(cascade_speed > speed);
    }
}
