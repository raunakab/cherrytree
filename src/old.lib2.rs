#![warn(missing_docs)]

//! A small, simple, and correct tree implementation.
//!
//! # Overview:
//! `pettree` is a library which exports safe and correct APIs for interacting
//! with tree data structures. The main way it does this is by exporting a
//! generic [`Tree`] type with associated methods to read and write to it.
//! ...But what exactly is a tree?
//!
//! ## Theory:
//! Formally, a [tree](https://en.wikipedia.org/wiki/Tree_(data_structure)) is
//! just a [graph](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)).
//! However, it's not just any type of graph; a tree is a special type of graph
//! that enforces certain invariants on the structure of its nodes and edges.
//!
//! Namely, the following invariants must be held:
//! - Each node (except for one) must have exactly 1 other node pointing to it
//! - The remaining node must have 0 other nodes pointing to it
//!
//! If any arbitrary graph meets the above requirements, then it can also be
//! classified as a tree!
//!
//! ## Implementation:
//! Therefore, internally, the [`Tree`] type is just a graph with the above
//! invariants enforced. Any mutations or modifications to this [`Tree`] will
//! *continue* to enforce these invariants, thus ensuring the end-user of a safe
//! and correct implementation of a tree for whatever purpose they desire.
//!
//! # Example:
//! ```
//! # fn main() {
//! use pettree::{
//!     Node,
//!     Tree,
//! };
//! use slotmap::DefaultKey;
//!
//! // Create a default, empty tree:
//! let mut tree = Tree::<DefaultKey, usize>::default();
//!
//! // Insert a root value:
//! let root_key = tree.insert_root(0);
//!
//! // Insert some children values:
//! let child_key_1 = tree.insert(root_key, 1).unwrap();
//! let child_key_2 = tree.insert(root_key, 2).unwrap();
//! let child_key_3 = tree.insert(root_key, 3).unwrap();
//!
//! // Get an immutable reference to one of the children's value:
//! let child_node_1 = tree.get(child_key_1).unwrap();
//! assert_eq!(child_node_1.value, 1);
//!
//! // Or get a mutable reference to one of the children's value:
//! let child_node_2 = tree.get_mut(child_key_2).unwrap();
//! *child_node_2.value = 100;
//! let child_node_2 = tree.get(child_key_2).unwrap();
//! assert_eq!(child_node_2.value, 100);
//! # }
//! ```

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

/// The data-structure containing all the data required to implement a fully
/// function arbitrary-arity-tree.
///
/// Exposes a number of APIs that allow the end-user to manipulate this [`Tree`]
/// (e.g., such as adding children values, removing values, etc.).
///
/// Internally, this [`Tree`] type uses a [`SlotMap`] to keep a track of each
/// value in the tree. This gives us the ability to index into the [`Tree`] (in
/// constant time) and retrieve whatever value is stored!
///
/// Since indexing requires a key, every insertion into a [`Tree`] will produce
/// a *unique* key that can be used to identify the value being inserted.
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

    /// Create a new [`Tree`] instance with the specified `capacity`
    /// pre-allocated.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            root_key: None,
            nodes: SlotMap::with_capacity_and_key(capacity),
        }
    }

    // Check methods:

    /// Checks whether or not this [`Tree`] instance has the given `key` inside
    /// of it.
    pub fn contains(&self, key: K) -> bool {
        self.nodes.contains_key(key)
    }

    /// Checks whether or not this [`Tree`] instance is empty (i.e., has no
    /// values inside of it).
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    // Insertion/removal methods:

    /// Inserts a new root value into this [`Tree`] instance.
    ///
    /// If this [`Tree`] instance already contains a root value (and possibly
    /// some children values), then this [`Tree`] instance is first fully
    /// cleared and then replaced with the new, singular root value.
    pub fn insert_root(&mut self, value: V) -> K {
        self.insert_root_with_capacity(value, 0)
    }

    /// Inserts a new root value into this [`Tree`] instance with a capacity
    /// specifying the number of children that this value will have.
    ///
    /// If this [`Tree`] instance already contains a root value (and possibly
    /// some children values), then this [`Tree`] instance is first fully
    /// cleared and then replaced with the new, singular root value.
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

    /// Inserts a new child value into this [`Tree`] instance.
    ///
    /// If this [`Tree`] instance does not contain the given `parent_key`, then
    /// [`None`] is returned. Otherwise, returns [`Some(..)`] containing the
    /// new key corresponding to this new child value.
    pub fn insert(&mut self, parent_key: K, value: V) -> Option<K> {
        self.insert_with_capacity(parent_key, value, 0)
    }

    /// Inserts a new child value into this [`Tree`] instance with a capacity
    /// specifying the number of children that this value will have.
    ///
    /// If this [`Tree`] instance does not contain the given `parent_key`, then
    /// [`None`] is returned. Otherwise, returns [`Some(..)`] containing the
    /// new key corresponding to this new child value.
    pub fn insert_with_capacity(&mut self, parent_key: K, value: V, capacity: usize) -> Option<K> {
        self.nodes.contains_key(parent_key).then(|| {
            self.nodes.insert(Node {
                parent_key: Some(parent_key),
                child_keys: HashSet::with_capacity(capacity),
                value,
            })
        })
    }

    /// Removes the value corresponding to the given `key` from this [`Tree`]
    /// instance as well as *all* of its children values.
    ///
    /// If this [`Tree`] instance does not contain the given `key`, then
    /// [`None`] is returned. Otherwise, returns [`Some(..)`] containing the
    /// removed value.
    ///
    /// The `size_hint` argument should be used to provide some sort of guess
    /// for how many children the given `key` has inside of this [`Tree`]
    /// instance. If you do not have a hint, then provide [`None`] as an
    /// argument.
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

    /// Rebase the subtree rooted at `key` to be a child underneath the subtree
    /// rooted at `parent_key`.
    ///
    /// After performing this operation, the new parent of `key` will be
    /// `parent_key`.
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

    /// Clears this [`Tree`] instance of *all* its values. Keeps the allocated
    /// memory for reuse.
    pub fn clear(&mut self) {
        self.root_key = None;
        self.nodes.clear();
    }

    // Getter/setter methods:

    /// Returns the number of elements in this [`Tree`] instance.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the `root_key` of this [`Tree`] instance.
    ///
    /// Returns [`None`] if this [`Tree`] instance is empty. Otherwise, returns
    /// [`Some(..)`] containing the `root_key`.
    pub fn root_key(&self) -> Option<K> {
        self.root_key
    }

    /// Gets a reference to the [`Node`] entry that corresponds to the given
    /// `key`.
    ///
    /// If the given `key` does not exist in this [`Tree`] instance, then
    /// [`None`] is returned. Otherwise, returns [`Some(..)`] containing the
    /// [`Node`] entry.
    pub fn get(&self, key: K) -> Option<&Node<K, V>> {
        self.nodes.get(key)
    }

    /// Gets an owned [`Node`] which contains a mutable reference to the
    /// underlying value that corresponds to the given `key`.
    ///
    /// If the given `key` does not exist in this [`Tree`] instance, then
    /// [`None`] is returned. Otherwise, returns [`Some(..)`] containing the
    /// [`Node`] entry.
    pub fn get_mut(&mut self, key: K) -> Option<Node<K, &mut V>> {
        self.nodes.get_mut(key).map(|node| Node {
            parent_key: node.parent_key,
            child_keys: node.child_keys.clone(),
            value: &mut node.value,
        })
    }

    /// Returns a [`Vec`] of all the descendent keys of the given `key`
    /// (not including the given `key` itself).
    ///
    /// If the given `key` does not exist in this [`Tree`] instance, then
    /// [`None`] is returned. Otherwise, returns [`Some(..)`] containing the
    /// descendent keys (including the given `key`).
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

    /// Gets the [`Relationship`] status between two keys.
    ///
    /// If either `key1` or `key2` do not exist in this [`Tree`] instance, then
    /// [`None`] is returned. Otherwise, returns [`Some(..)`] containing the
    /// relationship between the two keys.
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

    /// Returns an owned iterator over all the keys inside of this [`Tree`]
    /// instance.
    pub fn keys(&self) -> Keys<'_, K, Node<K, V>> {
        self.nodes.keys()
    }

    /// Returns an immutable iterator over all the [`Node`]s inside of this
    /// [`Tree`] instance.
    pub fn nodes(&self) -> Values<'_, K, Node<K, V>> {
        self.nodes.values()
    }

    /// Returns a mutable iterator over all the [`Node`]s inside of this
    /// [`Tree`] instance.
    pub fn nodes_mut(&mut self) -> impl Iterator<Item = Node<K, &mut V>> {
        self.nodes.values_mut().map(|node| Node {
            parent_key: node.parent_key,
            child_keys: node.child_keys.clone(),
            value: &mut node.value,
        })
    }

    /// Create an immutable iterator over the key-value pairs inside of this
    /// [`Tree`] instance.
    ///
    /// The order of iteration is arbitrary. It will not be guaranteed to be
    /// depth-first, breadth-first, in-order, etc.
    pub fn iter(&self) -> Iter<'_, K, Node<K, V>> {
        self.nodes.iter()
    }

    /// Create a mutable iterator over the key-value pairs inside of this
    /// [`Tree`] instance.
    ///
    /// Note that this iterator will yield elements of type `(K, &mut V)`.
    /// Namely, this function only provides mutable access to the values, not
    /// the keys!
    ///
    /// The order of iteration is arbitrary. It will not be guaranteed to be
    /// depth-first, breadth-first, in-order, etc.
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

/// A container over the underlying value inside of this [`Tree`] instance.
///
/// It contains the actual underlying value, as well as its `parent_key` and
/// `child_keys`.
pub struct Node<K, V> {
    /// The parent key of this value.
    ///
    /// Is [`None`] iff this value is the root value.
    pub parent_key: Option<K>,

    /// The children keys of this value.
    pub child_keys: HashSet<K>,

    /// The actual underlying value that is stored.
    pub value: V,
}

/// A description of the relationship between two keys in a [`Tree`] instance.
///
/// Each variant of a [`Relationship`] is based off of familial relationships
/// (i.e., parents, grandparent, great-grandparents are all your ancestors).
pub enum Relationship<K> {
    /// The two keys are the exact same.
    Same,

    /// The two keys are related through the same lineage. Namely, the
    /// `ancestor_key` can be found by traversing up the `descendent_key`'s
    /// parental lineage.
    ///
    /// In simpler terms, the `ancestor_key` is the parent of the
    /// `descendent_key`, OR the parent's parent of the `descendent_key`, OR
    /// the parent's parent's parent of the `descendent_key`, etc.
    Ancestral {
        /// The ancestor key in this relationship.
        ancestor_key: K,

        /// The descendent key in this relationship.
        descendent_key: K,
    },

    /// The two keys are not related through the same lineage. If you traverse
    /// up either key's lineage, you will *not* encounter the other.
    ///
    /// However, since all keys' parental lineage converges to one singular key,
    /// these two keys are related through some common ancestor. Therefore, they
    /// are considered "siblings".
    Siblings {
        /// The common ancestor key that both of these keys originate from.
        common_ancestor_key: K,
    },
}