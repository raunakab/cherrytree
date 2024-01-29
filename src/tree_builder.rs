use slotmap::Key;

use crate::Tree;

/// A "builder" pattern implementation for a [`Tree`].
///
/// # Problem:
/// Normally, if you would want to build a [`Tree`] instance, you *need* access
/// to the [`Tree`] instance. I.e.:
///
/// ```no_run
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
/// // Notice how this function does *not* need any additional parameters!
/// fn add_children() -> TreeBuilder<u8> {
///     let mut tree_builder = TreeBuilder::new(2);
///     let index = tree_builder.push(3, None);
///     let _ = tree_builder.push(4, Some(index));
///
///     tree_builder
/// }
///
/// let mut tree_builder = TreeBuilder::<u8>::new(0);
/// let other_tree_builder = add_children();
///
/// tree_builder.extend(other_tree_builder);
///
/// // Finally, turn it into a [`Tree`] instance:
/// let tree = tree_builder.finish();
/// ```
pub struct TreeBuilder<V> {
    root_value: V,
    hooks: Vec<(V, Option<usize>)>,
}

impl<V> TreeBuilder<V> {
    /// Construct a new [`TreeBuilder<V>`] with some given `root_value`.
    pub fn new(root_value: V) -> Self {
        Self {
            root_value,
            hooks: vec![],
        }
    }

    /// Push a new "hook" into this [`TreeBuilder<V>`].
    ///
    /// Returns a [`usize`]. Think of this as a "unique" key which identifies
    /// the newly inserted value. The `parent_index` is the index of the
    /// parent-value for which you want this given value to be a child of.
    pub fn push(&mut self, value: V, parent_index: Option<usize>) -> usize {
        let length = self.hooks.len();

        let is_valid = match (parent_index, length) {
            (Some(_), 0) => false,
            (Some(parent_index), _) if parent_index <= length - 1 => true,
            (Some(_), _) => false,
            (None, _) => true,
        };

        if is_valid {
            self.hooks.push((value, parent_index));
            length - 1
        } else {
            panic!()
        }
    }

    /// Extend the current [`TreeBuilder`] instance with the *entire* contents
    /// of another [`TreeBuilder`] instance.
    ///
    /// The `parent_index` is the index of the parent-value for which you want
    /// this given value to be a child of.
    pub fn extend(&mut self, mut other: Self, parent_index: Option<usize>) {
        let number_of_incoming_hooks = other.hooks.len() + 1;
        self.hooks.reserve(number_of_incoming_hooks);

        let length = self.hooks.len();

        let update_indices = |(value, parent_index): (V, Option<usize>)| {
            let parent_index = parent_index.map_or(length, |parent_index| parent_index + length);
            (value, Some(parent_index))
        };

        let is_valid = match (parent_index, length) {
            (Some(_), 0) => false,
            (Some(parent_index), _) if parent_index <= length - 1 => true,
            (Some(_), _) => false,
            (None, _) => true,
        };

        if is_valid {
            self.hooks.push((other.root_value, parent_index));
            let other_hooks_iter = other.hooks.drain(..).map(update_indices);
            self.hooks.extend(other_hooks_iter);
        } else {
            panic!()
        }
    }

    /// Finish building the structure of the tree and turn it into an actual
    /// instance of a [`Tree`]!
    pub fn finish<K>(self) -> Tree<K, V>
    where
        K: Key,
    {
        let length = self.hooks.len() + 1;
        let mut tree = Tree::with_capacity(length);

        let root_key = tree.insert_root(self.root_value);

        let mut keys = Vec::with_capacity(length - 1);

        for (value, index) in self.hooks {
            let parent_key = match index {
                Some(index) => *keys.get(index).unwrap(),
                None => root_key,
            };

            let key = tree.insert(value, parent_key).unwrap();
            keys.push(key);
        }

        tree
    }
}
