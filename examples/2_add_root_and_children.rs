//! This example showcases how to create a default, empty [`Tree`] instance and
//! insert a root value and 3 children values into it. In this example, the
//! children values will be *direct* children of the root value.

use tinytree::Tree;
use indexmap::IndexSet;
use slotmap::DefaultKey;

fn main() {
    let mut tree = Tree::<DefaultKey, usize>::default();

    let root_key = tree.insert_root(0);

    let child_key_1 = tree.insert(1, root_key).unwrap();
    let child_key_2 = tree.insert(2, root_key).unwrap();
    let child_key_3 = tree.insert(3, root_key).unwrap();

    assert!(!tree.is_empty());

    let root_node = tree.get(root_key).unwrap();
    assert_eq!(*root_node.value, 0);
    assert_eq!(root_node.parent_key, None);
    assert_eq!(
        root_node.child_keys.clone(),
        vec![child_key_1, child_key_2, child_key_3]
            .into_iter()
            .collect::<IndexSet<_>>(),
    );

    let child_node_1 = tree.get(child_key_1).unwrap();
    assert_eq!(*child_node_1.value, 1);
    assert_eq!(child_node_1.parent_key, Some(root_key));
    assert_eq!(child_node_1.child_keys.len(), 0,);

    let child_node_2 = tree.get(child_key_2).unwrap();
    assert_eq!(*child_node_2.value, 2);
    assert_eq!(child_node_2.parent_key, Some(root_key));
    assert_eq!(child_node_2.child_keys.len(), 0,);

    let child_node_3 = tree.get(child_key_3).unwrap();
    assert_eq!(*child_node_3.value, 3);
    assert_eq!(child_node_3.parent_key, Some(root_key));
    assert_eq!(child_node_3.child_keys.len(), 0,);
}
