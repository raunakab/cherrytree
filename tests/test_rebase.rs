#[path = "../common/mod.rs"]
mod common;

use common::{
    make_deserial_node,
    make_tree_and_key_map,
    node,
    DeserialNode,
};

#[test]
fn test_rebase() {
    let tests = [
        ((None, 0, 1), (None, false)),
        ((Some(node! { 0 }), 0, 1), (Some(node! { 0 }), false)),
        ((Some(node! { 0 }), 1, 0), (Some(node! { 0 }), false)),
        ((Some(node! { 0 }), 8, 0), (Some(node! { 0 }), false)),
        ((Some(node! { 0 }), 0, 0), (Some(node! { 0 }), true)),
        (
            (Some(node! { 0, [node! { 1 }, node! { 2 }] }), 1, 2),
            (Some(node! { 0, [node! { 2, [node! { 1 }] } ] }), true),
        ),
        (
            (Some(node! { 0, [node! { 1 }, node! { 2 }] }), 0, 1),
            (Some(node! { 1, [ node! { 0, [ node! { 2 } ] } ] }), true),
        ),
        (
            (Some(node! { 0, [ node! { 1, [ node! { 2 } ] } ] }), 0, 2),
            (Some(node! { 2, [ node! { 0, [ node! { 1 } ] } ] }), true),
        ),
        (
            (
                Some(node! {
                    0,
                    [
                        node! { 10 },
                        node! { 11 },
                        node! { 12, [ node! { 20 }, node! { 21 } ] },
                        node! { 13 },
                    ]
                }),
                0,
                21,
            ),
            (
                Some(node! {
                    21,
                    [
                        node! {
                            0,
                            [
                                node! { 10 },
                                node! { 11 },
                                node! { 12, [ node! { 20 } ] },
                                node! { 13 },
                            ]
                        }
                    ]
                }),
                true,
            ),
        ),
        (
            (
                Some(node! {
                    0,
                    [
                        node! { 10 },
                        node! { 11 },
                        node! { 12, [ node! { 20 }, node! { 21 } ] },
                        node! { 13 },
                    ]
                }),
                12,
                21,
            ),
            (
                Some(node! {
                    0,
                    [
                        node! { 10 },
                        node! { 11 },
                        node! { 13 },
                        node! { 21, [
                            node! { 12, [ node! { 20 } ] }
                        ] }
                    ]
                }),
                true,
            ),
        ),
    ];

    for ((deserial_node, key, new_parent_key), (expected_deserial_node, expected_did_rebase)) in
        tests
    {
        let (mut tree, key_map) = make_tree_and_key_map(deserial_node.as_ref());
        let before_length = tree.len();

        let key = key_map.get(&key).copied().unwrap_or_default();
        let new_parent_key = key_map.get(&new_parent_key).copied().unwrap_or_default();

        let actual_did_rebase = tree.rebase(key, new_parent_key);
        let actual_deserial_node = make_deserial_node(&tree);
        let after_length = tree.len();

        assert_eq!(actual_did_rebase, expected_did_rebase);
        assert_eq!(actual_deserial_node, expected_deserial_node);
        assert_eq!(before_length, after_length);
    }
}
