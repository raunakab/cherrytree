use pettree::Tree;
use slotmap::DefaultKey;

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
