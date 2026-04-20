//! Sickness gauge HUD element.
//!
//! Displays the player's fracture sickness level.

/// Sickness severity levels (mirrored from survival module).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SicknessLevel {
    /// No sickness (0).
    None,
    /// Mild sickness (1-25).
    Mild,
    /// Moderate sickness (26-50).
    Moderate,
    /// Severe sickness (51-75).
    Severe,
    /// Critical sickness (76-100).
    Critical,
}

impl SicknessLevel {
    /// Get level from numeric value.
    #[must_use]
    pub fn from_value(value: f32) -> Self {
        match value {
            v if v <= 0.0 => SicknessLevel::None,
            v if v <= 25.0 => SicknessLevel::Mild,
            v if v <= 50.0 => SicknessLevel::Moderate,
            v if v <= 75.0 => SicknessLevel::Severe,
            _ => SicknessLevel::Critical,
        }
    }
}

/// Display state for the sickness gauge.
#[derive(Clone, Debug)]
pub struct SicknessGaugeDisplay {
    /// Current sickness level (0-100).
    level: f32,
    /// Current sickness enum.
    sickness_enum: SicknessLevel,
}

impl SicknessGaugeDisplay {
    /// Create a new sickness gauge display.
    #[must_use]
    pub fn new() -> Self {
        Self {
            level: 0.0,
            sickness_enum: SicknessLevel::None,
        }
    }

    /// Update the display with a new sickness level.
    pub fn update(&mut self, level: f32) {
        self.level = level.clamp(0.0, 100.0);
        self.sickness_enum = SicknessLevel::from_value(self.level);
    }

    /// Get the current sickness level (0-100).
    #[must_use]
    pub fn level(&self) -> f32 {
        self.level
    }

    /// Get the sickness as an enum.
    #[must_use]
    pub fn sickness_enum(&self) -> SicknessLevel {
        self.sickness_enum
    }

    /// Get the sickness as a percentage (0.0 to 1.0).
    #[must_use]
    pub fn percentage(&self) -> f32 {
        self.level / 100.0
    }

    /// Get the warning text for the current sickness level.
    #[must_use]
    pub fn warning_text(&self) -> Option<&'static str> {
        match self.sickness_enum {
            SicknessLevel::None => None,
            SicknessLevel::Mild => Some("Mild disorientation"),
            SicknessLevel::Moderate => Some("Vision impaired"),
            SicknessLevel::Severe => Some("Hallucinations occurring"),
            SicknessLevel::Critical => Some("CRITICAL - Return to Prime!"),
        }
    }

    /// Get the display color based on sickness level.
    ///
    /// Returns RGB color values (0.0 to 1.0).
    #[must_use]
    pub fn color(&self) -> [f32; 3] {
        match self.sickness_enum {
            SicknessLevel::None => [0.3, 0.9, 0.3],     // Green - healthy
            SicknessLevel::Mild => [0.8, 0.9, 0.3],    // Yellow-green
            SicknessLevel::Moderate => [1.0, 0.8, 0.2], // Yellow-orange
            SicknessLevel::Severe => [1.0, 0.5, 0.2],  // Orange
            SicknessLevel::Critical => [1.0, 0.2, 0.2], // Red
        }
    }

    /// Check if sickness is at a warning level.
    #[must_use]
    pub fn is_warning(&self) -> bool {
        !matches!(self.sickness_enum, SicknessLevel::None)
    }

    /// Check if sickness is at a danger level (Severe or Critical).
    #[must_use]
    pub fn is_danger(&self) -> bool {
        matches!(self.sickness_enum, SicknessLevel::Severe | SicknessLevel::Critical)
    }

    /// Get the icon name for the current sickness level.
    #[must_use]
    pub fn icon(&self) -> &'static str {
        match self.sickness_enum {
            SicknessLevel::None => "icon_sickness_none",
            SicknessLevel::Mild => "icon_sickness_mild",
            SicknessLevel::Moderate => "icon_sickness_moderate",
            SicknessLevel::Severe => "icon_sickness_severe",
            SicknessLevel::Critical => "icon_sickness_critical",
        }
    }

    /// Get the pulse rate for visual effects (Hz).
    #[must_use]
    pub fn pulse_rate(&self) -> f32 {
        match self.sickness_enum {
            SicknessLevel::None => 0.0,
            SicknessLevel::Mild => 0.3,
            SicknessLevel::Moderate => 0.5,
            SicknessLevel::Severe => 1.0,
            SicknessLevel::Critical => 2.0,
        }
    }

    /// Get the screen effect intensity (0.0 to 1.0).
    #[must_use]
    pub fn effect_intensity(&self) -> f32 {
        match self.sickness_enum {
            SicknessLevel::None => 0.0,
            SicknessLevel::Mild => 0.1,
            SicknessLevel::Moderate => 0.3,
            SicknessLevel::Severe => 0.6,
            SicknessLevel::Critical => 1.0,
        }
    }
}

impl Default for SicknessGaugeDisplay {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sickness_level_from_value() {
        assert_eq!(SicknessLevel::from_value(0.0), SicknessLevel::None);
        assert_eq!(SicknessLevel::from_value(15.0), SicknessLevel::Mild);
        assert_eq!(SicknessLevel::from_value(40.0), SicknessLevel::Moderate);
        assert_eq!(SicknessLevel::from_value(60.0), SicknessLevel::Severe);
        assert_eq!(SicknessLevel::from_value(90.0), SicknessLevel::Critical);
    }

    #[test]
    fn test_sickness_gauge_new() {
        let gauge = SicknessGaugeDisplay::new();
        assert!((gauge.level() - 0.0).abs() < f32::EPSILON);
        assert_eq!(gauge.sickness_enum(), SicknessLevel::None);
    }

    #[test]
    fn test_sickness_gauge_update() {
        let mut gauge = SicknessGaugeDisplay::new();
        gauge.update(60.0);

        assert!((gauge.level() - 60.0).abs() < f32::EPSILON);
        assert_eq!(gauge.sickness_enum(), SicknessLevel::Severe);
    }

    #[test]
    fn test_sickness_gauge_clamped() {
        let mut gauge = SicknessGaugeDisplay::new();

        gauge.update(150.0);
        assert!((gauge.level() - 100.0).abs() < f32::EPSILON);

        gauge.update(-10.0);
        assert!((gauge.level() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_sickness_gauge_percentage() {
        let mut gauge = SicknessGaugeDisplay::new();
        gauge.update(50.0);
        assert!((gauge.percentage() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_sickness_gauge_warning_text() {
        let mut gauge = SicknessGaugeDisplay::new();

        assert!(gauge.warning_text().is_none());

        gauge.update(15.0);
        assert!(gauge.warning_text().is_some());

        gauge.update(90.0);
        assert!(gauge.warning_text().unwrap().contains("CRITICAL"));
    }

    #[test]
    fn test_sickness_gauge_color() {
        let mut gauge = SicknessGaugeDisplay::new();

        // Healthy - green
        let color = gauge.color();
        assert!(color[1] > color[0]); // Green dominant

        // Critical - red
        gauge.update(90.0);
        let color = gauge.color();
        assert!(color[0] > color[1]); // Red dominant
    }

    #[test]
    fn test_sickness_gauge_warning() {
        let mut gauge = SicknessGaugeDisplay::new();

        assert!(!gauge.is_warning());

        gauge.update(10.0);
        assert!(gauge.is_warning());
    }

    #[test]
    fn test_sickness_gauge_danger() {
        let mut gauge = SicknessGaugeDisplay::new();

        gauge.update(40.0); // Moderate
        assert!(!gauge.is_danger());

        gauge.update(60.0); // Severe
        assert!(gauge.is_danger());
    }

    #[test]
    fn test_sickness_gauge_icon() {
        let mut gauge = SicknessGaugeDisplay::new();

        assert!(gauge.icon().contains("none"));

        gauge.update(90.0);
        assert!(gauge.icon().contains("critical"));
    }

    #[test]
    fn test_sickness_gauge_pulse_rate() {
        let mut gauge = SicknessGaugeDisplay::new();

        assert!((gauge.pulse_rate() - 0.0).abs() < f32::EPSILON);

        gauge.update(90.0);
        assert!(gauge.pulse_rate() > 1.0);
    }

    #[test]
    fn test_sickness_gauge_effect_intensity() {
        let mut gauge = SicknessGaugeDisplay::new();

        assert!((gauge.effect_intensity() - 0.0).abs() < f32::EPSILON);

        gauge.update(90.0);
        assert!((gauge.effect_intensity() - 1.0).abs() < f32::EPSILON);
    }
}
