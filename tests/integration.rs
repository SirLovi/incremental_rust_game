use incremental_rust_game::{GameState, res};

#[test]
fn ten_minutes_growth() {
    let mut g = GameState::new();
    g.resources = res(20.0, 20.0, 0.0, 0.0, 0.0);
    assert!(g.build("farm".into()));
    let start = g.resources.food;
    g.tick(0.0); // initialize timestamp
    g.tick(600.0);
    assert_eq!(g.resources.food, start + 600.0);
}
