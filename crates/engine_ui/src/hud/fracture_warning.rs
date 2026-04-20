//! Fracture warning HUD element.
//!
//! Displays warnings when fractures are imminent.

/// Display state for fracture warnings.
#[derive(Clone, Debug)]
pub struct FractureWarningDisplay {
    /// Current fracture probability (0.0 to 1.0).
    probability: f32,
    /// Time until impact (in seconds), if known.
    time_to_impact: Option<f32>,
    /// Whether warning is currently active.
    active: bool,
}

impl FractureWarningDisplay {
    /// Create a new fracture warning display.
    #[must_use]
    pub fn new() -> Self {
        Self {
            probability: 0.0,
            time_to_impact: None,
            active: false,
        }
    }

    /// Update the display with new warning data.
    pub fn update(&mut self, probability: f32, time_to_impact: Option<f32>) {
        self.probability = probability.clamp(0.0, 1.0);
        self.time_to_impact = time_to_impact;
        self.active = self.probability > 0.1;
    }

    /// Clear the warning.
    pub fn clear(&mut self) {
        self.probability = 0.0;
        self.time_to_impact = None;
        self.active = false;
    }

    /// Get the current probability.
    #[must_use]
    pub fn probability(&self) -> f32 {
        self.probability
    }

    /// Get the time to impact, if known.
    #[must_use]
    pub fn time_to_impact(&self) -> Option<f32> {
        self.time_to_impact
    }

    /// Check if a fracture is imminent (probability > 70% or time < 5s).
    #[must_use]
    pub fn is_imminent(&self) -> bool {
        self.probability > 0.7 || self.time_to_impact.map_or(false, |t| t < 5.0)
    }

    /// Check if warning should be displayed.
    #[must_use]
    pub fn should_display(&self) -> bool {
        self.active
    }

    /// Get the warning level (0-3).
    #[must_use]
    pub fn warning_level(&self) -> u8 {
        if self.probability < 0.25 {
            0 // No warning
        } else if self.probability < 0.5 {
            1 // Low warning
        } else if self.probability < 0.75 {
            2 // Medium warning
        } else {
            3 // High warning
        }
    }

    /// Get the warning color based on severity.
    ///
    /// Returns RGB color values (0.0 to 1.0).
    #[must_use]
    pub fn color(&self) -> [f32; 3] {
        match self.warning_level() {
            0 => [0.5, 0.5, 0.5],  // Gray - none
            1 => [1.0, 1.0, 0.3],  // Yellow - low
            2 => [1.0, 0.6, 0.2],  // Orange - medium
            _ => [1.0, 0.2, 0.2],  // Red - high
        }
    }

    /// Get the warning text.
    #[must_use]
    pub fn warning_text(&self) -> &'static str {
        match self.warning_level() {
            0 => "",
            1 => "Dimensional instability detected",
            2 => "Fracture forming nearby",
            _ => "IMMINENT FRACTURE - SEEK SHELTER",
        }
    }

    /// Get formatted time remaining string.
    #[must_use]
    pub fn time_text(&self) -> Option<String> {
        self.time_to_impact.map(|t| {
            if t < 1.0 {
                "< 1s".to_string()
            } else {
                format!("{:.0}s", t)
            }
        })
    }

    /// Get the pulse rate for animation (Hz).
    #[must_use]
    pub fn pulse_rate(&self) -> f32 {
        match self.warning_level() {
            0 => 0.0,
            1 => 0.5,
            2 => 1.0,
            _ => 2.0,
        }
    }
}

impl Default for FractureWarningDisplay {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fracture_warning_new() {
        let warning = FractureWarningDisplay::new();
        assert!((warning.probability() - 0.0).abs() < f32::EPSILON);
        assert!(warning.time_to_impact().is_none());
        assert!(!warning.should_display());
    }

    #[test]
    fn test_fracture_warning_update() {
        let mut warning = FractureWarningDisplay::new();
        warning.update(0.5, Some(10.0));

        assert!((warning.probability() - 0.5).abs() < f32::EPSILON);
        assert_eq!(warning.time_to_impact(), Some(10.0));
        assert!(warning.should_display());
    }

    #[test]
    fn test_fracture_warning_clear() {
        let mut warning = FractureWarningDisplay::new();
        warning.update(0.8, Some(5.0));
        warning.clear();

        assert!((warning.probability() - 0.0).abs() < f32::EPSILON);
        assert!(!warning.should_display());
    }

    #[test]
    fn test_fracture_warning_imminent() {
        let mut warning = FractureWarningDisplay::new();

        // Not imminent
        warning.update(0.5, Some(10.0));
        assert!(!warning.is_imminent());

        // Imminent by probability
        warning.update(0.8, None);
        assert!(warning.is_imminent());

        // Imminent by time
        warning.update(0.3, Some(3.0));
        assert!(warning.is_imminent());
    }

    #[test]
    fn test_fracture_warning_level() {
        let mut warning = FractureWarningDisplay::new();

        warning.update(0.1, None);
        assert_eq!(warning.warning_level(), 0);

        warning.update(0.3, None);
        assert_eq!(warning.warning_level(), 1);

        warning.update(0.6, None);
        assert_eq!(warning.warning_level(), 2);

        warning.update(0.9, None);
        assert_eq!(warning.warning_level(), 3);
    }

    #[test]
    fn test_fracture_warning_color() {
        let mut warning = FractureWarningDisplay::new();

        warning.update(0.9, None);
        let color = warning.color();
        assert!(color[0] > color[1]); // Red dominant for high warning
    }

    #[test]
    fn test_fracture_warning_text() {
        let mut warning = FractureWarningDisplay::new();

        warning.update(0.9, None);
        assert!(warning.warning_text().contains("IMMINENT"));
    }

    #[test]
    fn test_fracture_warning_time_text() {
        let mut warning = FractureWarningDisplay::new();

        warning.update(0.5, Some(15.0));
        assert_eq!(warning.time_text(), Some("15s".to_string()));

        warning.update(0.5, Some(0.5));
        assert_eq!(warning.time_text(), Some("< 1s".to_string()));

        warning.update(0.5, None);
        assert!(warning.time_text().is_none());
    }

    #[test]
    fn test_fracture_warning_pulse_rate() {
        let mut warning = FractureWarningDisplay::new();

        warning.update(0.1, None);
        assert!((warning.pulse_rate() - 0.0).abs() < f32::EPSILON);

        warning.update(0.9, None);
        assert!(warning.pulse_rate() > 1.0);
    }
}
