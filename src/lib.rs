use hashbrown::HashSet;
use slotmap::{
    basic::{
        Iter,
        Keys,
        Values,
    },
    Key,
    SlotMap,
};

pub struct Tree<K, V>
where
    K: Key,
{
    root_key: Option<K>,
    nodes: SlotMap<K, Node<K, V>>,
}

impl<K, V> Tree<K, V>
where
    K: Key,
{
    // Creation methods:

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            root_key: None,
            nodes: SlotMap::with_capacity_and_key(capacity),
        }
    }

    // Check methods:

    pub fn contains(&self, key: K) -> bool {
        self.nodes.contains_key(key)
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    // Insertion/removal methods:

    pub fn insert_root(&mut self, value: V) -> K {
        self.insert_root_with_capacity(value, 0)
    }

    pub fn insert_root_with_capacity(&mut self, value: V, capacity: usize) -> K {
        if self.root_key.is_some() {
            self.clear();
        };

        let root_key = self.nodes.insert(Node {
            parent_key: None,
            child_keys: HashSet::with_capacity(capacity),
            value,
        });
        self.root_key = Some(root_key);

        root_key
    }

    pub fn insert(&mut self, parent_key: K, value: V) -> Option<K> {
        self.insert_with_capacity(parent_key, value, 0)
    }

    pub fn insert_with_capacity(&mut self, parent_key: K, value: V, capacity: usize) -> Option<K> {
        self.nodes.contains_key(parent_key).then(|| {
            self.nodes.insert(Node {
                parent_key: Some(parent_key),
                child_keys: HashSet::with_capacity(capacity),
                value,
            })
        })
    }

    pub fn remove(&mut self, key: K, size_hint: Option<usize>) -> Option<V> {
        self.root_key.and_then(|root_key| {
            if key == root_key {
                let node = self.nodes.remove(key).unwrap();
                self.clear();
                Some(node.value)
            }
            else {
                self.descendent_keys(key, size_hint).map(|descendent_keys| {
                    descendent_keys
                        .into_iter()
                        .skip(1)
                        .for_each(|descendent_key| {
                            self.nodes.remove(descendent_key).unwrap();
                        });

                    let node = self.nodes.remove(key).unwrap();
                    let parent_key = node.parent_key.unwrap();
                    self.nodes
                        .get_mut(parent_key)
                        .unwrap()
                        .child_keys
                        .remove(&key);

                    node.value
                })
            }
        })
    }

    pub fn rebase(&mut self, key: K, parent_key: K) -> bool {
        self.get_relationship(key, parent_key)
            .map_or(false, |relationship| {
                if let Relationship::Ancestral {
                    ancestor_key,
                    descendent_key,
                } = relationship
                {
                    if parent_key == ancestor_key {
                        let node = self.nodes.get_mut(key).unwrap();
                        let current_parent_key = node.parent_key.unwrap();
                        if current_parent_key != parent_key {
                            node.parent_key = Some(parent_key);
                            self.nodes
                                .get_mut(current_parent_key)
                                .unwrap()
                                .child_keys
                                .remove(&key);
                            self.nodes
                                .get_mut(parent_key)
                                .unwrap()
                                .child_keys
                                .insert(key);
                        }
                    }
                    else if parent_key == descendent_key {
                        todo!()
                    }
                    else {
                        unreachable!()
                    }
                }
                else if let Relationship::Siblings { .. } = relationship {
                    let node = self.nodes.get_mut(key).unwrap();
                    let current_parent_key = node.parent_key.unwrap();

                    node.parent_key = Some(parent_key);
                    self.nodes
                        .get_mut(current_parent_key)
                        .unwrap()
                        .child_keys
                        .remove(&key);
                    self.nodes
                        .get_mut(parent_key)
                        .unwrap()
                        .child_keys
                        .insert(key);
                };

                true
            })
    }

    pub fn clear(&mut self) {
        self.root_key = None;
        self.nodes.clear();
    }

    // Getter/setter methods:

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn root_key(&self) -> Option<K> {
        self.root_key
    }

    pub fn get(&self, key: K) -> Option<&Node<K, V>> {
        self.nodes.get(key)
    }

    pub fn get_mut(&mut self, key: K) -> Option<Node<K, &mut V>> {
        self.nodes.get_mut(key).map(|node| Node {
            parent_key: node.parent_key,
            child_keys: node.child_keys.clone(),
            value: &mut node.value,
        })
    }

    pub fn descendent_keys(&self, key: K, size_hint: Option<usize>) -> Option<Vec<K>> {
        self.nodes.contains_key(key).then(|| {
            let size_hint = size_hint.unwrap_or_else(|| self.nodes.len());

            let mut to_visit_keys = self.nodes.get(key).unwrap().child_keys.iter().fold(
                Vec::with_capacity(size_hint),
                |mut vec, &child_key| {
                    vec.push(child_key);
                    vec
                },
            );
            let mut descendent_keys = Vec::with_capacity(size_hint);

            while let Some(to_visit_key) = to_visit_keys.pop() {
                descendent_keys.push(to_visit_key);
                let to_visit_child_keys = &self.nodes.get(to_visit_key).unwrap().child_keys;
                to_visit_keys.extend(to_visit_child_keys);
            }

            descendent_keys
        })
    }

    pub fn get_relationship(&self, key1: K, key2: K) -> Option<Relationship<K>> {
        fn get_relationship<K, V>(tree: &Tree<K, V>, key1: K, key2: K) -> Relationship<K>
        where
            K: Key,
        {
            if key1 == key2 {
                Relationship::Same
            }
            else {
                let mut current_parent_key = tree.get(key1).unwrap().parent_key;
                let length = tree.nodes.len();
                let mut path = HashSet::with_capacity(length);

                loop {
                    match current_parent_key {
                        Some(parent_key) if parent_key == key2 => {
                            return Relationship::Ancestral {
                                ancestor_key: key2,
                                descendent_key: key1,
                            }
                        }
                        Some(parent_key) => {
                            path.insert(parent_key);
                            current_parent_key = tree.get(parent_key).unwrap().parent_key;
                        }
                        None => break,
                    }
                }

                let mut current_parent_key = tree.get(key2).unwrap().parent_key;

                loop {
                    match current_parent_key {
                        Some(parent_key) if parent_key == key1 => {
                            return Relationship::Ancestral {
                                ancestor_key: key1,
                                descendent_key: key2,
                            }
                        }
                        Some(parent_key) => {
                            if path.contains(&parent_key) {
                                return Relationship::Siblings {
                                    common_ancestor_key: parent_key,
                                };
                            }
                            else {
                                current_parent_key = tree.get(parent_key).unwrap().parent_key;
                            }
                        }
                        None => unreachable!(),
                    }
                }
            }
        }

        let key1_exists = self.nodes.contains_key(key1);
        let key2_exists = self.nodes.contains_key(key2);
        let both_keys_exist = key1_exists && key2_exists;

        both_keys_exist.then(|| get_relationship(self, key1, key2))
    }

    // Iter methods:

    pub fn keys(&self) -> Keys<'_, K, Node<K, V>> {
        self.nodes.keys()
    }

    pub fn nodes(&self) -> Values<'_, K, Node<K, V>> {
        self.nodes.values()
    }

    pub fn nodes_mut(&mut self) -> impl Iterator<Item = Node<K, &mut V>> {
        self.nodes.values_mut().map(|node| Node {
            parent_key: node.parent_key,
            child_keys: node.child_keys.clone(),
            value: &mut node.value,
        })
    }

    pub fn iter(&self) -> Iter<'_, K, Node<K, V>> {
        self.nodes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (K, Node<K, &mut V>)> {
        self.nodes.iter_mut().map(|(key, node)| {
            (
                key,
                Node {
                    parent_key: node.parent_key,
                    child_keys: node.child_keys.clone(),
                    value: &mut node.value,
                },
            )
        })
    }
}

impl<K, V> Default for Tree<K, V>
where
    K: Key,
{
    fn default() -> Self {
        Self {
            root_key: None,
            nodes: SlotMap::default(),
        }
    }
}

pub struct Node<K, V> {
    pub parent_key: Option<K>,
    pub child_keys: HashSet<K>,
    pub value: V,
}

pub enum Relationship<K> {
    Same,
    Ancestral { ancestor_key: K, descendent_key: K },
    Siblings { common_ancestor_key: K },
}
