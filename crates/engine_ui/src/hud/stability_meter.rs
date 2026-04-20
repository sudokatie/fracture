//! Stability meter HUD element.
//!
//! Displays the player's stability energy level.

/// Display state for the stability meter.
#[derive(Clone, Debug)]
pub struct StabilityMeterDisplay {
    /// Current energy level.
    energy: f32,
    /// Maximum energy capacity.
    max: f32,
}

impl StabilityMeterDisplay {
    /// Create a new stability meter display.
    #[must_use]
    pub fn new(max: f32) -> Self {
        Self {
            energy: max,
            max: max.max(1.0),
        }
    }

    /// Update the display with new energy values.
    pub fn update(&mut self, energy: f32, max: f32) {
        self.energy = energy.max(0.0);
        self.max = max.max(1.0);
    }

    /// Update only the current energy.
    pub fn set_energy(&mut self, energy: f32) {
        self.energy = energy.max(0.0);
    }

    /// Get the energy percentage (0.0 to 1.0).
    #[must_use]
    pub fn percentage(&self) -> f32 {
        (self.energy / self.max).clamp(0.0, 1.0)
    }

    /// Check if energy is at critical level (below 20%).
    #[must_use]
    pub fn is_critical(&self) -> bool {
        self.percentage() < 0.2
    }

    /// Check if energy is low (below 40%).
    #[must_use]
    pub fn is_low(&self) -> bool {
        self.percentage() < 0.4
    }

    /// Get the current energy value.
    #[must_use]
    pub fn current_energy(&self) -> f32 {
        self.energy
    }

    /// Get the maximum energy.
    #[must_use]
    pub fn max_energy(&self) -> f32 {
        self.max
    }

    /// Get the color for the meter based on level.
    ///
    /// Returns RGB color values (0.0 to 1.0).
    #[must_use]
    pub fn color(&self) -> [f32; 3] {
        let pct = self.percentage();
        if pct < 0.2 {
            [1.0, 0.2, 0.2] // Red - critical
        } else if pct < 0.4 {
            [1.0, 0.6, 0.2] // Orange - low
        } else if pct < 0.7 {
            [1.0, 1.0, 0.3] // Yellow - moderate
        } else {
            [0.3, 0.9, 0.5] // Green - good
        }
    }

    /// Get the formatted display string.
    #[must_use]
    pub fn display_text(&self) -> String {
        format!("{:.0}/{:.0}", self.energy, self.max)
    }
}

impl Default for StabilityMeterDisplay {
    fn default() -> Self {
        Self::new(100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stability_meter_new() {
        let meter = StabilityMeterDisplay::new(100.0);
        assert!((meter.current_energy() - 100.0).abs() < f32::EPSILON);
        assert!((meter.max_energy() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_stability_meter_percentage() {
        let mut meter = StabilityMeterDisplay::new(100.0);
        assert!((meter.percentage() - 1.0).abs() < f32::EPSILON);

        meter.set_energy(50.0);
        assert!((meter.percentage() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_stability_meter_update() {
        let mut meter = StabilityMeterDisplay::new(100.0);
        meter.update(75.0, 150.0);

        assert!((meter.current_energy() - 75.0).abs() < f32::EPSILON);
        assert!((meter.max_energy() - 150.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_stability_meter_critical() {
        let mut meter = StabilityMeterDisplay::new(100.0);
        assert!(!meter.is_critical());

        meter.set_energy(15.0);
        assert!(meter.is_critical());
    }

    #[test]
    fn test_stability_meter_low() {
        let mut meter = StabilityMeterDisplay::new(100.0);
        assert!(!meter.is_low());

        meter.set_energy(35.0);
        assert!(meter.is_low());
    }

    #[test]
    fn test_stability_meter_color() {
        let mut meter = StabilityMeterDisplay::new(100.0);

        // Full - green
        let color = meter.color();
        assert!(color[1] > color[0]); // Green dominant

        // Critical - red
        meter.set_energy(10.0);
        let color = meter.color();
        assert!(color[0] > color[1]); // Red dominant
    }

    #[test]
    fn test_stability_meter_display_text() {
        let meter = StabilityMeterDisplay::new(100.0);
        assert_eq!(meter.display_text(), "100/100");
    }
}
