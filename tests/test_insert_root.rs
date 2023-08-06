#[path = "../fixtures/mod.rs"]
mod fixtures;

#[test]
fn test_insert_root() {
    let trees = vec![
        fixtures::empty_tree(),
        fixtures::single_root_tree(),
        fixtures::depth_2_tree(),
        fixtures::linear_depth_4_tree(),
    ];

    for mut tree in trees {
        let root_key = tree.insert_root(0);

        assert_eq!(tree.len(), 1);
        assert!(tree.contains(root_key));
        assert_eq!(*tree.get(root_key).unwrap().value, 0);
        assert_eq!(tree.root_key().unwrap(), root_key);
    }
}
