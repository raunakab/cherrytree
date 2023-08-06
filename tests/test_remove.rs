#[path = "../fixtures/mod.rs"]
mod fixtures;

use slotmap::DefaultKey;

#[test]
fn test_remove_random_key() {
    let trees = vec![
        fixtures::empty_tree(),
        fixtures::single_root_tree(),
        fixtures::depth_2_tree(),
        fixtures::linear_depth_4_tree(),
    ];

    let key = DefaultKey::default();

    for mut tree in trees {
        assert!(!tree.contains(key));
        assert!(tree.remove(key, None).is_none());
    }
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
        assert!(tree.remove(root_key, None).is_some());

        assert!(tree.is_empty());
    }
}

#[test]
fn test_remove_child() {
    todo!()
}
