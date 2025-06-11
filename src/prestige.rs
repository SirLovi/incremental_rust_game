use serde::{Deserialize, Serialize};

/// Persistent prestige data
#[derive(Default, Serialize, Deserialize)]
pub struct Prestige {
    /// Points earned from resets
    pub points: u32,
}

impl Prestige {
    /// Calculate bonus multiplier from prestige points
    pub fn bonus_multiplier(&self) -> f64 {
        1.0 + self.points as f64 * 0.05
    }
}
