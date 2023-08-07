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
        small(),
        medium(),
        large(),
    ]
}

/// Returns all the below fixtures in a [`Vec`] *except* for the empty tree.
///
/// This can be useful for tests that want to test functionality on all
/// different types of *non-empty* trees.
pub fn all_non_empty() -> Vec<Tree<DefaultKey, usize>> {
    vec![
        single_root_tree(),
        depth_2_tree(),
        linear_depth_4_tree(),
        small(),
        medium(),
        large(),
    ]
}

/// Returns all the below fixtures in a [`Vec`] *except* for trees with depth of
/// less than 2.
///
/// This can be useful for tests that want to test functionality on all
/// different types of trees containing children.
pub fn all_depth_2_or_greater() -> Vec<Tree<DefaultKey, usize>> {
    vec![
        depth_2_tree(),
        linear_depth_4_tree(),
        small(),
        medium(),
        large(),
    ]
}

/// Returns all the below fixtures in a [`Vec`] *except* for trees with depth of
/// less than 3.
///
/// This can be useful for tests that want to test functionality on all
/// different types of trees containing children.
pub fn all_depth_3_or_greater() -> Vec<Tree<DefaultKey, usize>> {
    vec![linear_depth_4_tree(), medium(), large()]
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

/// Returns a uniform [`Tree`] with:
/// - depth of 2
/// - all parents having 2 children
/// - each node having a value of 0
///
/// Using the geometric sum formula, the number of nodes in this [`Tree`] will
/// be:
///
/// ```md
/// (2 ^ {2 + 1} - 1) / (2 - 1) = 7 / 1 = 7
/// ```
pub fn small() -> Tree<DefaultKey, usize> {
    uniform_tree(2, 2, 0)
}

/// Returns a uniform [`Tree`] with:
/// - depth of 3
/// - all parents having 3 children
/// - each node having a value of 0
///
/// Using the geometric sum formula, the number of nodes in this [`Tree`] will
/// be:
///
/// ```md
/// (3 ^ {3 + 1} - 1) / (3 - 1) = 40
/// ```
pub fn medium() -> Tree<DefaultKey, usize> {
    uniform_tree(3, 3, 0)
}

/// Returns a uniform [`Tree`] with:
/// - depth of 4
/// - all parents having 4 children
/// - each node having a value of 0
///
/// Using the geometric sum formula, the number of nodes in this [`Tree`] will
/// be:
///
/// ```md
/// (4 ^ {4 + 1} - 1) / (4 - 1) = 1023 / 3 = 341
/// ```
pub fn large() -> Tree<DefaultKey, usize> {
    uniform_tree(4, 4, 0)
}

/// Generates a new, uniform [`Tree`].
///
/// A uniform [`Tree`] is a [`Tree`] in which:
/// - all the children nodes are at the *same* level
/// - all the parent nodes have the exact same number of children nodes
///
/// # Number of Nodes:
/// Calculating the number of nodes in this [`Tree`] requires a little bit of
/// math.
///
/// Firstly, some definitions:
/// - let `S` represent the number of nodes in this [`Tree`]
/// - let `b` represent the number of children each parent has
/// - let `N` represent the depth of this [`Tree`]
///
/// Since this [`Tree`] is uniform, we can use the geometric sum for `S` to
/// calculate the number of nodes contained. Namely:
///
/// ```md
/// S = b^0 + b^1 + b^2 + ... + b^N
///   = (b^{N + 1} - 1) / (b - 1)
/// ```
pub fn uniform_tree(
    depth: usize,
    number_of_children: usize,
    default_value: usize,
) -> Tree<DefaultKey, usize> {
    fn add_children(
        tree: &mut Tree<DefaultKey, usize>,
        key: DefaultKey,
        current_depth: usize,
        depth: usize,
        number_of_children: usize,
        default_value: usize,
    ) {
        if current_depth < depth {
            (0..number_of_children).for_each(|_| {
                let key = tree.insert(key, default_value).unwrap();
                add_children(
                    tree,
                    key,
                    current_depth + 1,
                    depth,
                    number_of_children,
                    default_value,
                );
            });
        }
    }

    let mut tree = Tree::default();

    let root_key = tree.insert_root_with_capacity(default_value, number_of_children);
    add_children(
        &mut tree,
        root_key,
        0,
        depth,
        number_of_children,
        default_value,
    );

    tree
}
