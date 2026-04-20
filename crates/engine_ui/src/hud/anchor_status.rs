//! Anchor status HUD element.
//!
//! Displays the status of active dimensional anchors.

/// Display state for anchor status.
#[derive(Clone, Debug)]
pub struct AnchorStatusDisplay {
    /// Number of active anchors.
    active_anchors: u32,
    /// Total fuel remaining across all anchors.
    total_fuel: f32,
    /// Whether any anchor is low on fuel.
    any_low_fuel: bool,
}

impl AnchorStatusDisplay {
    /// Create a new anchor status display.
    #[must_use]
    pub fn new() -> Self {
        Self {
            active_anchors: 0,
            total_fuel: 0.0,
            any_low_fuel: false,
        }
    }

    /// Update the display with new anchor data.
    pub fn update(&mut self, active_anchors: u32, total_fuel: f32, any_low_fuel: bool) {
        self.active_anchors = active_anchors;
        self.total_fuel = total_fuel.max(0.0);
        self.any_low_fuel = any_low_fuel;
    }

    /// Get the number of active anchors.
    #[must_use]
    pub fn active_anchors(&self) -> u32 {
        self.active_anchors
    }

    /// Get the total fuel remaining.
    #[must_use]
    pub fn total_fuel(&self) -> f32 {
        self.total_fuel
    }

    /// Check if any anchor is low on fuel.
    #[must_use]
    pub fn has_low_fuel(&self) -> bool {
        self.any_low_fuel
    }

    /// Get the status text.
    #[must_use]
    pub fn status_text(&self) -> String {
        if self.active_anchors == 0 {
            "No active anchors".to_string()
        } else if self.active_anchors == 1 {
            format!("1 anchor ({:.0}s fuel)", self.total_fuel)
        } else {
            format!("{} anchors ({:.0}s fuel)", self.active_anchors, self.total_fuel)
        }
    }

    /// Get the display color based on status.
    ///
    /// Returns RGB color values (0.0 to 1.0).
    #[must_use]
    pub fn color(&self) -> [f32; 3] {
        if self.active_anchors == 0 {
            [0.5, 0.5, 0.5] // Gray - none active
        } else if self.any_low_fuel {
            [1.0, 0.6, 0.2] // Orange - warning
        } else {
            [0.3, 0.8, 1.0] // Cyan - normal
        }
    }

    /// Check if status should show a warning.
    #[must_use]
    pub fn is_warning(&self) -> bool {
        self.any_low_fuel
    }

    /// Check if anchors are providing protection.
    #[must_use]
    pub fn is_protected(&self) -> bool {
        self.active_anchors > 0 && !self.any_low_fuel
    }

    /// Get the icon name for the current status.
    #[must_use]
    pub fn icon(&self) -> &'static str {
        if self.active_anchors == 0 {
            "icon_anchor_inactive"
        } else if self.any_low_fuel {
            "icon_anchor_warning"
        } else {
            "icon_anchor_active"
        }
    }

    /// Get formatted fuel time string.
    #[must_use]
    pub fn fuel_time_text(&self) -> String {
        if self.total_fuel <= 0.0 {
            "Empty".to_string()
        } else if self.total_fuel < 60.0 {
            format!("{:.0}s", self.total_fuel)
        } else if self.total_fuel < 3600.0 {
            format!("{:.0}m", self.total_fuel / 60.0)
        } else {
            format!("{:.1}h", self.total_fuel / 3600.0)
        }
    }
}

impl Default for AnchorStatusDisplay {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anchor_status_new() {
        let status = AnchorStatusDisplay::new();
        assert_eq!(status.active_anchors(), 0);
        assert!((status.total_fuel() - 0.0).abs() < f32::EPSILON);
        assert!(!status.has_low_fuel());
    }

    #[test]
    fn test_anchor_status_update() {
        let mut status = AnchorStatusDisplay::new();
        status.update(3, 1500.0, false);

        assert_eq!(status.active_anchors(), 3);
        assert!((status.total_fuel() - 1500.0).abs() < f32::EPSILON);
        assert!(!status.has_low_fuel());
    }

    #[test]
    fn test_anchor_status_text_none() {
        let status = AnchorStatusDisplay::new();
        assert!(status.status_text().contains("No active"));
    }

    #[test]
    fn test_anchor_status_text_single() {
        let mut status = AnchorStatusDisplay::new();
        status.update(1, 500.0, false);
        assert!(status.status_text().contains("1 anchor"));
    }

    #[test]
    fn test_anchor_status_text_multiple() {
        let mut status = AnchorStatusDisplay::new();
        status.update(3, 1500.0, false);
        assert!(status.status_text().contains("3 anchors"));
    }

    #[test]
    fn test_anchor_status_color() {
        let mut status = AnchorStatusDisplay::new();

        // No anchors - gray
        let color = status.color();
        assert!((color[0] - color[1]).abs() < f32::EPSILON); // Gray

        // Active - cyan
        status.update(1, 500.0, false);
        let color = status.color();
        assert!(color[2] > color[0]); // Blue dominant

        // Low fuel - orange
        status.update(1, 50.0, true);
        let color = status.color();
        assert!(color[0] > color[2]); // Orange/red dominant
    }

    #[test]
    fn test_anchor_status_warning() {
        let mut status = AnchorStatusDisplay::new();

        assert!(!status.is_warning());

        status.update(1, 50.0, true);
        assert!(status.is_warning());
    }

    #[test]
    fn test_anchor_status_protected() {
        let mut status = AnchorStatusDisplay::new();

        assert!(!status.is_protected());

        status.update(1, 500.0, false);
        assert!(status.is_protected());

        status.update(1, 50.0, true);
        assert!(!status.is_protected()); // Low fuel means not protected
    }

    #[test]
    fn test_anchor_status_icon() {
        let mut status = AnchorStatusDisplay::new();

        assert!(status.icon().contains("inactive"));

        status.update(1, 500.0, false);
        assert!(status.icon().contains("active"));

        status.update(1, 50.0, true);
        assert!(status.icon().contains("warning"));
    }

    #[test]
    fn test_anchor_status_fuel_time_text() {
        let mut status = AnchorStatusDisplay::new();

        status.update(1, 0.0, false);
        assert_eq!(status.fuel_time_text(), "Empty");

        status.update(1, 45.0, false);
        assert!(status.fuel_time_text().contains("s"));

        status.update(1, 300.0, false);
        assert!(status.fuel_time_text().contains("m"));

        status.update(1, 7200.0, false);
        assert!(status.fuel_time_text().contains("h"));
    }
}
