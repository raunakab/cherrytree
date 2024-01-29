use slotmap::Key;

use crate::Tree;

/// A "builder" pattern implementation for a [`Tree`].
///
/// # Problem:
/// Normally, if you would want to build a [`Tree`] instance, you *need* access
/// to the [`Tree`] instance. I.e.:
///
/// ```no_run
/// use cherrytree::Tree;
/// use slotmap::DefaultKey;
///
/// let mut tree = Tree::<DefaultKey, u8>::default();
/// let root_key = tree.insert_root(0);
///
/// // *Need* the `tree` variable in order to insert into it.
/// let _ = tree.insert(1, root_key).unwrap();
/// ```
///
/// Now, this can become cumbersome when trying to execute functions that want
/// to add onto this tree, because you then need to pass in the `tree` variable,
/// as well as the parent key! For example:
///
/// ```no_run
/// use cherrytree::Tree;
/// use slotmap::DefaultKey;
///
/// // This function *needs* to have `tree` and `parent_key` as parameters.
/// // Otherwise, it can't add to the tree!
/// fn add_children(tree: &mut Tree<DefaultKey, u8>, parent_key: DefaultKey) {
///     let key = tree.insert(2, parent_key).unwrap();
///     let key = tree.insert(3, key).unwrap();
///     let _ = tree.insert(4, key).unwrap();
/// }
///
/// let mut tree = Tree::<DefaultKey, u8>::default();
/// let root_key = tree.insert_root(0);
///
/// let key = tree.insert(1, root_key).unwrap();
///
/// add_children(&mut tree, key);
/// ```
///
/// You could set the [`Tree`] instance as a global variable, but that
/// introduces 2 new problems:
/// 1. Global mutable data! Bad!
/// 2. You need some sort of global mutable stack data structure to track
///    different `parent_key`s as you recurse up and down the tree. More global
///    mutable data! Very bad!
///
/// Therefore, in order to make "tree-building" more ergonomic (for the
/// end-consumer) and more efficient (for the computer), the [`TreeBuilder`]
/// struct was built. It allows for you to "store" all the parent-children
/// relationships, return them from a function, and then build the actual
/// [`Tree`] up *after* the function has returned. E.g.:
///
/// ```no_run
/// use cherrytree::{tree_builder::TreeBuilder, Tree};
/// use slotmap::DefaultKey;
///
/// // Notice how this function does *not* need any additional parameters!
/// fn add_children() -> TreeBuilder<u8> {
///     let mut tree_builder = TreeBuilder::default();
///
///     let root_index = tree_builder.push_root(2);
///     let index = tree_builder.push(3, root_index);
///     let _ = tree_builder.push(4, index);
///
///     tree_builder
/// }
///
/// let mut tree_builder = TreeBuilder::<u8>::default();
/// let other_tree_builder = add_children();
///
/// let root_index = tree_builder.push_root(0);
/// tree_builder.extend(other_tree_builder, root_index);
///
/// // Finally, turn it into a [`Tree`] instance:
/// let tree = tree_builder.finish::<DefaultKey>();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
// pub struct TreeBuilder<V>(Vec<(V, Option<usize>)>);
pub struct TreeBuilder<V>(Option<(V, Vec<(V, usize)>)>);

impl<V> Default for TreeBuilder<V> {
    fn default() -> Self {
        Self(None)
    }
}

impl<V> TreeBuilder<V> {
    /// Push a new root "hook" into this [`TreeBuilder`] instance.
    ///
    /// Returns a [`usize`]. Think of this as a "unique" key which identifies
    /// the newly inserted value.
    ///
    /// # Panics:
    /// This function will panic if [`Self::push_root`] has already been called.
    pub fn push_root(&mut self, root_value: V) -> usize {
        match &mut self.0 {
            Some(..) => panic!(),
            None => {
                self.0 = Some((root_value, vec![]));
                0
            }
        }
    }

    /// Push a new child "hook" into this [`TreeBuilder`] instance.
    ///
    /// Returns a [`usize`]. Think of this as a "unique" key which identifies
    /// the newly inserted value.
    ///
    /// The `parent_index` is the index of the parent-value for which you want
    /// this given value to be a child of.
    ///
    /// # Panics:
    /// This function will panic if [`Self::push_root`] is not called first or if the given `parent_index` is out of bounds.
    pub fn push(&mut self, value: V, parent_index: usize) -> usize {
        let (_, hooks) = self.0.as_mut().unwrap();
        let length = hooks.len();
        let augmented_length = length + 1;

        if parent_index <= augmented_length - 1 {
            hooks.push((value, parent_index));
            augmented_length
        } else {
            panic!()
        }
    }

    /// Extend the current [`TreeBuilder`] instance with the *entire* contents
    /// of another [`TreeBuilder`] instance.
    ///
    /// The `parent_index` is the index of the parent-value for which you want
    /// this given value to be a child of.
    pub fn extend(&mut self, other: Self, parent_index: usize) {
        match (&mut self.0, other.0) {
            (Some((_, hooks)), Some((other_root_value, mut other_hooks))) => {
                let length = hooks.len();
                let augmented_length = length + 1;

                if parent_index <= augmented_length - 1 {
                    let other_length = other_hooks.len();
                    let other_augmented_length = other_length + 1;

                    hooks.reserve_exact(other_augmented_length);

                    hooks.push((other_root_value, parent_index));

                    let other_iter = other_hooks
                        .drain(..)
                        .map(|(value, parent_index)| (value, augmented_length + parent_index));
                    hooks.extend(other_iter);
                } else {
                    panic!()
                }
            }
            (Some(..), None) => (),
            (None, Some((other_root_value, other_hooks))) => {
                self.0 = Some((other_root_value, other_hooks))
            }
            (None, None) => (),
        }
    }

    /// Finish building the structure of the tree and turn it into an actual
    /// instance of a [`Tree`]!
    pub fn finish<K>(self) -> Tree<K, V>
    where
        K: Key,
    {
        match self.0 {
            Some((root_value, hooks)) => {
                let length = hooks.len();
                let augmented_length = length + 1;

                let mut tree = Tree::with_capacity(augmented_length);
                let mut keys = Vec::with_capacity(augmented_length);

                let root_key = tree.insert_root(root_value);
                keys.push(root_key);

                for (value, parent_index) in hooks {
                    let &parent_key = keys.get(parent_index).unwrap();
                    let key = tree.insert(value, parent_key).unwrap();
                    keys.push(key);
                }

                tree
            }
            None => Tree::default(),
        }
    }
}
