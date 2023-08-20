macro_rules! node {
    ($value:expr$(,)?) => { DeserialNode($value, vec![]) };

    (
        $value:expr,
        [$($child:expr),*$(,)?]$(,)?
    ) => {
        DeserialNode($value, vec![$($child),*])
    };
}

use std::collections::HashMap;

pub(crate) use node;
use pettree::Tree;
use slotmap::DefaultKey;

pub type Value = usize;

#[derive(Debug, PartialEq, Eq)]
pub struct DeserialNode(pub Value, pub Vec<Self>);

pub fn make_tree_and_key_map(
    deserial_node: Option<&DeserialNode>,
) -> (Tree<DefaultKey, Value>, HashMap<Value, DefaultKey>) {
    let mut tree = Tree::default();
    let mut key_map = HashMap::default();

    fn insert(
        tree: &mut Tree<DefaultKey, Value>,
        key_map: &mut HashMap<usize, DefaultKey>,
        deserial_node: &DeserialNode,
        parent_key: Option<DefaultKey>,
    ) {
        let key = match parent_key {
            Some(parent_key) => tree.insert(deserial_node.0, parent_key).unwrap(),
            None => tree.insert_root(deserial_node.0),
        };
        key_map.insert(deserial_node.0, key);
        deserial_node
            .1
            .iter()
            .for_each(|child_deserial_node| insert(tree, key_map, child_deserial_node, Some(key)));
    }

    if let Some(deserial_node) = deserial_node {
        insert(&mut tree, &mut key_map, deserial_node, None);
    };

    (tree, key_map)
}

pub fn make_deserial_node(tree: &Tree<DefaultKey, Value>) -> Option<DeserialNode> {
    fn make_deserial_node(tree: &Tree<DefaultKey, usize>, key: DefaultKey, depth: usize) -> DeserialNode {
        let node = tree.get(key).unwrap();

        assert_eq!(node.depth, depth);

        let value = *node.value;
        let child_keys = node
            .child_keys
            .iter()
            .map(|&child_key| make_deserial_node(tree, child_key, depth + 1))
            .collect();

        DeserialNode(value, child_keys)
    }

    tree.root_key()
        .map(|root_key| make_deserial_node(tree, root_key, 0))
}
