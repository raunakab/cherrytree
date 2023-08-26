#[path = "../common/mod.rs"]
mod common;

use common::{
    make_deserial_node,
    make_tree_and_key_map,
    node,
    DeserialNode,
};

#[test]
fn test_reorder_children() {
    let tests = [
        // Test reordering children on empty tree
        ((None, 0, vec![1, 2, 3]), (None, false)),
        // Test reordering children with non-existent parent-key
        (
            (Some(node! { 0 }), 8, vec![1, 2, 3]),
            (Some(node! { 0 }), false),
        ),
        // Test reordering children on node with no children
        (
            (Some(node! { 0 }), 0, vec![1, 2, 3]),
            (Some(node! { 0 }), false),
        ),
        // Test reordering children on node with keys that are not its own children keys
        (
            (
                Some(node! { 0, [
                    node! { 1, [node! { 2 }, node! { 3 }, node! { 4 }]},
                    node! { 5 },
                ] }),
                5,
                vec![2, 3, 4],
            ),
            (
                Some(node! { 0, [
                    node! { 1, [node! { 2 }, node! { 3 }, node! { 4 }]},
                    node! { 5 },
                ] }),
                false,
            ),
        ),
        // Test basic reordering children
        (
            (
                Some(node! { 0, [
                    node! { 1, [node! { 2, [node! { 6 }] }, node! { 3 }, node! { 4 }]},
                    node! { 5 },
                ] }),
                1,
                vec![3, 2, 4],
            ),
            (
                Some(node! { 0, [
                    node! { 1, [node! { 3 }, node! { 2, [node! { 6 }] }, node! { 4 }]},
                    node! { 5 },
                ] }),
                true,
            ),
        ),
        // Test basic reordering children and removing key
        (
            (
                Some(node! { 0, [
                    node! { 1, [node! { 2, [node! { 6 }] }, node! { 3 }, node! { 4 }]},
                    node! { 5 },
                ] }),
                1,
                vec![4, 3],
            ),
            (
                Some(node! { 0, [
                    node! { 1, [node! { 4 }, node! { 3 }]},
                    node! { 5 },
                ] }),
                true,
            ),
        ),
        // Test basic reordering children and removing key
        (
            (
                Some(node! { 0, [
                    node! { 1, [node! { 2, [node! { 6 }] }, node! { 3, [node! { 7 }] }, node! { 4 }]},
                    node! { 5 },
                ] }),
                1,
                vec![4, 3],
            ),
            (
                Some(node! { 0, [
                    node! { 1, [node! { 4 }, node! { 3, [node! { 7 }] }]},
                    node! { 5 },
                ] }),
                true,
            ),
        ),
    ];

    for (
        (deserial_node, key, reordered_child_keys),
        (expected_deserial_node, expected_did_reorder),
    ) in tests
    {
        let (mut tree, key_map) = make_tree_and_key_map(deserial_node.as_ref());

        let key = key_map.get(&key).copied().unwrap_or_default();
        let actual_did_reorder = tree.reorder_children(key, |_| {
            reordered_child_keys
                .into_iter()
                .map(|child_key| key_map.get(&child_key).copied().unwrap_or_default())
                .collect()
        });
        let actual_deserial_node = make_deserial_node(&tree);

        assert_eq!(actual_deserial_node, expected_deserial_node);
        assert_eq!(actual_did_reorder, expected_did_reorder);
    }
}