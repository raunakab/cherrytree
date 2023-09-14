#[path = "../common/mod.rs"]
mod common;

use common::DeclarativeTree;

#[test]
fn test_rebase_with_empty_tree() {
    let mut declarative_tree = DeclarativeTree::<_, char>::from_declarative_node(None);

    assert!(!declarative_tree.rebase(1, 0));
    assert!(!declarative_tree.rebase(0, 1));
    assert!(!declarative_tree.rebase(1, 1));
    assert!(!declarative_tree.rebase(0, 0));

    let actual_declarative_node = declarative_tree.into_declarative_node();
    let expected_declarative_node = None;

    assert_eq!(actual_declarative_node, expected_declarative_node);
}

#[test]
fn test_rebase_onto_self() {
    let mut declarative_tree = DeclarativeTree::from_declarative_node(Some(&node! { 0, 'a', [] }));

    assert!(!declarative_tree.rebase(0, 0));
    assert!(!declarative_tree.rebase(1, 1));

    let actual_declarative_node = declarative_tree.into_declarative_node();
    let expected_declarative_node = Some(node! { 0, 'a', [] });

    assert_eq!(actual_declarative_node, expected_declarative_node);
}

#[test]
fn test_rebase_onto_sibling() {
    let mut declarative_tree = DeclarativeTree::from_declarative_node(Some(&node! { 0, 'a', [
        node! { 1, 'b', [
            node! { 3, 'd', [
                node! { 6, 'g', [] }
            ] },
        ] },
        node! { 2, 'c', [
            node! { 4, 'e', [] },
            node! { 5, 'f', [] },
        ] },
    ] }));

    assert!(declarative_tree.rebase(3, 5));

    let actual_declarative_node = declarative_tree.into_declarative_node();
    let expected_declarative_node = Some(node! { 0, 'a', [
        node! { 1, 'b', [] },
        node! { 2, 'c', [
            node! { 4, 'e', [] },
            node! { 5, 'f', [
                node! { 3, 'd', [
                    node! { 6, 'g', [] }
                ] },
            ] },
        ] },
    ] });

    assert_eq!(actual_declarative_node, expected_declarative_node);
}

#[test]
fn test_rebase_onto_ancestor() {
    let mut declarative_tree = DeclarativeTree::from_declarative_node(Some(&node! { 0, 'a', [
        node! { 1, 'b', [
            node! { 3, 'd', [] },
        ] },
        node! { 2, 'c', [
            node! { 4, 'e', [
                node! { 6, 'g', [
                    node! { 7, 'h', [] },
                    node! { 8, 'i', [] },
                ] },
            ] },
            node! { 5, 'f', [] },
        ] },
    ] }));

    assert!(declarative_tree.rebase(6, 2));

    let actual_declarative_node = declarative_tree.into_declarative_node();
    let expected_declarative_node = Some(node! { 0, 'a', [
        node! { 1, 'b', [
            node! { 3, 'd', [] },
        ] },
        node! { 2, 'c', [
            node! { 4, 'e', [] },
            node! { 5, 'f', [] },
            node! { 6, 'g', [
                    node! { 7, 'h', [] },
                    node! { 8, 'i', [] },
            ] },
        ] },
    ] });

    assert_eq!(actual_declarative_node, expected_declarative_node);
}

#[test]
fn test_rebase_onto_descendent() {
    let mut declarative_tree = DeclarativeTree::from_declarative_node(Some(&node! { 0, 'a', [
        node! { 1, 'b', [
            node! { 3, 'd', [] },
        ] },
        node! { 2, 'c', [
            node! { 4, 'e', [
                node! { 6, 'g', [
                    node! { 7, 'h', [] },
                    node! { 8, 'i', [] },
                ] },
            ] },
            node! { 5, 'f', [] },
        ] },
    ] }));

    assert!(declarative_tree.rebase(2, 6));

    let actual_declarative_node = declarative_tree.into_declarative_node();
    let expected_declarative_node = Some(node! { 0, 'a', [
        node! { 1, 'b', [
            node! { 3, 'd', [] },
        ] },
        node! { 6, 'g', [
                node! { 7, 'h', [] },
                node! { 8, 'i', [] },
                node! { 2, 'c', [
                    node! { 4, 'e', [] },
                    node! { 5, 'f', [] },
                ] },
        ] },
    ] });

    assert_eq!(actual_declarative_node, expected_declarative_node);
}
