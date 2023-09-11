#[macro_export]
macro_rules! node {
    ($value:expr$(,)?) => {{
        use crate::tests::utils::DeclTree;
        DeclTree { value: $value, child_keys: vec![] }
    }};

    (
        $value:expr,
        [$($child:expr),*$(,)?]$(,)?
    ) => {{
        use crate::tests::utils::DeclTree;
        DeclTree { value: $value, child_keys: vec![$($child),*] }
    }};
}

use std::collections::HashMap;

pub use node;
use slotmap::DefaultKey;

use crate::Tree;

#[derive(Debug, PartialEq, Eq)]
pub struct DeclTree {
    pub value: usize,
    pub child_keys: Vec<Self>,
}

pub fn make_tree_and_key_map(
    decl_tree: Option<&DeclTree>,
) -> (Tree<DefaultKey, usize>, HashMap<usize, DefaultKey>) {
    let mut tree = Tree::default();
    let mut key_map = HashMap::default();

    fn insert(
        tree: &mut Tree<DefaultKey, usize>,
        key_map: &mut HashMap<usize, DefaultKey>,
        decl_tree: &DeclTree,
        parent_key: Option<DefaultKey>,
    ) {
        let key = match parent_key {
            Some(parent_key) => tree.insert(decl_tree.value, parent_key).unwrap(),
            None => tree.insert_root(decl_tree.value),
        };

        let previous_key = key_map.insert(decl_tree.value, key);
        assert!(previous_key.is_none());

        decl_tree
            .child_keys
            .iter()
            .for_each(|child_decl_tree| insert(tree, key_map, child_decl_tree, Some(key)));
    }

    if let Some(decl_tree) = decl_tree {
        insert(&mut tree, &mut key_map, decl_tree, None);
    };

    (tree, key_map)
}

pub fn make_decl_tree(tree: &Tree<DefaultKey, usize>) -> Option<DeclTree> {
    fn make_decl_tree(
        tree: &Tree<DefaultKey, usize>,
        key: DefaultKey,
        parent_key: Option<DefaultKey>,
    ) -> DeclTree {
        let node = tree.get(key).unwrap();

        assert_eq!(node.parent_key, parent_key);

        let value = *node.value;
        let child_keys = node
            .child_keys
            .iter()
            .map(|&child_key| make_decl_tree(tree, child_key, Some(key)))
            .collect();

        DeclTree { value, child_keys }
    }

    tree.root_key()
        .map(|root_key| make_decl_tree(tree, root_key, None))
}

pub fn make_reverse_key_map(key_map: &HashMap<usize, DefaultKey>) -> HashMap<DefaultKey, usize> {
    let length = key_map.len();

    let mut reverse_map = HashMap::with_capacity(length);

    key_map.iter().for_each(|(&value, &default_key)| {
        let previous_value = reverse_map.insert(default_key, value);
        assert!(previous_value.is_none());
    });

    reverse_map
}
