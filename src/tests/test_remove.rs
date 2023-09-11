use crate::tests::utils::{
    make_decl_tree,
    make_tree_and_key_map,
    node,
};

#[test]
fn test_remove() {
    let tests = [
        // Test remove from empty
        ((None, 0), (None, None)),
        // Test remove root-node in a single root-tree
        ((Some(node! { 0 }), 0), (None, Some(0))),
        // Test remove non-existent node
        ((Some(node! { 0 }), 1), (Some(node! { 0 }), None)),
        // Test remove root-node in a multi-node-tree
        (
            (
                Some(node! {
                    0,
                    [
                        node! { 1 },
                        node! { 2 },
                        node! { 3 },
                    ],
                }),
                0,
            ),
            (None, Some(0)),
        ),
        // Test remove child-node in a multi-node-tree
        (
            (
                Some(node! {
                    0,
                    [
                        node! { 1 },
                        node! { 2 },
                        node! { 3 },
                    ],
                }),
                1,
            ),
            (
                Some(node! {
                    0,
                    [
                        node! { 2 },
                        node! { 3 },
                    ],
                }),
                Some(1),
            ),
        ),
        // Test remove child-node with its own children in a multi-node-tree
        (
            (
                Some(node! {
                    0,
                    [
                        node! { 1, [node!{ 10 }, node!{ 11 }] },
                        node! { 2, [node!{ 12 }, node!{ 13 }] },
                        node! { 3 },
                    ],
                }),
                1,
            ),
            (
                Some(node! {
                    0,
                    [
                        node! { 2, [node!{ 12 }, node!{ 13 }] },
                        node! { 3 },
                    ],
                }),
                Some(1),
            ),
        ),
    ];

    for ((decl_tree, key), (expected_decl_tree, expected_removed_value)) in tests {
        let (mut tree, key_map) = make_tree_and_key_map(decl_tree.as_ref());

        let key = key_map.get(&key).copied().unwrap_or_default();
        let actual_removed_value = tree.remove(key, None);
        let actual_decl_tree = make_decl_tree(&tree);

        assert_eq!(actual_decl_tree, expected_decl_tree);
        assert_eq!(actual_removed_value, expected_removed_value);
    }
}
