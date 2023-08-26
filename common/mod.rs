#![allow(unused)]

macro_rules! node {
    ($value:expr$(,)?) => { DeserialNode($value, vec![]) };

    (
        $value:expr,
        [$($child:expr),*$(,)?]$(,)?
    ) => {
        DeserialNode($value, vec![$($child),*])
    };
}

use std::collections::{
    HashMap,
    HashSet,
};

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

        let previous_key = key_map.insert(deserial_node.0, key);
        assert!(previous_key.is_none());

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
    fn make_deserial_node(tree: &Tree<DefaultKey, usize>, key: DefaultKey, parent_key: Option<DefaultKey>) -> DeserialNode {
        let node = tree.get(key).unwrap();

        assert_eq!(node.parent_key, parent_key);

        let value = *node.value;
        let child_keys = node
            .child_keys
            .iter()
            .map(|&child_key| make_deserial_node(tree, child_key, Some(key)))
            .collect();

        DeserialNode(value, child_keys)
    }

    tree.root_key()
        .map(|root_key| make_deserial_node(tree, root_key, None))
}

pub fn make_reverse_key_map(key_map: &HashMap<Value, DefaultKey>) -> HashMap<DefaultKey, Value> {
    let length = key_map.len();

    let mut reverse_map = HashMap::with_capacity(length);

    key_map.iter().for_each(|(&value, &default_key)| {
        let previous_value = reverse_map.insert(default_key, value);
        assert!(previous_value.is_none());
    });

    reverse_map
}
