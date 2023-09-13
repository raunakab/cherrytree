#![allow(missing_docs)]

#[cfg(test)]
mod test_get_relationship;

#[cfg(test)]
mod test_insert;

#[cfg(test)]
mod test_insert_root;

#[cfg(test)]
mod test_rebase;

#[cfg(test)]
mod test_remove;

#[cfg(test)]
mod test_reorder_children;

#[cfg(any(test, feature = "decl_tree"))]
pub mod utils;
