use crate::tests::utils::{
    node,
    DeclarativeTree,
};

#[test]
fn test_insert_into_empty_tree_with_a_non_existent_parent_key() {
    let mut declarative_tree = DeclarativeTree::from_declarative_node(None);

    assert!(!declarative_tree.insert(1, 'a', 0));

    let actual_declarative_node = declarative_tree.into_declarative_node();
    let expected_declarative_node = None;

    assert_eq!(actual_declarative_node, expected_declarative_node);
}

#[test]
fn test_insert_into_single_element_tree_with_a_non_existent_parent_key() {
    let mut declarative_tree = DeclarativeTree::from_declarative_node(Some(&node! { 0, 'a', [] }));

    assert!(!declarative_tree.insert(1, 'b', 100));

    let actual_declarative_node = declarative_tree.into_declarative_node();
    let expected_declarative_node = Some(node! { 0, 'a', [] });

    assert_eq!(actual_declarative_node, expected_declarative_node);
}

#[test]
fn test_insert_into_single_element_tree() {
    let mut declarative_tree = DeclarativeTree::from_declarative_node(Some(&node! { 0, 'a', [] }));

    assert!(declarative_tree.insert(1, 'b', 0));

    let actual_declarative_node = declarative_tree.into_declarative_node();
    let expected_declarative_node = Some(node! { 0, 'a', [
        node! { 1, 'b', [] },
    ] });

    assert_eq!(actual_declarative_node, expected_declarative_node);
}

#[test]
fn test_insert_into_multi_element_as_a_child() {
    let mut declarative_tree = DeclarativeTree::from_declarative_node(Some(&node! { 0, 'a', [
        node! { 1, 'b', [] },
    ] }));

    assert!(declarative_tree.insert(2, 'c', 1));

    let actual_declarative_node = declarative_tree.into_declarative_node();
    let expected_declarative_node = Some(node! { 0, 'a', [
        node! { 1, 'b', [
            node! { 2, 'c', [] },
        ] },
    ] });

    assert_eq!(actual_declarative_node, expected_declarative_node);
}

#[test]
fn test_insert_into_multi_element_as_a_sibling() {
    let mut declarative_tree = DeclarativeTree::from_declarative_node(Some(&node! { 0, 'a', [
        node! { 1, 'b', [
            node! { 2, 'c', [] },
            node! { 3, 'd', [] },
        ] },
    ] }));

    assert!(declarative_tree.insert(4, 'e', 0));

    let actual_declarative_node = declarative_tree.into_declarative_node();
    let expected_declarative_node = Some(node! { 0, 'a', [
        node! { 1, 'b', [
            node! { 2, 'c', [] },
            node! { 3, 'd', [] },
        ] },
        node! { 4, 'e', [] },
    ] });

    assert_eq!(actual_declarative_node, expected_declarative_node);
}
