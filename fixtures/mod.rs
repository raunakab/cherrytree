#![allow(unused)]

//! Test fixtures that can be called in tests to provide easy, already
//! configured [`Tree`]s of specific shapes.

use pettree::Tree;
use slotmap::DefaultKey;

/// Returns all the below fixtures in a [`Vec`].
pub fn all() -> Vec<Tree<DefaultKey, usize>> {
    vec![
        empty_tree(),
        single_root_tree(),
        depth_2_tree(),
        linear_depth_4_tree(),
    ]
}

/// Returns all the below fixtures in a [`Vec`] *except* for the empty tree.
///
/// This can be useful for tests that want to test functionality on all
/// different types of *non-empty* trees.
pub fn all_non_empty() -> Vec<Tree<DefaultKey, usize>> {
    vec![single_root_tree(), depth_2_tree(), linear_depth_4_tree()]
}

/// Returns an empty [`Tree`] with no elements in it (not even a root
/// [`pettree::Node`]).
pub fn empty_tree() -> Tree<DefaultKey, usize> {
    Tree::default()
}

/// Returns a [`Tree`] with only a single root [`pettree::Node`] in it.
pub fn single_root_tree() -> Tree<DefaultKey, usize> {
    let mut tree = Tree::with_capacity(1);

    tree.insert_root(0);

    tree
}

/// Returns a [`Tree`] with only a root [`pettree::Node`] and 3 children
/// [`pettree::Node`]s.
///
/// # Shape:
/// ```md
/// 0
/// |-- 1
/// |-- 2
/// |-- 3
/// ```
///
/// The depth of this tree is only 2. Namely, the children of the root
/// node do *NOT* have any children of their own.
pub fn depth_2_tree() -> Tree<DefaultKey, usize> {
    let mut tree = Tree::with_capacity(1);

    let root_key = tree.insert_root(0);

    tree.insert(root_key, 1).unwrap();
    tree.insert(root_key, 2).unwrap();
    tree.insert(root_key, 3).unwrap();

    tree
}

/// Returns a [`Tree`] with 4 [`pettree::Node`]'s in it, all structured in a
/// "linked-list" format.
///
/// # Shape:
/// ```md
/// 0
/// |-- 1
///     |-- 2
///         |-- 3
/// ```
///
/// Notice how the is shaped such a way that no node is a
/// sibling of another node. There exists strictly an ancestral
/// relationship between all nodes in this tree.
pub fn linear_depth_4_tree() -> Tree<DefaultKey, usize> {
    let mut tree = Tree::with_capacity(1);

    let root_key = tree.insert_root(0);

    let child_1_key = tree.insert(root_key, 1).unwrap();
    let child_2_key = tree.insert(child_1_key, 2).unwrap();
    tree.insert(child_2_key, 3).unwrap();

    tree
}
