use std::collections::BTreeMap;

use petgraph::{
    prelude::GraphMap,
    Directed,
};
use slotmap::{
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
/// Insertion/removal methods.
impl<K, V> Tree<K, V>
where
    K: Key,
{
    /// # Purpose:
    /// Inserts a new root node into this [`Tree`].
    ///
    /// # Note:
    /// In the case that this [`Tree`] already contained a root node, the entire
    /// [`Tree`] is cleared and replaced with this new root node.
    pub fn insert_root(&mut self, value: V) -> K {
        if self.root_key.is_some() {
            self.clear();
        };

        let root_key = self.values.insert(value);
        self.down_map.add_node(root_key);
        self.root_key = Some(root_key);

        root_key
    }

    pub fn try_insert(&mut self, parent_key: K, value: V) -> Option<K> {
        todo!()
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        todo!()
    }

    /// # Purpose:
    /// Clears the entire [`Tree`] instance of all nodes and edges. Keeps the
    /// allocated memory for reuse.
    pub fn clear(&mut self) {
        self.values.clear();
        self.up_map.clear();
        self.down_map.clear();
        self.root_key = None;
    }
}
