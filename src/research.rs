use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Technologies that can be researched
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Tech {
    /// Unlocks mines
    Mining,
    /// Unlocks bakeries
    Baking,
    /// Allows construction of generators
    Electricity,
    /// Allows laboratories for science
    Education,
    /// Enables shrines and mana
    Alchemy,
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

    /// Attempt to unlock a technology consuming science
    pub fn try_unlock(&mut self, tech: Tech, res: &mut crate::resources::Resources) -> bool {
        if self.is_unlocked(tech) {
            return true;
        }
        let cost = 100.0;
        if res.science >= cost {
            res.science -= cost;
            self.unlocked.insert(tech);
            true
        } else {
            false
        }
    }
}
