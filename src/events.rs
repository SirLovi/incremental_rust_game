use crate::buildings::{BuildingType, Buildings};
use crate::resources::Resources;
use rand::Rng;

/// Chance of farm loss event each tick
pub const FARM_LOSS_CHANCE: f64 = 0.05;

/// Checks for random events and applies effects. Returns a message when an
/// event occurs.
pub fn check_random_events<R: Rng>(
    buildings: &mut Buildings,
    resources: &mut Resources,
    rng: &mut R,
    chance: f64,
) -> Option<String> {
    if rng.gen_bool(0.5) {
        farm_loss_event(buildings, rng, chance)
    } else {
        treasure_event(resources, rng, chance)
    }
}

/// 10% of farms are destroyed when triggered.
pub fn farm_loss_event<R: Rng>(
    buildings: &mut Buildings,
    rng: &mut R,
    chance: f64,
) -> Option<String> {
    if rng.gen_bool(chance) {
        let farms = buildings.level(BuildingType::Farm);
        if farms > 0 {
            let mut loss = ((farms as f64) * 0.1).ceil() as u32;
            if loss == 0 {
                loss = 1;
            }
            buildings.decrement(BuildingType::Farm, loss);
            return Some(format!("A storm destroyed {loss} farms!"));
        }
    }
    None
}

/// Random treasure awarding gold
pub fn treasure_event<R: Rng>(res: &mut Resources, rng: &mut R, chance: f64) -> Option<String> {
    if rng.gen_bool(chance * 0.5) {
        let gold = rng.gen_range(5..20) as f64;
        res.gold += gold;
        return Some(format!("Found a hidden treasure worth {gold} gold!"));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn farm_loss_triggers() {
        let mut b = Buildings::default();
        for _ in 0..10 {
            b.increment(BuildingType::Farm);
        }
        let mut rng = StdRng::seed_from_u64(1);
        let msg = farm_loss_event(&mut b, &mut rng, 1.0).expect("should trigger");
        assert!(msg.contains("storm"));
        assert_eq!(b.level(BuildingType::Farm), 9); // ceil(10% of 10) = 1
    }
}
