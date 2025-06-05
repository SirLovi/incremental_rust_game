use incremental_rust_game::Game;

#[test]
fn craft_axe_only_once_integration() {
    let mut game = Game::new();
    for _ in 0..10 { game.collect_wood(); }
    for _ in 0..5 { game.collect_stone(); }

    // First crafting should succeed
    assert!(game.craft_axe());
    assert_eq!(game.get_wood(), 0);
    assert_eq!(game.get_stone(), 0);

    // Second crafting should fail and not change resources
    assert!(!game.craft_axe());
    assert_eq!(game.get_wood(), 0);
    assert_eq!(game.get_stone(), 0);
}

#[test]
fn craft_pickaxe_only_once_integration() {
    let mut game = Game::new();
    for _ in 0..5 { game.collect_wood(); }
    for _ in 0..10 { game.collect_stone(); }

    // First crafting should succeed
    assert!(game.craft_pickaxe());
    assert_eq!(game.get_wood(), 0);
    assert_eq!(game.get_stone(), 0);

    // Second crafting should fail and not change resources
    assert!(!game.craft_pickaxe());
    assert_eq!(game.get_wood(), 0);
    assert_eq!(game.get_stone(), 0);
}
