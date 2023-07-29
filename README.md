# `pettree`
A small and simple tree implementation.

## Example:
```rust
use pettree::Tree;
use pettree::Node;
use pettree::NodeMut;
use slotmap::new_key_type;

new_key_type! { pub struct MyKey; }

fn main() {
    // First, we create an empty tree.
    // This tree is indexed by [`MyKey`]s and the values stored inside are [`usize`]s.
    let mut tree = Tree::<MyKey, usize>::default();

    // After creating this default, empty tree, we can do multiple things to it.
    // The below statements highlight just a few of the things that are possible:

    // 1. You can insert a root value into the tree.
    let root_key: MyMey = tree.insert_root(0);

    // 2. You can insert children values into the tree.
    let child_key_1: MyMey = tree.insert(root_key, 1);
    let child_key_2: MyMey = tree.insert(root_key, 2);
    let child_key_3: MyMey = tree.insert(root_key, 3);

    // 3. You can remove values from the tree.
    // (If the key does *not* exist in the tree, [`None`] is returned).
    let child_value: Option<usize> = tree.remove(child_key_1);

    // 4. You can get a [`Node`], which contains:
    // - an immutable reference to the underlying value
    // - the parent key ([`None`] if this value is the root value)
    // - the children keys (an empty iterator if this value has no children)
    let node: Option<Node> = tree.get(child_key_2);

    // 5. You can get a [`NodeMut`], which is the same as a [`Node`], but instead contains a *mutable* reference to the underlying data.
    let node_mut: Option<NodeMut> = tree.get_mut(child_key_3);
}
```
