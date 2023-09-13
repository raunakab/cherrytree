use crate::tests::utils::{
    node,
    DeclarativeTree,
};

#[test]
fn test_reorder_children_on_empty_tree() {
    let mut declarative_tree = DeclarativeTree::<_, char>::from_declarative_node(None);

    assert!(!declarative_tree.reorder_children(0, |_| [0, 1, 2, 3].into()));

    let actual_declarative_node = declarative_tree.into_declarative_node();
    let expected_declarative_node = None;

    assert_eq!(actual_declarative_node, expected_declarative_node);
}

#[test]
fn test_reorder_children_on_single_element_tree_with_non_existent_key() {
    let mut declarative_tree = DeclarativeTree::from_declarative_node(Some(&node! { 0, 'a', [] }));

    assert!(!declarative_tree.reorder_children(1, |_| [0, 1, 2, 3].into()));

    let actual_declarative_node = declarative_tree.into_declarative_node();
    let expected_declarative_node = Some(node! { 0, 'a', [] });

    assert_eq!(actual_declarative_node, expected_declarative_node);
}

#[test]
fn test_reorder_children_on_single_element_tree() {
    let mut declarative_tree = DeclarativeTree::from_declarative_node(Some(&node! { 0, 'a', [] }));

    // Fails if nonsense child-keys are passed in!
    assert!(!declarative_tree.reorder_children(0, |_| [0, 1, 2, 3].into()));

    let actual_declarative_node = declarative_tree.into_declarative_node();
    let expected_declarative_node = Some(node! { 0, 'a', [] });

    assert_eq!(actual_declarative_node, expected_declarative_node);
}

#[test]
fn test_reorder_children_with_no_deletions() {
    let mut declarative_tree = DeclarativeTree::from_declarative_node(Some(&node! { 0, 'a', [
        node! { 1, 'b', [
            node! { 4, 'e', [] },
            node! { 5, 'f', [] },
        ] },
        node! { 2, 'c', [] },
        node! { 3, 'd', [
            node! { 6, 'g', [] },
        ] },
    ] }));

    assert!(declarative_tree.reorder_children(0, |_| [3, 2, 1].into()));

    let actual_declarative_node = declarative_tree.into_declarative_node();
    let expected_declarative_node = Some(node! { 0, 'a', [
        node! { 3, 'd', [
            node! { 6, 'g', [] },
        ] },
        node! { 2, 'c', [] },
        node! { 1, 'b', [
            node! { 4, 'e', [] },
            node! { 5, 'f', [] },
        ] },
    ] });

    assert_eq!(actual_declarative_node, expected_declarative_node);
}

#[test]
fn test_reorder_children_with_deletions() {
    let mut declarative_tree = DeclarativeTree::from_declarative_node(Some(&node! { 0, 'a', [
        node! { 1, 'b', [
            node! { 4, 'e', [] },
            node! { 5, 'f', [] },
        ] },
        node! { 2, 'c', [] },
        node! { 3, 'd', [
            node! { 6, 'g', [] },
        ] },
    ] }));

    assert!(declarative_tree.reorder_children(0, |_| [3, 2].into()));

    let actual_declarative_node = declarative_tree.into_declarative_node();
    let expected_declarative_node = Some(node! { 0, 'a', [
        node! { 3, 'd', [
            node! { 6, 'g', [] },
        ] },
        node! { 2, 'c', [] },
    ] });

    assert_eq!(actual_declarative_node, expected_declarative_node);
}
