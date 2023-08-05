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
//! let child_value_1 = tree.get(child_key_1).unwrap();
//! assert_eq!(*child_value_1.value, 1);
//!
//! // Or get a mutable reference to one of the children's value:
//! let child_value_2 = tree.get_mut(child_key_2).unwrap();
//! *child_value_2.value = 100;
//! let child_value_2 = tree.get(child_key_2).unwrap();
//! assert_eq!(*child_value_2.value, 100);
//! # }
//! ```

use std::collections::{
    BTreeMap,
    BTreeSet,
};

use petgraph::{
    graphmap::Neighbors,
    prelude::GraphMap,
    Directed,
};
use slotmap::{
    basic::{
        IntoIter,
        Iter,
        IterMut,
        Keys,
        Values,
        ValuesMut,
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
#[derive(Default)]
pub struct Tree<K, V>
where
    K: Key,
{
    root_key: Option<K>,
    values: SlotMap<K, V>,
    up_map: BTreeMap<K, K>,
    down_map: GraphMap<K, (), Directed>,
}

impl<K, V> Tree<K, V>
where
    K: Key,
{
    /// Create a new [`Tree`] instance with the specified `capacity` pre-allocated.
    pub fn with_capacity(nodes_capacity: usize, edges_capacity: usize) -> Self {
        Self {
            root_key: None,
            values: SlotMap::with_capacity_and_key(nodes_capacity),
            up_map: BTreeMap::default(),
            down_map: GraphMap::with_capacity(nodes_capacity, edges_capacity)
        }
    }

    // Check methods:

    /// Checks whether or not this instance of [`Tree`] has the given `key`
    /// inside of it.
    pub fn contains(&self, key: K) -> bool {
        self.values.contains_key(key)
    }

    /// Checks whether or not this [`Tree`] instance is empty (i.e., has no
    /// values inside of it).
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    // Insertion/removal methods:

    /// Inserts a new root value into this [`Tree`] instance.
    ///
    /// If this [`Tree`] instance already contains a root value (and possibly
    /// some children values) inside of it, then this [`Tree`] instance is
    /// first fully cleared and replaced with the new, singular root value.
    pub fn insert_root(&mut self, value: V) -> K {
        if self.root_key.is_some() {
            self.clear();
        };

        let root_key = self.values.insert(value);

        self.down_map.add_node(root_key);
        self.root_key = Some(root_key);

        root_key
    }

    /// Inserts a new child value into this [`Tree`] instance.
    ///
    /// If this [`Tree`] instance does not contain the given `parent_key`, then
    /// [`None`] is returned. Otherwise, returns [`Some(..)`] containing the
    /// new key corresponding to this new child value.
    pub fn insert(&mut self, parent_key: K, value: V) -> Option<K> {
        self.contains(parent_key).then(|| {
            let key = self.values.insert(value);

            self.up_map.insert(key, parent_key);
            self.down_map.add_edge(parent_key, key, ());

            key
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
            }
            else {
                self.descendent_keys_inclusive(key, size_hint)
                    .map(|descendent_keys| {
                        descendent_keys.iter().skip(1).for_each(|&descendent_key| {
                            remove_single_forced(self, descendent_key);
                        });
                        remove_single_forced(self, key)
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
                        let current_parent_key = *self.up_map.get(&key).unwrap();

                        if current_parent_key != parent_key {
                            self.down_map.remove_edge(current_parent_key, key);
                            self.down_map.add_edge(parent_key, key, ());
                            *self.up_map.get_mut(&key).unwrap() = parent_key;
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
                    let current_parent_key = *self.up_map.get(&key).unwrap();

                    self.down_map.remove_edge(current_parent_key, key);
                    self.down_map.add_edge(parent_key, key, ());
                    *self.up_map.get_mut(&key).unwrap() = parent_key;
                };

                true
            })
    }

    /// Clears this [`Tree`] instance of *all* its values. Keeps the allocated
    /// memory for reuse.
    pub fn clear(&mut self) {
        self.values.clear();
        self.up_map.clear();
        self.down_map.clear();
        self.root_key = None;
    }

    // Getter/setter methods:

    /// Returns the number of elements in this [`Tree`] instance.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns the `root_key` of this [`Tree`] instance.
    ///
    /// Returns [`None`] if this [`Tree`] instance is empty. Otherwise, returns
    /// [`Some(..)`] containing the `root_key`.
    pub fn root_key(&self) -> Option<K> {
        self.root_key
    }

    /// Returns an owned iterator over all the keys inside of this [`Tree`]
    /// instance.
    pub fn keys(&self) -> Keys<'_, K, V> {
        self.values.keys()
    }

    /// Returns an immutable iterator over all the values inside of this
    /// [`Tree`] instance.
    pub fn values(&self) -> Values<'_, K, V> {
        self.values.values()
    }

    /// Returns a mutable iterator over all the values inside of this [`Tree`]
    /// instance.
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        self.values.values_mut()
    }

    /// Gets the [`Node`] entry that corresponds to the given `key`.
    ///
    /// If the given `key` does not exist in this [`Tree`] instance, then
    /// [`None`] is returned. Otherwise, returns [`Some(..)`] containing the
    /// [`Node`] entry.
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

    /// Gets the [`NodeMut`] entry that corresponds to the given `key`.
    ///
    /// If the given `key` does not exist in this [`Tree`] instance, then
    /// [`None`] is returned. Otherwise, returns [`Some(..)`] containing the
    /// [`NodeMut`] entry.
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

    /// Returns a [`Vec`] of all the descendent keys of the given `key`
    /// (including the given `key` itself).
    ///
    /// If the given `key` does not exist in this [`Tree`] instance, then
    /// [`None`] is returned. Otherwise, returns [`Some(..)`] containing the
    /// descendent keys (including the given `key`).
    pub fn descendent_keys_inclusive(&self, key: K, size_hint: Option<usize>) -> Option<Vec<K>> {
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

    /// Gets the [`Relationship`] status between two keys.
    ///
    /// If either `key1` or `key2` do not exist in this [`Tree`] instance, then
    /// [`None`] is returned. Otherwise, returns [`Some(..)`] containing the
    /// relationship between the two keys.
    pub fn get_relationship(&self, key1: K, key2: K) -> Option<Relationship<K>> {
        let key1_exists = self.contains(key1);
        let key2_exists = self.contains(key2);
        let both_keys_exist = key1_exists && key2_exists;

        fn find_common_ancestor<K, V>(tree: &Tree<K, V>, key1: K, key2: K) -> Relationship<K>
        where
            K: Key,
        {
            if key1 == key2 {
                Relationship::Same
            }
            else {
                let mut current_parent_key = tree.get(key1).unwrap().parent_key;
                let mut path = BTreeSet::default();

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

        both_keys_exist.then(|| find_common_ancestor(self, key1, key2))
    }

    // Iterator methods:

    /// Create an immutable iterator over the key-value pairs inside of this
    /// [`Tree`] instance.
    ///
    /// The order of iteration is arbitrary. It will not be guaranteed to be
    /// depth-first, breadth-first, in-order, etc.
    pub fn iter(&self) -> Iter<'_, K, V> {
        self.values.iter()
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
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        self.values.iter_mut()
    }
}

impl<K, V> IntoIterator for Tree<K, V>
where
    K: Key,
{
    type IntoIter = IntoIter<K, V>;
    type Item = <Self::IntoIter as IntoIterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

/// A container that wraps over an immutable reference to to the actual
/// underlying value that is stored.
///
/// It also contains:
/// - this value's parent key ([`None`] if this value is the root value)
/// - this value's children keys (an empty iterator if this value has no
///   children)
pub struct Node<'a, K, V> {
    /// The parent key of this value.
    ///
    /// Is [`None`] iff this value is the root value.
    pub parent_key: Option<K>,

    /// An immutable reference to the actual underlying value that is stored.
    pub value: &'a V,

    /// The children keys of this value.
    pub child_keys: Neighbors<'a, K, Directed>,
}

/// A container that wraps over a mutable reference to the actual underlying
/// value that is stored.
///
/// It also contains:
/// - this value's parent key ([`None`] if this value is the root value)
/// - this value's children keys (an empty iterator if this value has no
///   children)
pub struct NodeMut<'a, K, V> {
    /// The parent key of this value.
    ///
    /// Is [`None`] iff this value is the root value.
    pub parent_key: Option<K>,

    /// A mutable reference to the actual underlying value that is stored.
    pub value: &'a mut V,

    /// The children keys of this value.
    pub child_keys: Neighbors<'a, K, Directed>,
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
        /// The ancestor in this relationship.
        ancestor_key: K,

        /// The descendent in this relationship.
        descendent_key: K,
    },

    /// The two keys are not related through the same lineage. If you traverse
    /// up either key's lineage, you will *not* encounter the other.
    ///
    /// However, since all keys' parental lineage converges to one singular key,
    /// these two keys are related through some common ancestor. Therefore, they
    /// are considered "siblings".
    Siblings {
        /// The common ancestor that both of these keys originate from.
        common_ancestor_key: K,
    },
}