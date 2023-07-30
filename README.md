# `pettree`
A small, simple, and correct tree implementation.

## Example:
The following program will show you just some of the ways that you can interact with a [`Tree`], the data-structure responsible for providing an arbitrary-arity-tree implementation.

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
    let root_key: MyKey = tree.insert_root(0);

    // 2. You can insert children values into the tree.
    let child_key_1: MyKey = tree.insert(root_key, 1).unwrap();
    let child_key_2: MyKey = tree.insert(root_key, 2).unwrap();
    let child_key_3: MyKey = tree.insert(root_key, 3).unwrap();

    // 3. You can remove values from the tree.
    // (If the key does *not* exist in the tree, [`None`] is returned).
    // The second argument is a size-hint which can help provide an indication on how many children this value has.
    // You can pass in [`None`] if you do not know.
    let child_value_1: usize = tree.remove(child_key_1, None).unwrap();
    assert_eq!(child_value_1, 1);

    // 4. You can get a [`Node`], which contains:
    // - an immutable reference to the underlying value
    // - the parent key ([`None`] if this value is the root value)
    // - the children keys (an empty iterator if this value has no children)
    let node: Node<MyKey, usize> = tree.get(child_key_2).unwrap();
    assert_eq!(node.parent_key, Some(root_key));
    assert_eq!(*node.value, 2);
    assert_eq!(node.child_keys.into_iter().collect::<Vec<_>>().len(), 0);

    // 5. You can get a [`NodeMut`], which is the same as a [`Node`], but instead contains a *mutable* reference to the underlying data.
    let node_mut: NodeMut<MyKey, usize> = tree.get_mut(child_key_3).unwrap();
    assert_eq!(node_mut.parent_key, Some(root_key));
    assert_eq!(*node_mut.value, 3);
    assert_eq!(node_mut.child_keys.into_iter().collect::<Vec<_>>().len(), 0);
}
```

More examples of programs can be found [in the examples directory](./examples).
Each example file is prefixed with a number and followed by a short explanation on what that example showcases (e.g., `1_this_example_does_xyz.rs`, `2_this_example_does_abc.rs`).
It is recommended to read through the examples in numerical order.
