use std::collections::BTreeMap;

use petgraph::{
    graphmap::Neighbors,
    prelude::GraphMap,
    Directed,
};
use slotmap::{
    basic::{
        Keys,
        Values,
        ValuesMut,
    },
    Key,
    SlotMap,
};

pub struct Tree<K, V>
where
    K: Key,
{
    values: SlotMap<K, V>,
    up_map: BTreeMap<K, K>,
    down_map: GraphMap<K, (), Directed>,
    root_key: Option<K>,
}

/// # Purpose:
/// Checking/assertion methods.
impl<K, V> Tree<K, V>
where
    K: Key,
{
    pub fn contains(&self, key: K) -> bool {
        self.values.contains_key(key)
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

/// # Purpose:
/// Insertion/removal methods.
impl<K, V> Tree<K, V>
where
    K: Key,
{
    pub fn insert_root(&mut self, value: V) -> K {
        if self.root_key.is_some() {
            self.clear();
        };

        let root_key = self.values.insert(value);
        self.down_map.add_node(root_key);
        self.root_key = Some(root_key);

        root_key
    }

    pub fn insert(&mut self, parent_key: K, value: V) -> Option<K> {
        self.contains(parent_key).then(|| {
            let key = self.values.insert(value);
            self.up_map.insert(key, parent_key);
            self.down_map.add_edge(parent_key, key, ());
            key
        })
    }

    pub fn remove(&mut self, key: K, size_hint: Option<usize>) -> Option<V> {
        let remove_single_forced = |tree: &mut Tree<_, _>, key| {
            let value = tree.values.remove(key).unwrap();

            tree.up_map.remove(&key);
            tree.down_map.remove_node(key);

            value
        };

        self.root_key.and_then(|root_key| {
            if key == root_key {
                let value = remove_single_forced(self, key);
                self.clear();
                Some(value)
            } else {
                self.descendent_keys(key, size_hint).map(|descendent_keys| {
                    descendent_keys.iter().skip(1).for_each(|&descendent_key| {
                        remove_single_forced(self, descendent_key);
                    });
                    remove_single_forced(self, key)
                })
            }
        })
    }

    pub fn clear(&mut self) {
        self.values.clear();
        self.up_map.clear();
        self.down_map.clear();
        self.root_key = None;
    }
}

/// # Purpose:
/// Getter/setter methods.
impl<K, V> Tree<K, V>
where
    K: Key,
{
    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn keys(&self) -> Keys<'_, K, V> {
        self.values.keys()
    }

    pub fn values(&self) -> Values<'_, K, V> {
        self.values.values()
    }

    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        self.values.values_mut()
    }

    pub fn get(&self, key: K) -> Option<Node<'_, K, V>> {
        self.values.get(key).map(|value| {
            let parent_key = self.up_map.get(&key).copied();
            let child_keys = self.down_map.neighbors(key);

            Node {
                parent_key,
                value,
                child_keys,
            }
        })
    }

    pub fn get_mut(&mut self, key: K) -> Option<NodeMut<'_, K, V>> {
        self.values.get_mut(key).map(|value| {
            let parent_key = self.up_map.get(&key).copied();
            let child_keys = self.down_map.neighbors(key);

            NodeMut {
                parent_key,
                value,
                child_keys,
            }
        })
    }

    pub fn descendent_keys(&self, key: K, size_hint: Option<usize>) -> Option<Vec<K>> {
        self.contains(key).then(|| {
            let size_hint = size_hint.unwrap_or_else(|| self.len());

            let mut to_visit_keys = Vec::with_capacity(size_hint);
            let mut descendent_keys = Vec::with_capacity(size_hint);

            to_visit_keys.push(key);

            while let Some(to_visit_key) = to_visit_keys.pop() {
                descendent_keys.push(to_visit_key);
                let to_visit_child_keys = self.down_map.neighbors(to_visit_key);
                to_visit_keys.extend(to_visit_child_keys);
            }

            descendent_keys
        })
    }
}

/// # Purpose:
/// Default creation of a [`Tree`] instance.
impl<K, D> Default for Tree<K, D>
where
    K: Key,
{
    fn default() -> Self {
        Self {
            values: SlotMap::default(),
            up_map: BTreeMap::default(),
            down_map: GraphMap::default(),
            root_key: None,
        }
    }
}

pub struct Node<'a, K, V> {
    pub parent_key: Option<K>,
    pub value: &'a V,
    pub child_keys: Neighbors<'a, K, Directed>,
}

pub struct NodeMut<'a, K, V> {
    pub parent_key: Option<K>,
    pub value: &'a mut V,
    pub child_keys: Neighbors<'a, K, Directed>,
}
