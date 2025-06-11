use crate::buildings::BuildingType;
use crate::buildings::Buildings;
use crate::research::{Research, Tech};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Tracks unlocked achievements
#[derive(Default, Serialize, Deserialize)]
pub struct Achievements {
    unlocked: HashSet<String>,
}

impl Achievements {
    /// Check for new achievements based on game state. Returns messages for
    /// any newly unlocked achievements.
    pub fn check(&mut self, buildings: &Buildings, research: &Research) -> Vec<String> {
        let mut msgs = Vec::new();
        if buildings.level(BuildingType::Farm) >= 1
            && self.unlocked.insert("First Farm".to_string())
        {
            msgs.push("Achievement unlocked: First Farm".to_string());
        }
        if research.is_unlocked(Tech::Mining)
            && self
                .unlocked
                .insert("Discovered Mining".to_string())
        {
            msgs.push("Achievement unlocked: Discovered Mining".to_string());
        }
        msgs
    }

    /// List achievements as strings
    pub fn list(&self) -> Vec<String> {
        let mut v: Vec<_> = self.unlocked.iter().cloned().collect();
        v.sort();
        v
    }
}
