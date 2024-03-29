#![deny(missing_docs)]

//! A small, simple, and correct tree implementation.
//!
//! # Example:
//! ```
//! use tinytree::{
//!     Node,
//!     Tree,
//! };
//! use slotmap::DefaultKey;
//!
//! # fn main() {
//! // Create a default, empty tree:
//! let mut tree = Tree::<DefaultKey, usize>::default();
//!
//! // Insert a root value:
//! let root_key = tree.insert_root(0);
//!
//! // Insert some children values:
//! let child_key_1 = tree.insert(1, root_key).unwrap();
//! let child_key_2 = tree.insert(2, root_key).unwrap();
//! let child_key_3 = tree.insert(3, root_key).unwrap();
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

use std::mem::replace;

use indexmap::IndexSet;
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
#[derive(Clone)]
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
            child_keys: IndexSet::with_capacity(capacity),
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
    pub fn insert(&mut self, value: V, parent_key: K) -> Option<K> {
        self.insert_with_capacity(value, parent_key, 0)
    }

    /// Inserts a new child value into this [`Tree`] instance with a capacity
    /// specifying the number of children that this value will have.
    ///
    /// If this [`Tree`] instance does not contain the given `parent_key`, then
    /// [`None`] is returned. Otherwise, returns [`Some(..)`] containing the
    /// new key corresponding to this new child value.
    pub fn insert_with_capacity(&mut self, value: V, parent_key: K, capacity: usize) -> Option<K> {
        self.inner_nodes.contains_key(parent_key).then(|| {
            // # Note:
            // The reason why we `get(parent_key)` once and then do a `get_mut(parent_key)`
            // a second time is because of mutable borrowship issues.
            //
            // Potential source for optimization at a future point (although this would
            // likely require `unsafe`).

            let key = self.inner_nodes.insert(InnerNode {
                parent_key: Some(parent_key),
                child_keys: IndexSet::with_capacity(capacity),
                value,
            });

            self.inner_nodes
                .get_mut(parent_key)
                .unwrap()
                .child_keys
                .insert(key);

            key
        })
    }

    /// Reorder the children of the given `key` in this [`Tree`] instance.
    ///
    /// This function accepts a closure, `get_reordered_keys`, which passes in
    /// the current children of the given `key`. The closure is then
    /// expected to return a new [`IndexSet`] containing the original keys
    /// in the specified order that the caller would like.
    ///
    /// # Note:
    /// Callers must ensure that `get_reordered_keys` returns a [`IndexSet`]
    /// that is a *strict* subseteq of the current child keys. If
    /// `get_reordered_keys` returns an [`IndexSet`] that contains at least one
    /// key that was not contained in the original child keys, then this
    /// function will return `false`.
    ///
    /// However, returning an [`IndexSet`] that is *missing* a few keys from the
    /// original child keys is fine. This function will interpret that
    /// situation as the caller requesting to have those keys removed from
    /// this [`Tree`] instance.
    pub fn reorder_children<F>(&mut self, key: K, get_reordered_keys: F) -> bool
    where
        F: FnOnce(&IndexSet<K>) -> IndexSet<K>,
    {
        self.inner_nodes
            .get(key)
            .and_then(|inner_node| {
                let child_keys = &inner_node.child_keys;

                let reordered_keys = get_reordered_keys(&inner_node.child_keys);

                let difference = reordered_keys.difference(child_keys).next();

                match difference {
                    Some(..) => None,
                    None => {
                        let keys_to_remove = inner_node
                            .child_keys
                            .difference(&reordered_keys)
                            .copied()
                            .collect::<Vec<_>>();

                        Some((reordered_keys, keys_to_remove))
                    }
                }
            })
            .map(|(reordered_keys, mut keys_to_remove)| {
                let keys_to_remove_length = keys_to_remove.len();
                let tree_length = self.inner_nodes.len();

                // # Note:
                // Safe to perform `tree_length - keys_to_remove_length` because `tree_length >=
                // keys_to_remove_length`.
                keys_to_remove.reserve(tree_length - keys_to_remove_length);

                while let Some(key_to_remove) = keys_to_remove.pop() {
                    let inner_node = self.inner_nodes.remove(key_to_remove).unwrap();
                    keys_to_remove.extend(inner_node.child_keys);
                }

                self.inner_nodes.get_mut(key).unwrap().child_keys = reordered_keys;
            })
            .is_some()
    }

    /// Removes the value corresponding to the given `key` from this [`Tree`]
    /// instance as well as *all* of its children values.
    ///
    /// If this [`Tree`] instance does not contain the given `key`, then
    /// [`None`] is returned. Otherwise, returns [`Some(..)`] containing the
    /// removed value.
    ///
    /// The `size_hint` argument allows for one to specify the number of
    /// descendents the given `key` has. This can be helpful in order
    /// to allocate only the necessary amount of space and to avoid
    /// additional allocations + `memcpy`'s.
    ///
    /// If you do not have a hint, then provide [`None`] as the argument.
    pub fn remove(&mut self, key: K, size_hint: Option<usize>) -> Option<V> {
        fn remove_root<K, V>(tree: &mut Tree<K, V>, root_key: K) -> V
        where
            K: Key,
        {
            let value = tree.inner_nodes.remove(root_key).unwrap().value;
            tree.clear();
            value
        }

        fn remove_non_root<K, V>(
            tree: &mut Tree<K, V>,
            key: K,
            size_hint: Option<usize>,
        ) -> Option<V>
        where
            K: Key,
        {
            tree.inner_nodes.remove(key).map(|inner_node| {
                let size_hint = size_hint.unwrap_or_else(|| tree.inner_nodes.len());

                let mut to_visit_keys = Vec::with_capacity(size_hint);
                to_visit_keys.extend(inner_node.child_keys);

                while let Some(to_visit_key) = to_visit_keys.pop() {
                    let inner_node = tree.inner_nodes.remove(to_visit_key).unwrap();
                    to_visit_keys.extend(inner_node.child_keys);
                }

                let parent_key = inner_node.parent_key.unwrap();
                tree.inner_nodes
                    .get_mut(parent_key)
                    .unwrap()
                    .child_keys
                    .shift_remove(&key);

                inner_node.value
            })
        }

        self.root_key.and_then(|root_key| {
            if key == root_key {
                let root_value = remove_root(self, root_key);
                Some(root_value)
            }
            else {
                remove_non_root(self, key, size_hint)
            }
        })
    }

    /// Rebase the subtree rooted at `key` to be a child underneath the subtree
    /// rooted at `new_parent_key`.
    ///
    /// After performing this operation, the new parent of `key` will be
    /// `new_parent_key`.
    ///
    /// The `size_hint` argument allows for one to specify the number of
    /// descendents the given `key` has. This can be helpful in order
    /// to allocate only the necessary amount of space and to avoid
    /// additional allocations + `memcpy`'s.
    ///
    /// If you do not have a hint, then provide [`None`] as the argument.
    ///
    /// If `key` was not found in this [`Tree`] instance, then `false` is
    /// returned and no updates to the [`Tree`] are made. Otherwise,
    /// performs the requested rebase and returns `true`.
    pub fn rebase(&mut self, key: K, new_parent_key: K) -> bool {
        /// Performs a generic rebase of the given `key` onto the given
        /// `new_parent_key`.
        ///
        /// This rebasing algorithm is very generic and should be used during
        /// the "happy" paths. (I.e., when the `new_parent_key` is *not*
        /// a descendent of `key`).
        fn rebase_generic<K, V>(tree: &mut Tree<K, V>, key: K, new_parent_key: K)
        where
            K: Key,
        {
            let node = tree.inner_nodes.get_mut(key).unwrap();

            let current_parent_key = node.parent_key.unwrap();

            if current_parent_key != new_parent_key {
                node.parent_key = Some(new_parent_key);

                let current_parent_node = tree.inner_nodes.get_mut(current_parent_key).unwrap();
                current_parent_node.child_keys.shift_remove(&key);

                let new_parent_node = tree.inner_nodes.get_mut(new_parent_key).unwrap();
                new_parent_node.child_keys.insert(key);
            };
        }

        /// Performs a rebase where the `new_parent_key` is a decscendent of
        /// `key`.
        ///
        /// The reason or this function existing (as opposed to just using
        /// `rebase_generic`) is because rebasing onto one of your own
        /// descendents is much different than rebasing onto a non-descendent.
        ///
        /// For example, you need to worry about if the given `key` is the
        /// root-key and how to deal with that specific edge-case.
        fn rebase_onto_descendent<K, V>(tree: &mut Tree<K, V>, key: K, new_parent_key: K)
        where
            K: Key,
        {
            let inner_node = tree.inner_nodes.get_mut(key).unwrap();

            match inner_node.parent_key {
                Some(parent_key) => {
                    let beta_key = key;
                    let beta_parent_key = parent_key;
                    let alpha_key = new_parent_key;

                    inner_node.parent_key = Some(alpha_key);

                    let alpha_node = tree.inner_nodes.get_mut(alpha_key).unwrap();
                    let alpha_parent_key = alpha_node.parent_key.unwrap();

                    alpha_node.parent_key = Some(beta_parent_key);
                    alpha_node.child_keys.insert(beta_key);

                    let beta_parent_node = tree.inner_nodes.get_mut(beta_parent_key).unwrap();
                    beta_parent_node.child_keys.remove(&beta_key);
                    beta_parent_node.child_keys.insert(alpha_key);

                    tree.inner_nodes
                        .get_mut(alpha_parent_key)
                        .unwrap()
                        .child_keys
                        .remove(&alpha_key);
                }
                None => {
                    let beta_key = key;
                    let alpha_key = new_parent_key;

                    inner_node.parent_key = Some(alpha_key);

                    let alpha_node = tree.inner_nodes.get_mut(alpha_key).unwrap();
                    let alpha_parent_key = alpha_node.parent_key.unwrap();

                    alpha_node.parent_key = None;
                    alpha_node.child_keys.insert(beta_key);

                    tree.inner_nodes
                        .get_mut(alpha_parent_key)
                        .unwrap()
                        .child_keys
                        .remove(&alpha_key);

                    tree.root_key = Some(alpha_key);
                }
            }
        }

        /// Rebase `key` onto `parent_key` when their [`Relationship`] has been
        /// properly determined.
        fn rebase<K, V>(
            tree: &mut Tree<K, V>,
            relationship: Relationship<K>,
            key: K,
            new_parent_key: K,
        ) -> bool
        where
            K: Key,
        {
            match relationship {
                Relationship::Same => false,

                Relationship::Ancestral { ancestor_key, .. } if new_parent_key == ancestor_key => {
                    rebase_generic(tree, key, new_parent_key);
                    true
                }
                Relationship::Ancestral { descendent_key, .. }
                    if new_parent_key == descendent_key =>
                {
                    rebase_onto_descendent(tree, key, new_parent_key);
                    true
                }
                Relationship::Ancestral { .. } => unreachable!(),

                Relationship::Siblings { .. } => {
                    rebase_generic(tree, key, new_parent_key);
                    true
                }
            }
        }

        self.get_relationship(key, new_parent_key)
            .map_or(false, |relationship| {
                rebase(self, relationship, key, new_parent_key)
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

    /// Returns the `root_key` of this [`Tree`] instance, as well as its
    /// corresponding [`Node`], as a 2-tuple.
    ///
    /// Returns [`None`] if this [`Tree`] instance is empty. Otherwise, returns
    /// [`Some(..)`] containing appropriate values.
    pub fn root_key_value(&self) -> Option<(K, Node<'_, K, V>)> {
        self.root_key.map(|root_key| {
            let root_inner_node = self.inner_nodes.get(root_key).unwrap();
            let root_node = Node {
                parent_key: root_inner_node.parent_key,
                value: &root_inner_node.value,
                child_keys: &root_inner_node.child_keys,
            };

            (root_key, root_node)
        })
    }

    /// Returns the `root_key` of this [`Tree`] instance, as well as its
    /// corresponding [`NodeMut`], as a 2-tuple.
    ///
    /// Returns [`None`] if this [`Tree`] instance is empty. Otherwise, returns
    /// [`Some(..)`] containing appropriate values.
    pub fn root_key_value_mut(&mut self) -> Option<(K, NodeMut<'_, K, V>)> {
        self.root_key.map(|root_key| {
            let root_inner_node = self.inner_nodes.get_mut(root_key).unwrap();
            let root_node = NodeMut {
                parent_key: root_inner_node.parent_key,
                value: &mut root_inner_node.value,
                child_keys: &root_inner_node.child_keys,
            };

            (root_key, root_node)
        })
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
        })
    }

    /// Update the currently stored value at the given `key` with the
    /// `new_value` for this [`Tree`] instance.
    ///
    /// If the given `key` does not exist in this [`Tree`] instance, then
    /// [`None`] is returned. Otherwise, returns [`Some(..)`] containing the
    /// old value.
    pub fn set(&mut self, key: K, new_value: V) -> Option<V> {
        self.inner_nodes
            .get_mut(key)
            .map(|inner_node| replace(&mut inner_node.value, new_value))
    }

    /// Gets the [`Relationship`] status between two keys.
    ///
    /// If either `key_1` or `key_2` do not exist in this [`Tree`] instance,
    /// then [`None`] is returned. Otherwise, returns [`Some(..)`]
    /// containing the relationship between the two keys.
    pub fn get_relationship(&self, key_1: K, key_2: K) -> Option<Relationship<K>> {
        fn get_relationship<K, V>(tree: &Tree<K, V>, key_1: K, key_2: K) -> Relationship<K>
        where
            K: Key,
        {
            if key_1 == key_2 {
                Relationship::Same
            }
            else {
                let mut current_parent_key = tree.inner_nodes.get(key_1).unwrap().parent_key;
                let length = tree.inner_nodes.len();
                let mut path = IndexSet::with_capacity(length);

                loop {
                    match current_parent_key {
                        Some(parent_key) if parent_key == key_2 => {
                            return Relationship::Ancestral {
                                ancestor_key: key_2,
                                descendent_key: key_1,
                            }
                        }
                        Some(parent_key) => {
                            path.insert(parent_key);
                            current_parent_key =
                                tree.inner_nodes.get(parent_key).unwrap().parent_key;
                        }
                        None => break,
                    }
                }

                let mut current_parent_key = tree.inner_nodes.get(key_2).unwrap().parent_key;

                loop {
                    match current_parent_key {
                        Some(parent_key) if parent_key == key_1 => {
                            return Relationship::Ancestral {
                                ancestor_key: key_1,
                                descendent_key: key_2,
                            }
                        }
                        Some(parent_key) => {
                            if path.contains(&parent_key) {
                                return Relationship::Siblings {
                                    common_ancestor_key: parent_key,
                                };
                            }
                            else {
                                current_parent_key =
                                    tree.inner_nodes.get(parent_key).unwrap().parent_key;
                            }
                        }
                        None => unreachable!(),
                    }
                }
            }
        }

        let key_1_exists = self.inner_nodes.contains_key(key_1);
        let key_2_exists = self.inner_nodes.contains_key(key_2);
        let both_keys_exist = key_1_exists && key_2_exists;

        both_keys_exist.then(|| get_relationship(self, key_1, key_2))
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
        })
    }

    /// Returns a mutable iterator over all the [`Node`]s inside of this
    /// [`Tree`] instance.
    pub fn nodes_mut(&mut self) -> impl Iterator<Item = NodeMut<'_, K, V>> {
        self.inner_nodes.values_mut().map(|inner_node| NodeMut {
            parent_key: inner_node.parent_key,
            child_keys: &inner_node.child_keys,
            value: &mut inner_node.value,
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
#[derive(Clone)]
struct InnerNode<K, V> {
    /// The parent key of this value.
    ///
    /// Is [`None`] iff this value is the root value.
    parent_key: Option<K>,

    /// The children keys of this value.
    child_keys: IndexSet<K>,

    /// The actual underlying value that is stored.
    value: V,
}

/// An immutable container over the underlying value inside of this [`Tree`]
/// instance as well as some other relevant information.
#[derive(Clone, Copy)]
pub struct Node<'a, K, V> {
    /// The parent key of this value.
    ///
    /// Is [`None`] iff this value is the root value.
    pub parent_key: Option<K>,

    /// An immutable reference to the children keys of this value.
    pub child_keys: &'a IndexSet<K>,

    /// An immutable reference to the underlying value that is stored.
    pub value: &'a V,
}

/// A mutable container over the underlying value inside of this [`Tree`]
/// instance as well as some other relevant information.
pub struct NodeMut<'a, K, V> {
    /// The parent key of this value.
    ///
    /// Is [`None`] iff this value is the root value.
    pub parent_key: Option<K>,

    /// An immutable reference to the children keys of this value.
    pub child_keys: &'a IndexSet<K>,

    /// A mutable reference to the underlying value that is stored.
    pub value: &'a mut V,
}

/// A description of the relationship between two keys in a [`Tree`] instance.
///
/// Each variant of a [`Relationship`] is based off of familial relationships
/// (i.e., parents, grandparent, great-grandparents are all your ancestors).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
