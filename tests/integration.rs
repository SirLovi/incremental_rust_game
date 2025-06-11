use incremental_rust_game::{farm_loss_event, res, BuildingType, GameState};
use rand::{rngs::StdRng, SeedableRng};
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn ten_minutes_growth() {
    let mut g = GameState::new();
    g.event_chance = 0.0;
    g.resources = res(20.0, 20.0, 0.0, 0.0, 0.0);
    assert!(g.build("farm".into()));
    let start = g.resources.food;
    g.tick(0.0); // initialize timestamp
    g.tick(600.0);
    assert_eq!(g.resources.food, start + 600.0);
}

#[wasm_bindgen_test]
fn event_triggers() {
    let mut g = GameState::new();
    g.resources = res(0.0, 0.0, 0.0, 0.0, 0.0);
    for _ in 0..10 {
        g.buildings.increment(BuildingType::Farm);
    }
    let mut rng = StdRng::seed_from_u64(0);
    let msg = farm_loss_event(&mut g.buildings, &mut rng, 1.0).unwrap();
    assert!(msg.contains("storm"));
    assert_eq!(g.buildings.level(BuildingType::Farm), 9);
}

#[wasm_bindgen_test]
fn save_load_integrity() {
    let mut g = GameState::new();
    g.event_chance = 0.0;
    g.resources.gold = 42.0;
    let data = g.save_string();
    let loaded = GameState::load_string(&data).unwrap();
    assert!((loaded.resources.gold - 42.0).abs() < 1e-6);
}

#[wasm_bindgen_test]
fn prestige_resets() {
    let mut g = GameState::new();
    g.resources.gold = 1_000_000.0;
    g.buildings.increment(BuildingType::Farm);
    g.prestige();
    assert!(g.prestige.points > 0);
    assert_eq!(g.buildings.level(BuildingType::Farm), 0);
}
