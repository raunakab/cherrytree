use hashbrown::HashSet;
use slotmap::{SlotMap, Key};

pub(super) struct InnerTree<K, V> where K: Key {
    root_key: K,
    map: SlotMap<K, Node<K, V>>,
}

impl<K, V> InnerTree<K, V> where K: Key {
    // Check methods:

    pub(super) fn contains(&self, key: K) -> bool {
        self.map.contains_key(key)
    }

    // Insertion/removal methods:

    pub(super) fn insert_root(&mut self, value: V) -> K {
        self.map.clear();

        let root_key = self.map.insert(Node { value, parent: None, children: HashSet::default() });
        self.root_key = root_key;

        root_key
    }

    pub(super) fn insert(&mut self, value: V, parent_key: K) -> Option<K> {
        self.map
            .contains_key(parent_key)
            .then(|| {
                self.map.insert(Node { value, parent: Some(parent_key), children: HashSet::default() })
            })
    }

    pub(super) fn rebase(&mut self, _: K, _: K) -> bool {
        todo!()
    }

    // pub(super) fn 
}

struct Node<K, V> {
    value: V,
    parent: Option<K>,
    children: HashSet<K>,
}
