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
//! assert_eq!(*child_node_1.value, 1);
//!
//! // Or get a mutable reference to one of the children's value:
//! let child_node_2 = tree.get_mut(child_key_2).unwrap();
//! *child_node_2.value = 100;
//! let child_node_2 = tree.get(child_key_2).unwrap();
//! assert_eq!(*child_node_2.value, 100);
//! # }
//! ```

use hashbrown::HashSet;
use slotmap::{
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
    inner_nodes: SlotMap<K, InnerNode<K, V>>,
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
            inner_nodes: SlotMap::with_capacity_and_key(capacity),
        }
    }

    // Check methods:

    /// Checks whether or not this [`Tree`] instance has the given `key` inside
    /// of it.
    pub fn contains(&self, key: K) -> bool {
        self.inner_nodes.contains_key(key)
    }

    /// Checks whether or not this [`Tree`] instance is empty (i.e., has no
    /// values inside of it).
    pub fn is_empty(&self) -> bool {
        self.inner_nodes.is_empty()
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

        let root_key = self.inner_nodes.insert(InnerNode {
            parent_key: None,
            child_keys: HashSet::with_capacity(capacity),
            value,
            depth: 0,
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
        self.inner_nodes.contains_key(parent_key).then(|| {
            // # Note:
            // The reason why we `get(parent_key)` once and then do a `get_mut(parent_key)`
            // a second time is because of mutable borrowship issues.
            //
            // Potential source for optimization at a future point (although this would
            // likely require `unsafe`).

            let parent_depth = self.inner_nodes.get(parent_key).unwrap().depth;

            let key = self.inner_nodes.insert(InnerNode {
                parent_key: Some(parent_key),
                child_keys: HashSet::with_capacity(capacity),
                value,
                depth: parent_depth + 1,
            });

            self.inner_nodes
                .get_mut(parent_key)
                .unwrap()
                .child_keys
                .insert(key);

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
        self.root_key.and_then(|root_key| {
            if key == root_key {
                let node = self.inner_nodes.remove(key).unwrap();
                self.clear();
                Some(node.value)
            }
            else {
                self.get_descendent_keys(key, size_hint)
                    .map(|descendent_keys| {
                        descendent_keys.into_iter().for_each(|descendent_key| {
                            self.inner_nodes.remove(descendent_key).unwrap();
                        });

                        let node = self.inner_nodes.remove(key).unwrap();
                        let parent_key = node.parent_key.unwrap();
                        self.inner_nodes
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
        /// Performs a rebase where the `parent_key` is a sibling of `key`.
        fn rebase_onto_sibling<K, V>(tree: &mut Tree<K, V>, key: K, parent_key: K)
        where
            K: Key,
        {
            let node = tree.inner_nodes.get_mut(key).unwrap();
            let current_parent_key = node.parent_key.unwrap();

            node.parent_key = Some(parent_key);

            tree.inner_nodes
                .get_mut(current_parent_key)
                .unwrap()
                .child_keys
                .remove(&key);

            tree.inner_nodes
                .get_mut(parent_key)
                .unwrap()
                .child_keys
                .insert(key);
        }

        /// Performs a rebase where the `parent_key` is an ancestor of `key`.
        fn rebase_onto_ancestor<K, V>(tree: &mut Tree<K, V>, key: K, parent_key: K)
        where
            K: Key,
        {
            let node = tree.inner_nodes.get_mut(key).unwrap();
            let current_parent_key = node.parent_key.unwrap();

            if current_parent_key != parent_key {
                node.parent_key = Some(parent_key);

                tree.inner_nodes
                    .get_mut(current_parent_key)
                    .unwrap()
                    .child_keys
                    .remove(&key);

                tree.inner_nodes
                    .get_mut(parent_key)
                    .unwrap()
                    .child_keys
                    .insert(key);
            };
        }

        /// Performs a rebase where the `parent_key` is a decscendent of `key`.
        fn rebase_onto_descendent<K, V>(tree: &mut Tree<K, V>, key: K, parent_key: K)
        where
            K: Key,
        {
            // let node = tree.inner_nodes.get_mut(key).unwrap();
            // match node.parent_key {
            //     Some(..) => todo!(),
            //     None => {
            //         node.parent_key = Some(parent_key);
            //         let parent_node = tree.inner_nodes.get_mut(parent_key).unwrap();
            //         parent_node.child_keys.insert(key);
            //         let current_parent_key = parent_node.parent_key.unwrap();
            //         parent_node.parent_key = None;
            //         tree.inner_nodes.get_mut(current_parent_key).unwrap().child_keys.remove(&parent_key);
            //     },
            // }

            todo!()
        }

        /// Rebase `key` onto `parent_key` when their [`Relationship`] has been
        /// properly determined.
        fn rebase<K, V>(tree: &mut Tree<K, V>, relationship: Relationship<K>, key: K, parent_key: K)
        where
            K: Key,
        {
            match relationship {
                Relationship::Same => (),
                Relationship::Ancestral {
                    ancestor_key,
                    descendent_key,
                } => {
                    if parent_key == ancestor_key {
                        rebase_onto_ancestor(tree, key, parent_key);
                    }
                    else if parent_key == descendent_key {
                        rebase_onto_descendent(tree, key, parent_key);
                    }
                    else {
                        unreachable!()
                    }
                }
                Relationship::Siblings { .. } => rebase_onto_sibling(tree, key, parent_key),
            };
        }

        self.get_relationship(key, parent_key)
            .map_or(false, |relationship| {
                rebase(self, relationship, key, parent_key);

                true
            })
    }

    /// Clears this [`Tree`] instance of *all* its values. Keeps the allocated
    /// memory for reuse.
    pub fn clear(&mut self) {
        self.root_key = None;
        self.inner_nodes.clear();
    }

    // Getter/setter methods:

    /// Returns the number of elements in this [`Tree`] instance.
    pub fn len(&self) -> usize {
        self.inner_nodes.len()
    }

    /// Returns the `root_key` of this [`Tree`] instance.
    ///
    /// Returns [`None`] if this [`Tree`] instance is empty. Otherwise, returns
    /// [`Some(..)`] containing the `root_key`.
    pub fn root_key(&self) -> Option<K> {
        self.root_key
    }

    /// Returns a [`Node`] which corresponds to the given `key` inside of this
    /// [`Tree`] instance.
    ///
    /// If the given `key` does not exist in this [`Tree`] instance, then
    /// [`None`] is returned. Otherwise, returns [`Some(..)`] containing the
    /// [`Node`] entry.
    pub fn get(&self, key: K) -> Option<Node<'_, K, V>> {
        self.inner_nodes.get(key).map(|inner_node| Node {
            parent_key: inner_node.parent_key,
            child_keys: &inner_node.child_keys,
            value: &inner_node.value,
            depth: inner_node.depth,
        })
    }

    /// Returns a [`NodeMut`] which corresponds to the given `key` inside of
    /// this [`Tree`] instance.
    ///
    /// If the given `key` does not exist in this [`Tree`] instance, then
    /// [`None`] is returned. Otherwise, returns [`Some(..)`] containing the
    /// [`Node`] entry.
    pub fn get_mut(&mut self, key: K) -> Option<NodeMut<'_, K, V>> {
        self.inner_nodes.get_mut(key).map(|inner_node| NodeMut {
            parent_key: inner_node.parent_key,
            child_keys: &inner_node.child_keys,
            value: &mut inner_node.value,
            depth: inner_node.depth,
        })
    }

    /// Returns a [`Vec`] of all the descendent keys of the given `key`
    /// (not including the given `key` itself).
    ///
    /// If the given `key` does not exist in this [`Tree`] instance, then
    /// [`None`] is returned. Otherwise, returns [`Some(..)`] containing the
    /// descendent keys (including the given `key`).
    pub fn get_descendent_keys(&self, key: K, size_hint: Option<usize>) -> Option<Vec<K>> {
        self.inner_nodes.get(key).map(|node| {
            let size_hint = size_hint.unwrap_or_else(|| self.inner_nodes.len());

            let mut to_visit_keys = node.child_keys.iter().fold(
                Vec::with_capacity(size_hint),
                |mut vec, &child_key| {
                    vec.push(child_key);
                    vec
                },
            );
            let mut descendent_keys = Vec::with_capacity(size_hint);

            while let Some(to_visit_key) = to_visit_keys.pop() {
                descendent_keys.push(to_visit_key);
                let to_visit_child_keys = &self.inner_nodes.get(to_visit_key).unwrap().child_keys;
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
                let mut current_parent_key = tree.inner_nodes.get(key1).unwrap().parent_key;
                let length = tree.inner_nodes.len();
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
                            current_parent_key = tree.inner_nodes.get(parent_key).unwrap().parent_key;
                        }
                        None => break,
                    }
                }

                let mut current_parent_key = tree.inner_nodes.get(key2).unwrap().parent_key;

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
                                current_parent_key = tree.inner_nodes.get(parent_key).unwrap().parent_key;
                            }
                        }
                        None => unreachable!(),
                    }
                }
            }
        }

        let key1_exists = self.inner_nodes.contains_key(key1);
        let key2_exists = self.inner_nodes.contains_key(key2);
        let both_keys_exist = key1_exists && key2_exists;

        both_keys_exist.then(|| get_relationship(self, key1, key2))
    }

    // Iter methods:

    /// Returns an owned iterator over all the keys inside of this [`Tree`]
    /// instance.
    pub fn keys(&self) -> impl '_ + Iterator<Item = K> {
        self.inner_nodes.keys()
    }

    /// Returns an immutable iterator over all the [`Node`]s inside of this
    /// [`Tree`] instance.
    pub fn nodes(&self) -> impl Iterator<Item = Node<'_, K, V>> {
        self.inner_nodes.values().map(|inner_node| Node {
            parent_key: inner_node.parent_key,
            child_keys: &inner_node.child_keys,
            value: &inner_node.value,
            depth: inner_node.depth,
        })
    }

    /// Returns a mutable iterator over all the [`Node`]s inside of this
    /// [`Tree`] instance.
    pub fn nodes_mut(&mut self) -> impl Iterator<Item = NodeMut<'_, K, V>> {
        self.inner_nodes.values_mut().map(|inner_node| NodeMut {
            parent_key: inner_node.parent_key,
            child_keys: &inner_node.child_keys,
            value: &mut inner_node.value,
            depth: inner_node.depth,
        })
    }

    /// Create an immutable iterator over the key-value pairs inside of this
    /// [`Tree`] instance.
    ///
    /// The order of iteration is arbitrary. It will not be guaranteed to be
    /// depth-first, breadth-first, in-order, etc.
    pub fn iter(&self) -> impl Iterator<Item = (K, Node<'_, K, V>)> {
        self.inner_nodes.iter().map(|(key, inner_node)| {
            (
                key,
                Node {
                    parent_key: inner_node.parent_key,
                    child_keys: &inner_node.child_keys,
                    value: &inner_node.value,
                    depth: inner_node.depth,
                },
            )
        })
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
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (K, NodeMut<'_, K, V>)> {
        self.inner_nodes.iter_mut().map(|(key, inner_node)| {
            (
                key,
                NodeMut {
                    parent_key: inner_node.parent_key,
                    child_keys: &inner_node.child_keys,
                    value: &mut inner_node.value,
                    depth: inner_node.depth,
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
            inner_nodes: SlotMap::default(),
        }
    }
}

/// An internal container over the underlying value inside of this [`Tree`]
/// instance.
///
/// It contains the actual underlying value, as well as its `parent_key` and
/// `child_keys`.
struct InnerNode<K, V> {
    /// The parent key of this value.
    ///
    /// Is [`None`] iff this value is the root value.
    parent_key: Option<K>,

    /// The children keys of this value.
    child_keys: HashSet<K>,

    /// The actual underlying value that is stored.
    value: V,

    /// The depth that this [`InnerNode`] sits at.
    ///
    /// Here, depth starts at `0` which represents the root.
    depth: usize,
}

/// An immutable container over the underlying value inside of this [`Tree`]
/// instance as well as some other relevant information.
pub struct Node<'a, K, V> {
    /// The parent key of this value.
    ///
    /// Is [`None`] iff this value is the root value.
    pub parent_key: Option<K>,

    /// An immutable reference to the children keys of this value.
    pub child_keys: &'a HashSet<K>,

    /// An immutable reference to the underlying value that is stored.
    pub value: &'a V,

    /// The depth that this [`Node`] sits at.
    ///
    /// Here, depth starts at `0` which represents the root.
    pub depth: usize,
}

/// A mutable container over the underlying value inside of this [`Tree`]
/// instance as well as some other relevant information.
pub struct NodeMut<'a, K, V> {
    /// The parent key of this value.
    ///
    /// Is [`None`] iff this value is the root value.
    pub parent_key: Option<K>,

    /// An immutable reference to the children keys of this value.
    pub child_keys: &'a HashSet<K>,

    /// A mutable reference to the underlying value that is stored.
    pub value: &'a mut V,

    /// The depth that this [`NodeMut`] sits at.
    ///
    /// Here, depth starts at `0` which represents the root.
    pub depth: usize,
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
