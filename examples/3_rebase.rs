//! This example showcases how to rebase a subtree inside of our [`Tree`] onto
//! another subtree inside of our [`Tree`].
//!
//! Namely, consider the following tree:
//!
//! ```md
//! 0
//! |-- 1
//!     |-- 3
//!     |-- 4
//!         |-- 5
//! |-- 2
//!     |-- 6
//!     |-- 7
//! ```
//!
//! Rebasing `4` underneath `2` would result in:
//!
//! ```md
//! 0
//! |-- 1
//!     |-- 3
//! |-- 2
//!     |-- 6
//!     |-- 7
//!     |-- 4
//!         |-- 5
//! ```
//!
//! Notice how the *entire* subtree of `4` gets shifted underneath `2`. The `4`
//! subtree is now siblings with `6` and `7`.

use pettree::Tree;
use slotmap::DefaultKey;

fn main() {
    let mut tree = Tree::<DefaultKey, usize>::default();

    let root_key = tree.insert_root(0);

    let child_key_1 = tree.insert(1, root_key).unwrap();
    let child_key_2 = tree.insert(2, root_key).unwrap();

    tree.insert(3, child_key_1).unwrap();

    let child_key_4 = tree.insert(4, child_key_1).unwrap();
    tree.insert(5, child_key_4).unwrap();
    tree.insert(6, child_key_2).unwrap();
    tree.insert(7, child_key_2).unwrap();

    assert!(tree.rebase(child_key_4, child_key_2, None));
}
