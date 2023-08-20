// #[path = "../common/mod.rs"]
// mod common;
//
// use slotmap::DefaultKey;
//
// #[test]
// fn test_insert_with_random_parent_key() {
//     let trees = common::fixtures::all();
//
//     for mut tree in trees {
//         let key = DefaultKey::default();
//         assert!(tree.insert(key, 0).is_none());
//         assert!(!tree.contains(key));
//     }
// }
//
// #[test]
// fn test_insert_many_children() {
//     let trees = common::fixtures::all_non_empty();
//
//     const NUMBER_OF_CHILDREN_TO_INSERT: usize = 100;
//
//     for mut tree in trees {
//         let key = common::utils::get_some_leaf_key(&tree);
//
//         assert!(tree.contains(key));
//         assert_eq!(tree.get(key).unwrap().child_keys.len(), 0);
//
//         (0..NUMBER_OF_CHILDREN_TO_INSERT).for_each(|value| {
//             tree.insert(key, value).unwrap();
//         });
//
//         let child_keys = tree.get(key).unwrap().child_keys;
//         assert_eq!(child_keys.len(), NUMBER_OF_CHILDREN_TO_INSERT);
//         child_keys.iter().for_each(|&child_key| {
//             let child_node = tree.get(child_key).unwrap();
//
//             assert!(child_node.child_keys.is_empty());
//             assert_eq!(child_node.parent_key.unwrap(), key);
//         });
//     }
// }
