//! Dimension state tracking for chunks in the world.
//!
//! Tracks which dimension each chunk belongs to and provides
//! utilities for querying dimension boundaries and properties.

use std::collections::HashMap;
use std::fmt;

use glam::IVec3;
use serde::{Deserialize, Serialize};

/// The four dimensions of reality in the game world.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Dimension {
    /// The normal, stable dimension where players start.
    #[default]
    Prime,
    /// A mirror dimension with altered physics and temperature.
    Inverted,
    /// A cold, dark, low-gravity dimension.
    Void,
    /// A nexus dimension connecting all others with high gravity.
    Nexus,
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Dimension::Prime => write!(f, "Prime"),
            Dimension::Inverted => write!(f, "Inverted"),
            Dimension::Void => write!(f, "Void"),
            Dimension::Nexus => write!(f, "Nexus"),
        }
    }
}

impl Dimension {
    /// Get all dimension variants.
    #[must_use]
    pub fn all() -> &'static [Dimension] {
        &[
            Dimension::Prime,
            Dimension::Inverted,
            Dimension::Void,
            Dimension::Nexus,
        ]
    }
}

/// Physical properties of a dimension.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DimensionProperties {
    /// The dimension these properties belong to.
    pub dimension: Dimension,
    /// Base ambient temperature in degrees Celsius.
    pub base_temperature: f32,
    /// Light level multiplier (0.0 to 1.0).
    pub light_level: f32,
    /// Fog density (0.0 = clear, 1.0 = opaque).
    pub fog_density: f32,
    /// Gravity modifier (1.0 = normal).
    pub gravity_modifier: f32,
}

impl DimensionProperties {
    /// Get properties for a specific dimension.
    #[must_use]
    pub fn for_dimension(dimension: Dimension) -> Self {
        match dimension {
            Dimension::Prime => Self {
                dimension,
                base_temperature: 20.0,
                light_level: 1.0,
                fog_density: 0.0,
                gravity_modifier: 1.0,
            },
            Dimension::Inverted => Self {
                dimension,
                base_temperature: 60.0,
                light_level: 0.3,
                fog_density: 0.3,
                gravity_modifier: 0.8,
            },
            Dimension::Void => Self {
                dimension,
                base_temperature: -40.0,
                light_level: 0.05,
                fog_density: 0.9,
                gravity_modifier: 0.3,
            },
            Dimension::Nexus => Self {
                dimension,
                base_temperature: 30.0,
                light_level: 0.6,
                fog_density: 0.5,
                gravity_modifier: 1.2,
            },
        }
    }
}

/// Get dimension properties for a given dimension.
#[must_use]
pub fn get_dimension_properties(dim: Dimension) -> DimensionProperties {
    DimensionProperties::for_dimension(dim)
}

/// The six cardinal directions for neighbor lookups.
const NEIGHBOR_OFFSETS: [IVec3; 6] = [
    IVec3::new(1, 0, 0),
    IVec3::new(-1, 0, 0),
    IVec3::new(0, 1, 0),
    IVec3::new(0, -1, 0),
    IVec3::new(0, 0, 1),
    IVec3::new(0, 0, -1),
];

/// Tracks which dimension each chunk in the world belongs to.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DimensionState {
    /// Map from chunk position to its current dimension.
    chunk_dimensions: HashMap<IVec3, Dimension>,
}

impl DimensionState {
    /// Create a new dimension state with no tracked chunks.
    ///
    /// Untracked chunks default to Prime dimension.
    #[must_use]
    pub fn new() -> Self {
        Self {
            chunk_dimensions: HashMap::new(),
        }
    }

    /// Get the dimension of a chunk.
    ///
    /// Returns Prime if the chunk is not explicitly tracked.
    #[must_use]
    pub fn get_dimension(&self, pos: IVec3) -> Dimension {
        self.chunk_dimensions
            .get(&pos)
            .copied()
            .unwrap_or(Dimension::Prime)
    }

    /// Set the dimension of a chunk.
    ///
    /// Returns the previous dimension (Prime if untracked).
    pub fn set_dimension(&mut self, pos: IVec3, dim: Dimension) -> Dimension {
        let previous = self.get_dimension(pos);
        if dim == Dimension::Prime {
            // Remove Prime entries to save memory (it's the default)
            self.chunk_dimensions.remove(&pos);
        } else {
            self.chunk_dimensions.insert(pos, dim);
        }
        previous
    }

    /// Get unique dimensions of the 6 cardinal neighbors.
    #[must_use]
    pub fn get_adjacent_dimensions(&self, pos: IVec3) -> Vec<Dimension> {
        let mut dimensions = Vec::with_capacity(6);
        for offset in &NEIGHBOR_OFFSETS {
            let neighbor_pos = pos + *offset;
            let dim = self.get_dimension(neighbor_pos);
            if !dimensions.contains(&dim) {
                dimensions.push(dim);
            }
        }
        dimensions
    }

    /// Get all chunk positions in a specific dimension.
    #[must_use]
    pub fn chunks_in_dimension(&self, dim: Dimension) -> Vec<IVec3> {
        if dim == Dimension::Prime {
            // Prime chunks are not stored, so we can't enumerate them
            // This would require knowing all chunks in the world
            // For now, return empty vec for Prime
            Vec::new()
        } else {
            self.chunk_dimensions
                .iter()
                .filter_map(|(&pos, &d)| if d == dim { Some(pos) } else { None })
                .collect()
        }
    }

    /// Count chunks in a specific dimension.
    #[must_use]
    pub fn dimension_count(&self, dim: Dimension) -> usize {
        if dim == Dimension::Prime {
            // Can't count Prime chunks without knowing total world chunks
            0
        } else {
            self.chunk_dimensions.values().filter(|&&d| d == dim).count()
        }
    }

    /// Check if a chunk is on a fracture border.
    ///
    /// Returns true if any of the 6 cardinal neighbors has a different dimension.
    #[must_use]
    pub fn is_fracture_border(&self, pos: IVec3) -> bool {
        let my_dim = self.get_dimension(pos);
        for offset in &NEIGHBOR_OFFSETS {
            let neighbor_pos = pos + *offset;
            if self.get_dimension(neighbor_pos) != my_dim {
                return true;
            }
        }
        false
    }

    /// Get total number of explicitly tracked chunks (non-Prime).
    #[must_use]
    pub fn total_tracked(&self) -> usize {
        self.chunk_dimensions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimension_display() {
        assert_eq!(format!("{}", Dimension::Prime), "Prime");
        assert_eq!(format!("{}", Dimension::Inverted), "Inverted");
        assert_eq!(format!("{}", Dimension::Void), "Void");
        assert_eq!(format!("{}", Dimension::Nexus), "Nexus");
    }

    #[test]
    fn test_dimension_default() {
        assert_eq!(Dimension::default(), Dimension::Prime);
    }

    #[test]
    fn test_dimension_all() {
        let all = Dimension::all();
        assert_eq!(all.len(), 4);
        assert!(all.contains(&Dimension::Prime));
        assert!(all.contains(&Dimension::Inverted));
        assert!(all.contains(&Dimension::Void));
        assert!(all.contains(&Dimension::Nexus));
    }

    #[test]
    fn test_dimension_properties_prime() {
        let props = get_dimension_properties(Dimension::Prime);
        assert_eq!(props.dimension, Dimension::Prime);
        assert!((props.base_temperature - 20.0).abs() < f32::EPSILON);
        assert!((props.light_level - 1.0).abs() < f32::EPSILON);
        assert!((props.fog_density - 0.0).abs() < f32::EPSILON);
        assert!((props.gravity_modifier - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_dimension_properties_inverted() {
        let props = get_dimension_properties(Dimension::Inverted);
        assert!((props.base_temperature - 60.0).abs() < f32::EPSILON);
        assert!((props.light_level - 0.3).abs() < f32::EPSILON);
        assert!((props.fog_density - 0.3).abs() < f32::EPSILON);
        assert!((props.gravity_modifier - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn test_dimension_properties_void() {
        let props = get_dimension_properties(Dimension::Void);
        assert!((props.base_temperature - -40.0).abs() < f32::EPSILON);
        assert!((props.light_level - 0.05).abs() < f32::EPSILON);
        assert!((props.fog_density - 0.9).abs() < f32::EPSILON);
        assert!((props.gravity_modifier - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn test_dimension_properties_nexus() {
        let props = get_dimension_properties(Dimension::Nexus);
        assert!((props.base_temperature - 30.0).abs() < f32::EPSILON);
        assert!((props.light_level - 0.6).abs() < f32::EPSILON);
        assert!((props.fog_density - 0.5).abs() < f32::EPSILON);
        assert!((props.gravity_modifier - 1.2).abs() < f32::EPSILON);
    }

    #[test]
    fn test_dimension_state_new() {
        let state = DimensionState::new();
        assert_eq!(state.total_tracked(), 0);
        assert_eq!(state.get_dimension(IVec3::ZERO), Dimension::Prime);
    }

    #[test]
    fn test_dimension_state_set_get() {
        let mut state = DimensionState::new();
        let pos = IVec3::new(1, 2, 3);

        // Default is Prime
        assert_eq!(state.get_dimension(pos), Dimension::Prime);

        // Set to Void
        let prev = state.set_dimension(pos, Dimension::Void);
        assert_eq!(prev, Dimension::Prime);
        assert_eq!(state.get_dimension(pos), Dimension::Void);

        // Set back to Prime
        let prev = state.set_dimension(pos, Dimension::Prime);
        assert_eq!(prev, Dimension::Void);
        assert_eq!(state.get_dimension(pos), Dimension::Prime);
    }

    #[test]
    fn test_dimension_state_adjacent() {
        let mut state = DimensionState::new();
        let center = IVec3::ZERO;

        // All neighbors are Prime by default
        let adjacent = state.get_adjacent_dimensions(center);
        assert_eq!(adjacent.len(), 1);
        assert!(adjacent.contains(&Dimension::Prime));

        // Set one neighbor to Void
        state.set_dimension(IVec3::new(1, 0, 0), Dimension::Void);
        let adjacent = state.get_adjacent_dimensions(center);
        assert_eq!(adjacent.len(), 2);
        assert!(adjacent.contains(&Dimension::Prime));
        assert!(adjacent.contains(&Dimension::Void));
    }

    #[test]
    fn test_dimension_state_is_fracture_border() {
        let mut state = DimensionState::new();
        let center = IVec3::ZERO;

        // Not a border when all neighbors are same dimension
        assert!(!state.is_fracture_border(center));

        // Becomes border when neighbor is different
        state.set_dimension(IVec3::new(1, 0, 0), Dimension::Inverted);
        assert!(state.is_fracture_border(center));
    }

    #[test]
    fn test_dimension_state_chunks_in_dimension() {
        let mut state = DimensionState::new();

        state.set_dimension(IVec3::new(0, 0, 0), Dimension::Void);
        state.set_dimension(IVec3::new(1, 0, 0), Dimension::Void);
        state.set_dimension(IVec3::new(2, 0, 0), Dimension::Nexus);

        let void_chunks = state.chunks_in_dimension(Dimension::Void);
        assert_eq!(void_chunks.len(), 2);
        assert!(void_chunks.contains(&IVec3::new(0, 0, 0)));
        assert!(void_chunks.contains(&IVec3::new(1, 0, 0)));

        let nexus_chunks = state.chunks_in_dimension(Dimension::Nexus);
        assert_eq!(nexus_chunks.len(), 1);

        // Prime chunks can't be enumerated
        let prime_chunks = state.chunks_in_dimension(Dimension::Prime);
        assert!(prime_chunks.is_empty());
    }

    #[test]
    fn test_dimension_state_count() {
        let mut state = DimensionState::new();

        state.set_dimension(IVec3::new(0, 0, 0), Dimension::Void);
        state.set_dimension(IVec3::new(1, 0, 0), Dimension::Void);
        state.set_dimension(IVec3::new(2, 0, 0), Dimension::Nexus);

        assert_eq!(state.dimension_count(Dimension::Void), 2);
        assert_eq!(state.dimension_count(Dimension::Nexus), 1);
        assert_eq!(state.dimension_count(Dimension::Inverted), 0);
        assert_eq!(state.total_tracked(), 3);
    }
}
