#[macro_export]
macro_rules! node {
    (
        $id:expr,
        $value:expr
        $(,)?
    ) => {{
        use crate::tests::utils::DeclarativeNode;
        DeclarativeNode {
            id: $id,
            value: $value,
            child_declarative_nodes: vec![],
        }
    }};

    (
        $id:expr,
        $value:expr,
        [
            $($child:expr),*
            $(,)?
        ]
        $(,)?
    ) => {{
        use crate::tests::utils::DeclarativeNode;
        DeclarativeNode {
            id: $id,
            value: $value,
            child_declarative_nodes: vec![$($child),*],
        }
    }};
}

use std::collections::BTreeMap;

pub use node;
use slotmap::DefaultKey;

use crate::{
    Relationship,
    Tree,
};

#[derive(Clone)]
pub struct DeclarativeTree<K, V>
where
    K: Copy + Ord,
    V: Copy,
{
    tree: Tree<DefaultKey, V>,
    key_map: BTreeMap<K, DefaultKey>,
}

impl<K, V> DeclarativeTree<K, V>
where
    K: Copy + Ord,
    V: Copy,
{
    // From/Into methods:

    pub fn from_declarative_node(declarative_node: Option<&DeclarativeNode<K, V>>) -> Self {
        declarative_node.into()
    }

    pub fn into_declarative_node(&self) -> Option<DeclarativeNode<K, V>> {
        self.into()
    }

    // Insertion/removal methods:

    pub fn insert_root(&mut self, root_id: K, value: V) {
        let root_key = self.tree.insert_root(value);
        let previous_value = self.key_map.insert(root_id, root_key);
        assert!(previous_value.is_none());
    }

    pub fn insert(&mut self, id: K, value: V, parent_id: K) -> bool {
        let parent_key = get_or_default(&self.key_map, parent_id);

        self.tree
            .insert(value, parent_key)
            .map(|key| {
                let previous_value = self.key_map.insert(id, key);
                assert!(previous_value.is_none());
            })
            .is_some()
    }

    pub fn reorder_children<F>(&mut self, id: K, get_reordered_ids: F) -> bool
    where
        F: FnOnce(&Vec<K>) -> Vec<K>,
    {
        let key = get_or_default(&self.key_map, id);
        let old_child_keys = self.tree.get(key).map(|node| node.child_keys.clone());

        let key_map = self.key_map.clone();
        let inverse_key_map = invert(&self.key_map);

        let did_reorder = self.tree.reorder_children(key, |current_child_keys| {
            let current_child_ids = current_child_keys
                .iter()
                .map(|current_child_key| *inverse_key_map.get(&current_child_key).unwrap())
                .collect();

            let mut reordered_children = get_reordered_ids(&current_child_ids);
            reordered_children.dedup();
            reordered_children
                .iter()
                .map(|&id| get_or_default(&key_map, id))
                .collect()
        });

        if did_reorder {
            let old_child_keys = old_child_keys.unwrap();
            let new_child_keys = self
                .tree
                .get(key)
                .map(|node| node.child_keys.clone())
                .unwrap();

            let mut inverse_key_map = inverse_key_map;

            for &key in old_child_keys.difference(&new_child_keys) {
                let id = inverse_key_map.remove(&key).unwrap();
                let removed_key = self.key_map.remove(&id).unwrap();
                assert_eq!(key, removed_key);
            }

            assert!(new_child_keys.difference(&old_child_keys).next().is_none());
        }

        did_reorder
    }

    pub fn remove(&mut self, id: K) -> Option<V> {
        let key = get_or_default(&self.key_map, id);
        let value = self.tree.remove(key, None);

        if value.is_some() {
            self.key_map.remove(&id).unwrap();
        };

        value
    }

    pub fn rebase(&mut self, id: K, new_parent_id: K) -> bool {
        let key = get_or_default(&self.key_map, id);
        let new_parent_key = get_or_default(&self.key_map, new_parent_id);

        self.tree.rebase(key, new_parent_key)
    }

    // Getter/setter methods:

    pub fn get_relationship(&self, id_1: K, id_2: K) -> Option<Relationship<K>> {
        let key_1 = get_or_default(&self.key_map, id_1);
        let key_2 = get_or_default(&self.key_map, id_2);

        self.tree
            .get_relationship(key_1, key_2)
            .map(|relationship| match relationship {
                Relationship::Same => Relationship::Same,
                Relationship::Ancestral {
                    ancestor_key,
                    descendent_key,
                } => {
                    if ancestor_key == key_1 && descendent_key == key_2 {
                        Relationship::Ancestral {
                            ancestor_key: id_1,
                            descendent_key: id_2,
                        }
                    }
                    else if ancestor_key == key_2 && descendent_key == key_1 {
                        Relationship::Ancestral {
                            ancestor_key: id_2,
                            descendent_key: id_1,
                        }
                    }
                    else {
                        unreachable!()
                    }
                }
                Relationship::Siblings {
                    common_ancestor_key,
                } => {
                    let inverse_key_map = invert(&self.key_map);
                    let common_ancestor_id = *inverse_key_map.get(&common_ancestor_key).unwrap();
                    Relationship::Siblings {
                        common_ancestor_key: common_ancestor_id,
                    }
                }
            })
    }
}

impl<K, V> Default for DeclarativeTree<K, V>
where
    K: Copy + Ord,
    V: Copy,
{
    fn default() -> Self {
        Self {
            tree: Tree::default(),
            key_map: BTreeMap::default(),
        }
    }
}

impl<'a, K, V> From<Option<&'a DeclarativeNode<K, V>>> for DeclarativeTree<K, V>
where
    K: Copy + Ord,
    V: Copy,
{
    fn from(declarative_node: Option<&'a DeclarativeNode<K, V>>) -> Self {
        fn construct<K, V>(
            tree: &mut Tree<DefaultKey, V>,
            key_map: &mut BTreeMap<K, DefaultKey>,
            declarative_node: &DeclarativeNode<K, V>,
            parent_key: Option<DefaultKey>,
        ) where
            K: Copy + Ord,
            V: Copy,
        {
            let key = match parent_key {
                Some(parent_key) => tree.insert(declarative_node.value, parent_key).unwrap(),
                None => tree.insert_root(declarative_node.value),
            };

            let previous_value = key_map.insert(declarative_node.id, key);
            assert!(previous_value.is_none());

            declarative_node
                .child_declarative_nodes
                .iter()
                .for_each(|child_declarative_node| {
                    construct(tree, key_map, child_declarative_node, Some(key));
                });
        }

        declarative_node.map_or_else(Self::default, |declarative_node| {
            let mut tree = Tree::default();
            let mut key_map = BTreeMap::default();

            construct(&mut tree, &mut key_map, declarative_node, None);

            Self { tree, key_map }
        })
    }
}

impl<'a, K, V> From<&'a DeclarativeTree<K, V>> for Option<DeclarativeNode<K, V>>
where
    K: Copy + Ord,
    V: Copy,
{
    fn from(declarative_tree: &'a DeclarativeTree<K, V>) -> Self {
        fn construct<K, V>(
            tree: &Tree<DefaultKey, V>,
            inverse_key_map: &BTreeMap<DefaultKey, K>,
            key: DefaultKey,
            parent_key: Option<DefaultKey>,
        ) -> DeclarativeNode<K, V>
        where
            K: Copy + Ord,
            V: Copy,
        {
            let node = tree.get(key).unwrap();

            assert_eq!(parent_key, node.parent_key);

            let id = *inverse_key_map.get(&key).unwrap();
            let child_declarative_nodes = node
                .child_keys
                .iter()
                .map(|&child_key| construct(tree, inverse_key_map, child_key, Some(key)))
                .collect();

            DeclarativeNode {
                id,
                value: *node.value,
                child_declarative_nodes,
            }
        }

        declarative_tree.tree.root_key().map(|root_key| {
            let inverse_key_map = invert(&declarative_tree.key_map);
            construct(&declarative_tree.tree, &inverse_key_map, root_key, None)
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclarativeNode<K, V>
where
    K: Copy + Ord,
    V: Copy,
{
    pub id: K,
    pub value: V,
    pub child_declarative_nodes: Vec<Self>,
}

fn get_or_default<K>(key_map: &BTreeMap<K, DefaultKey>, id: K) -> DefaultKey
where
    K: Copy + Ord,
{
    key_map.get(&id).copied().unwrap_or_default()
}

pub fn invert<A, B>(key_map: &BTreeMap<A, B>) -> BTreeMap<B, A>
where
    A: Copy + Ord,
    B: Copy + Ord,
{
    let mut inverse_map = BTreeMap::default();

    key_map.iter().for_each(|(&value, &default_key)| {
        let previous_value = inverse_map.insert(default_key, value);
        assert!(previous_value.is_none());
    });

    inverse_map
}
