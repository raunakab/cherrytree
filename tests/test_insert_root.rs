#[path = "../common/mod.rs"]
mod common;

use common::{
    make_deserial_node,
    make_tree_and_key_map,
    node,
    DeserialNode,
};

#[test]
fn test_insert_root() {
    let tests = [
        ((None, 0), Some(node! { 0 })),
        ((Some(node! { 1 }), 0), Some(node! { 0 })),
        (
            (
                Some(node! { 1, [node! { 2, [node! { 4 }] }, node! { 3 }] }),
                0,
            ),
            Some(node! { 0 }),
        ),
    ];

    for ((deserial_node, value_to_insert), expected_deserial_node) in tests {
        let (mut tree, _) = make_tree_and_key_map(deserial_node.as_ref());

        let _ = tree.insert_root(value_to_insert);

        let actual_deserial_node = make_deserial_node(&tree);
        assert_eq!(actual_deserial_node, expected_deserial_node);
    }
}
