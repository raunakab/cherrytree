use crate::tests::utils::{
    make_decl_tree,
    make_tree_and_key_map,
    node,
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

    for ((decl_tree, value_to_insert), expected_decl_tree) in tests {
        let (mut tree, _) = make_tree_and_key_map(decl_tree.as_ref());

        let _ = tree.insert_root(value_to_insert);

        let actual_decl_tree = make_decl_tree(&tree);
        assert_eq!(actual_decl_tree, expected_decl_tree);
    }
}
