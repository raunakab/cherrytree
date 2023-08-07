use pettree::Tree;
use slotmap::DefaultKey;

/// Gets some leaf key in this [`Tree`].
///
/// A "leaf key" is a key which has no children of its own.
///
/// # Note:
/// This function pulls the first leaf key in the [`hashbrown::HashSet`] that it
/// finds. Since [`hashbrown::HashSet`]'s do *NOT* guarantee order, this
/// function will not necessarily return the same key for two separate
/// [`hashbrown::HashSet`] instances with the same exact data structure.
pub fn get_some_leaf_key(tree: &Tree<DefaultKey, usize>) -> DefaultKey {
    let mut key = tree.root_key().unwrap();

    loop {
        let selected_child_key = tree.get(key).unwrap().child_keys.iter().next();
        match selected_child_key {
            Some(&selected_child_key) => key = selected_child_key,
            None => break key,
        }
    }
}

/// Gets the parent of some leaf key key in this [`Tree`].
///
/// # Note:
/// This function pulls the first parent of some leaf key in the
/// [`hashbrown::HashSet`] that it finds. Since [`hashbrown::HashSet`]'s do
/// *NOT* guarantee order, this function will not necessarily return the same
/// key for two separate [`hashbrown::HashSet`] instances with the same exact
/// data structure.
pub fn get_parent_of_some_leaf_key(tree: &Tree<DefaultKey, usize>) -> DefaultKey {
    let mut key = tree.root_key().unwrap();
    let mut parent_key = None;

    loop {
        let selected_child_key = tree.get(key).unwrap().child_keys.iter().next();
        match selected_child_key {
            Some(&selected_child_key) => {
                parent_key = Some(key);
                key = selected_child_key
            }
            None => break parent_key.unwrap(),
        }
    }
}
