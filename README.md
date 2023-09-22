<div align="center">
    <h1>cherrytree 🍒🌳</h1>
    <p><strong>A small, simple, and correct tree implementation.</strong></p>
</div>

<br>

## Overview

`cherrytree` is a library which provies a safe, simple, and correct API over a tree data structure, "`Tree`".

`Tree` has been specifically designed in such a way that *each* node inside of it can be queried with a unique key in *constant time*.
This proves highly suitable for applications where data needs to be grouped in a tree-like fashion while also being able to be retrieved as quickly as possible.
Certain popular applications include DOMs and non-cyclical solvers.

<br>

## Features

#### Immutable APIs:

- Constant time access to an immutable reference to a `Node`s value, its child keys, and its optional parent key given the unique key that identifies it.

- Getting the relationship between two `Node`s in a `Tree` instance. Examples of relationships include "ancestor-descendent" and "siblings".

- Immutable iterator over the `Node`s of the `Tree` in some arbitrary order. BFS and DFS immutable iterators are planning on being supported in a future release.

#### Mutable APIs:

- Constant time access to a mutable reference to a `Node`s value. This API will **not**, however, provide mutable references to its child keys and optional parent key in order to preserve the integrity of the `Tree` instance.

- Removal of a `Node` from the `Tree`. This will subsequently remove all of that `Node`s descendent `Node`s.

- Rebasing a `Node` onto another `Node` in the `Tree`. Even "unusual" rebase operations (e.g., rebasing a parent onto its descendent) are supported! More information about rebasing can be found in the docs. The [examples directory](./examples) also contains an example on how to do this.

- Reordering the child keys of a `Node`. Child keys are inserted into a `Node` in insertion-order (i.e., first come, first in). As such, the child keys can be reordered to match some new desired order.

- Mutable iterator over the `Node`s of the `Tree` in some arbitrary order. BFS and DFS mutable iterators are planning on being supported in a future release.

<br>

## Usage

Add this to your `Cargo.toml` manifest file, substituting the `VERSION_NUMBER` variable for any real version:

```toml
[dependencies]
cherrytree = "${VERSION_NUMBER}"
```

<br>

## Examples

### Basic
Let's take a look at how to interact with a `Tree` instance.

First we create a tree which stores `usize`s and uses `DefaultKey`s to index into it.
```rust
use cherrytree::{
    Node,
    Tree,
};
use slotmap::DefaultKey;

let mut tree = Tree::<DefaultKey, usize>::default();
```

We then insert a root `Node`.

```rust
let root_key = tree.insert_root(0);
```

And then insert some children `Node`s under the root `Node`.
The way we do this is by calling the `insert` method and pass the `root_key` we created earlier to notify the tree to use that node as the parent.

```rust
let child_key_1 = tree.insert(1, root_key).unwrap();
let child_key_2 = tree.insert(2, root_key).unwrap();
let child_key_3 = tree.insert(3, root_key).unwrap();
```

We can then get the first child `Node` (in constant time) that we inserted and pull its value out.
This value will be an immutable reference, so we will **not** be able to mutate it.

```rust
let child_node_1 = tree.get(child_key_1).unwrap();
assert_eq!(*child_node_1.value, 1);
```

We can also get the second child `Node` (in constant time) that we inserted and pull its value out.
This time, the value will be a mutable reference, so we **can** mutate it!

```rust
let child_node_2 = tree.get_mut(child_key_2).unwrap();
assert_eq!(*child_node_2.value, 2);
*child_node_2.value = 100;

// assert that the value has indeed been updated
let child_node_2 = tree.get(child_key_2).unwrap();
assert_eq!(*child_node_2.value, 100);
```

<br>

### More Examples

Further examples of programs can be found in the [examples directory](./examples).

Each example file is prefixed with a number and followed by a short explanation on what that example showcases (e.g., `1_this_example_does_xyz.rs`, `2_this_example_does_abc.rs`).
It is recommended to read through the examples in numerical order.

<br>

## Implementation

Internaly, a `Tree` just contains a map of `Node`s, and each `Node` contains the value it was given as well as the keys to other `Node`s in the map.
This way, given some key, a `Node` can be queried for in constant time.

<br>

## License

This software is licensed under the [BSD 3-Clause License](./LICENSE-BSD-3-CLAUSE).
The license's source can be found [here](https://opensource.org/license/bsd-3-clause/).
