//! This example showcases how to create a default, empty [`Tree`] instance.
//!
//! The key that is being used to index into this [`Tree`] is the [`DefaultKey`]
//! type that is provided by the [`slotmap`] crate. If you want to be able to
//! use your own custom keys, consider using the [`slotmap::new_key_type`]
//! macro.

use cherrytree::Tree;
use slotmap::DefaultKey;

fn main() {
    let tree = Tree::<DefaultKey, usize>::default();
    assert!(tree.is_empty());
}
