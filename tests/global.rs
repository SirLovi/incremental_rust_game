use incremental_rust_game::*;
use serial_test::serial;

#[test]
#[serial]
fn collect_wood_global_increments() {
    reset_game();
    let start = get_wood_global();
    assert_eq!(collect_wood_global(), start + 1);
    assert_eq!(get_wood_global(), start + 1);
}

#[test]
#[serial]
fn craft_axe_global_uses_resources() {
    reset_game();
    for _ in 0..10 { collect_wood_global(); }
    for _ in 0..5 { collect_stone_global(); }
    assert!(craft_axe_global());
    assert!(has_axe_global());
    assert_eq!(get_wood_global(), 0);
    assert_eq!(get_stone_global(), 0);
    assert!(!craft_axe_global());
}

#[test]
#[serial]
fn passive_collection_increases_resources() {
    reset_game();
    // Craft tools
    for _ in 0..10 { collect_wood_global(); }
    for _ in 0..5 { collect_stone_global(); }
    craft_axe_global();
    for _ in 0..5 { collect_wood_global(); }
    for _ in 0..10 { collect_stone_global(); }
    craft_pickaxe_global();

    let wood_before = get_wood_global();
    let stone_before = get_stone_global();
    passive_wood_collection();
    passive_stone_collection();
    assert_eq!(get_wood_global(), wood_before + 1);
    assert_eq!(get_stone_global(), stone_before + 1);
}

#[test]
#[serial]
fn farm_generates_food_passively() {
    reset_game();
    for _ in 0..10 { collect_wood_global(); }
    for _ in 0..10 { collect_stone_global(); }
    assert!(build_farm_global());
    let food_before = get_food_global();
    passive_food_generation();
    assert_eq!(get_food_global(), food_before + 1);
}
