use crate::tests::utils::{
    node,
    DeclarativeTree,
};

#[test]
fn test_insert_root_into_empty_tree() {
    let mut declarative_tree = DeclarativeTree::from_declarative_node(None);

    declarative_tree.insert_root(0, 'a');

    let actual_declarative_node = declarative_tree.into_declarative_node();
    let expected_declarative_node = Some(node! { 0, 'a', [] });

    assert_eq!(actual_declarative_node, expected_declarative_node);
}

#[test]
fn test_insert_root_into_single_element_tree() {
    let mut declarative_tree = DeclarativeTree::from_declarative_node(Some(&node! { 0, 'a', [] }));

    declarative_tree.insert_root(1, 'b');

    let actual_declarative_node = declarative_tree.into_declarative_node();
    let expected_declarative_node = Some(node! { 1, 'b', [] });

    assert_eq!(actual_declarative_node, expected_declarative_node);
}

#[test]
fn test_insert_root_into_multi_element_tree() {
    let mut declarative_tree = DeclarativeTree::from_declarative_node(Some(&node! { 0, 'a', [
        node! { 10, 'b', [] },
        node! { 11, 'c', [] },
        node! { 12, 'd', [
            node! { 22, 'e', [
                node! { 32, 'f', [] }
            ] },
        ] },
    ] }));

    declarative_tree.insert_root(1, 'z');

    let actual_declarative_node = declarative_tree.into_declarative_node();
    let expected_declarative_node = Some(node! { 1, 'z', [] });

    assert_eq!(actual_declarative_node, expected_declarative_node);
}
