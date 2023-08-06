#[path = "../fixtures/mod.rs"]
mod fixtures;

use slotmap::DefaultKey;

#[test]
fn test_remove_from_empty() {
    let mut tree = fixtures::empty_tree();

    let key = DefaultKey::default();

    assert!(!tree.remove(key, None));
    assert!(tree.is_empty());
}

#[test]
fn test_remove_root() {
    let trees = vec![
        fixtures::single_root_tree(),
        fixtures::depth_2_tree(),
        fixtures::linear_depth_4_tree(),
    ];

    for mut tree in trees {
        assert!(!tree.is_empty());

        let root_key = tree.root_key().unwrap();
        assert!(tree.remove(root_key, None));

        assert!(tree.is_empty());
    }
}
