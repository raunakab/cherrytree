# `tinytree`
A small, simple, and correct tree implementation.

## Overview:
`tinytree` is a library which provies a safe, simple, and correct API over a tree data structure, "`Tree`".

`Tree` has been specifically designed in such a way that *each* node inside of it can be queried with a unique key in *constant time*.
This proves highly suitable for applications where data needs to be grouped in a tree-like fashion while also being able to be retrieved as quickly as possible.
Certain popular applications include DOMs and non-cyclical solvers.

## Features:
`Tree` provides:
- basic immutable APIs such as:
    - getting the value of a `Node` in constant time
    - getting the relationship between two `Node`s in the `Tree`
    - immutably iterating over `Node`s

- more advanced mutable APIs such as:
    - removing `Node`s (which then removes its entire subtree)
    - arbitrarily reordering children `Node`s
    - rebasing `Node`s onto other `Node`s (regardless of their relationship)
        - i.e., can even rebase a parent `Node` onto one of its descendent `Node`s
    - mutably iterating over `Node`s

## Example:

```rust
use tinytree::{
    Node,
    Tree,
};
use slotmap::DefaultKey;

fn main() {
    // Create a default, empty tree:
    let mut tree = Tree::<DefaultKey, usize>::default();

    // Insert a root value:
    let root_key = tree.insert_root(0);

    // Insert some children values:
    let child_key_1 = tree.insert(1, root_key).unwrap();
    let child_key_2 = tree.insert(2, root_key).unwrap();
    let child_key_3 = tree.insert(3, root_key).unwrap();

    // Get an immutable reference to one of the children's value:
    let child_node_1 = tree.get(child_key_1).unwrap();
    assert_eq!(*child_node_1.value, 1);

    // Or get a mutable reference to one of the children's value:
    let child_node_2 = tree.get_mut(child_key_2).unwrap();
    *child_node_2.value = 100;

    // And assert that that value has been updated:
    let child_node_2 = tree.get(child_key_2).unwrap();
    assert_eq!(*child_node_2.value, 100);
}
```

Further examples of programs can be found in the [examples directory](./examples).
Each example file is prefixed with a number and followed by a short explanation on what that example showcases (e.g., `1_this_example_does_xyz.rs`, `2_this_example_does_abc.rs`).
It is recommended to read through the examples in numerical order.

## Usage
Add this to your `Cargo.toml` manifest file:

```toml
[dependencies]
rand = "0.8.5"
```

## Theory:
Formally, a [tree](https://en.wikipedia.org/wiki/Tree_(data_structure)) is
just a [graph](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)).
However, it's not just any type of graph; a tree is a special type of graph
that enforces certain invariants on the structure of its nodes and edges.

Namely, the following invariants must be held:
- Each node (except for one) must have exactly 1 other node pointing to it. This other node pointing to it is referred to as the parent node.
- The remaining node must have 0 other nodes pointing to it. This is the root node.

If any arbitrary graph meets the above requirements, then it can also be
classified as a tree.

## Implementation:
Internaly, a `Tree` just contains a map of `Node`s, and each `Node` contains the value it was given as well as the keys to other `Node`s in the map.
This way, given some key, a `Node` can be queried for in constant time.

## License
This software is licensed under the [BSD 3-Clause License](./LICENSE-BSD-3-CLAUSE).
The license's source can be found [here](https://opensource.org/license/bsd-3-clause/).
