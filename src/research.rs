use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Technologies that can be researched
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Tech {
    /// Unlocks mines
    Mining,
    /// Unlocks bakeries
    Baking,
}

/// Player research tree
#[derive(Default, Serialize, Deserialize)]
pub struct Research {
    unlocked: HashSet<Tech>,
}

impl Research {
    /// Check if technology is unlocked
    pub fn is_unlocked(&self, tech: Tech) -> bool {
        self.unlocked.contains(&tech)
    }

    /// Unlock a technology
    pub fn unlock(&mut self, tech: Tech) {
        self.unlocked.insert(tech);
    }
}
