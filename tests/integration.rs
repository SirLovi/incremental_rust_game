use incremental_rust_game::{GameState, res, farm_loss_event, BuildingType};
use rand::{rngs::StdRng, SeedableRng};

#[test]
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

#[test]
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
