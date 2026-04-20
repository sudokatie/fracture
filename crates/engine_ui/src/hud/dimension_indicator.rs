//! Dimension indicator HUD element.
//!
//! Displays the current dimension the player is in.

use engine_physics::dimension::Dimension;

/// Display state for the dimension indicator.
#[derive(Clone, Debug)]
pub struct DimensionIndicatorDisplay {
    /// Current dimension.
    current_dim: Dimension,
}

impl DimensionIndicatorDisplay {
    /// Create a new dimension indicator display.
    #[must_use]
    pub fn new() -> Self {
        Self {
            current_dim: Dimension::Prime,
        }
    }

    /// Update the display with a new dimension.
    pub fn update(&mut self, dim: Dimension) {
        self.current_dim = dim;
    }

    /// Get the current dimension.
    #[must_use]
    pub fn current_dimension(&self) -> Dimension {
        self.current_dim
    }

    /// Get the display color for the current dimension.
    ///
    /// Returns RGB color values (0.0 to 1.0).
    #[must_use]
    pub fn color(&self) -> [f32; 3] {
        match self.current_dim {
            Dimension::Prime => [0.3, 0.7, 1.0],     // Light blue
            Dimension::Inverted => [1.0, 0.5, 0.2],  // Orange
            Dimension::Void => [0.4, 0.4, 0.5],      // Gray
            Dimension::Nexus => [0.7, 0.3, 0.9],     // Purple
        }
    }

    /// Get the display label for the current dimension.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self.current_dim {
            Dimension::Prime => "PRIME",
            Dimension::Inverted => "INVERTED",
            Dimension::Void => "VOID",
            Dimension::Nexus => "NEXUS",
        }
    }

    /// Get the icon name for the current dimension.
    #[must_use]
    pub fn icon(&self) -> &'static str {
        match self.current_dim {
            Dimension::Prime => "icon_dimension_prime",
            Dimension::Inverted => "icon_dimension_inverted",
            Dimension::Void => "icon_dimension_void",
            Dimension::Nexus => "icon_dimension_nexus",
        }
    }

    /// Check if an alert should be shown (non-Prime dimension).
    #[must_use]
    pub fn should_show_alert(&self) -> bool {
        !matches!(self.current_dim, Dimension::Prime)
    }
}

impl Default for DimensionIndicatorDisplay {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimension_indicator_new() {
        let indicator = DimensionIndicatorDisplay::new();
        assert_eq!(indicator.current_dimension(), Dimension::Prime);
    }

    #[test]
    fn test_dimension_indicator_update() {
        let mut indicator = DimensionIndicatorDisplay::new();
        indicator.update(Dimension::Void);
        assert_eq!(indicator.current_dimension(), Dimension::Void);
    }

    #[test]
    fn test_dimension_indicator_color() {
        let mut indicator = DimensionIndicatorDisplay::new();

        let prime_color = indicator.color();
        assert_eq!(prime_color.len(), 3);

        indicator.update(Dimension::Inverted);
        let inverted_color = indicator.color();
        assert!(inverted_color[0] > inverted_color[2]); // More red than blue
    }

    #[test]
    fn test_dimension_indicator_label() {
        let mut indicator = DimensionIndicatorDisplay::new();

        assert_eq!(indicator.label(), "PRIME");

        indicator.update(Dimension::Nexus);
        assert_eq!(indicator.label(), "NEXUS");
    }

    #[test]
    fn test_dimension_indicator_icon() {
        let indicator = DimensionIndicatorDisplay::new();
        assert!(indicator.icon().contains("prime"));
    }

    #[test]
    fn test_dimension_indicator_alert() {
        let mut indicator = DimensionIndicatorDisplay::new();

        assert!(!indicator.should_show_alert());

        indicator.update(Dimension::Void);
        assert!(indicator.should_show_alert());
    }
}
