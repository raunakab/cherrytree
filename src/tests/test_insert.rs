use crate::decl_tree::{
    make_decl_tree,
    make_tree_and_key_map,
    node,
};

#[test]
fn test_insert() {
    let tests = [
        ((None, 0, 1), (None, false)),
        ((Some(node! { 0 }), 2, 1), (Some(node! { 0 }), false)),
        (
            (Some(node! { 0 }), 0, 1),
            (Some(node! { 0, [node! { 1 }] }), true),
        ),
        (
            (
                Some(node! {
                    0,
                    [
                        node! { 1 },
                        node! { 2, [node! { 4, [node! { 5 }] }] },
                        node! { 3 },
                    ],
                }),
                2,
                6,
            ),
            (
                Some(node! {
                    0,
                    [
                        node! { 1 },
                        node! { 2, [node! { 4, [node! { 5 }] }, node! { 6 }] },
                        node! { 3 },
                    ],
                }),
                true,
            ),
        ),
    ];

    for ((decl_tree, parent_key, value_to_insert), (expected_decl_tree, expected_did_insert)) in
        tests
    {
        let (mut tree, mut key_map) = make_tree_and_key_map(decl_tree.as_ref());

        let parent_key = key_map.get(&parent_key).copied().unwrap_or_default();
        let key = tree.insert(value_to_insert, parent_key);

        if let Some(key) = key {
            key_map.insert(value_to_insert, key);
        };

        let actual_decl_tree = make_decl_tree(&tree);
        let actual_did_insert = key.is_some();

        assert_eq!(actual_decl_tree, expected_decl_tree);
        assert_eq!(actual_did_insert, expected_did_insert);
    }
}
