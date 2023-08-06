use pettree::Tree;
use slotmap::DefaultKey;

#[path = "../fixtures/mod.rs"]
mod fixtures;

#[test]
fn test_insert_with_random_parent_key() {
    let trees = fixtures::all();

    for mut tree in trees {
        let key = DefaultKey::default();
        assert!(tree.insert(key, 0).is_none());
        assert!(!tree.contains(key));
    }
}

#[test]
fn test_insert_many_children() {
    let trees = fixtures::all_non_empty();

    fn get_right_most_key(tree: &Tree<DefaultKey, usize>) -> DefaultKey {
        let mut key = tree.root_key().unwrap();

        loop {
            let selected_child_key = tree.get(key).unwrap().child_keys.iter().next();
            match selected_child_key {
                Some(&selected_child_key) => key = selected_child_key,
                None => break key,
            }
        }
    }

    const NUMBER_OF_CHILDREN_TO_INSERT: usize = 100;

    for mut tree in trees {
        let key = get_right_most_key(&tree);

        assert!(tree.contains(key));
        assert_eq!(tree.get(key).unwrap().child_keys.len(), 0);

        (0..NUMBER_OF_CHILDREN_TO_INSERT).for_each(|value| {
            tree.insert(key, value).unwrap();
        });

        let child_keys = tree.get(key).unwrap().child_keys;
        assert_eq!(child_keys.len(), NUMBER_OF_CHILDREN_TO_INSERT);
        child_keys.iter().for_each(|&child_key| {
            let child_node = tree.get(child_key).unwrap();

            assert!(child_node.child_keys.is_empty());
            assert_eq!(child_node.parent_key.unwrap(), key);
        });
    }
}
