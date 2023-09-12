use crate::tests::utils::{
    node,
    DeclarativeTree,
};

#[test]
fn test_remove_from_empty_tree() {
    let mut declarative_tree = DeclarativeTree::<_, char>::from_declarative_node(None);

    assert!(declarative_tree.remove(0).is_none());

    let actual_declarative_node = declarative_tree.into_declarative_node();
    let expected_declarative_node = None;

    assert_eq!(actual_declarative_node, expected_declarative_node);
}

#[test]
fn test_remove_a_non_existent_key() {
    let mut declarative_tree =
        DeclarativeTree::<_, char>::from_declarative_node(Some(&node! { 0, 'a', [] }));

    assert!(declarative_tree.remove(100).is_none());

    let actual_declarative_node = declarative_tree.into_declarative_node();
    let expected_declarative_node = Some(node! { 0, 'a', [] });

    assert_eq!(actual_declarative_node, expected_declarative_node);
}

#[test]
fn test_remove_root_in_a_single_element_tree() {
    let mut declarative_tree =
        DeclarativeTree::<_, char>::from_declarative_node(Some(&node! { 0, 'a', [] }));

    assert_eq!(declarative_tree.remove(0), Some('a'));

    let actual_declarative_node = declarative_tree.into_declarative_node();
    let expected_declarative_node = None;

    assert_eq!(actual_declarative_node, expected_declarative_node);
}

#[test]
fn test_remove_root_in_a_multi_element_tree() {
    let mut declarative_tree =
        DeclarativeTree::<_, char>::from_declarative_node(Some(&node! { 0, 'a', [
            node! { 1, 'b', [
                node! { 2, 'c', [] }
            ] },
            node! { 3, 'd', [] },
        ] }));

    assert_eq!(declarative_tree.remove(0), Some('a'));

    let actual_declarative_node = declarative_tree.into_declarative_node();
    let expected_declarative_node = None;

    assert_eq!(actual_declarative_node, expected_declarative_node);
}

#[test]
fn test_remove_node_with_no_children() {
    let mut declarative_tree =
        DeclarativeTree::<_, char>::from_declarative_node(Some(&node! { 0, 'a', [
            node! { 1, 'b', [] },
            node! { 2, 'c', [
                node! { 3, 'd', [] }
            ] },
            node! { 4, 'e', [] },
        ] }));

    assert_eq!(declarative_tree.remove(3), Some('d'));

    let actual_declarative_node = declarative_tree.into_declarative_node();
    let expected_declarative_node = Some(node! { 0, 'a', [
        node! { 1, 'b', [] },
        node! { 2, 'c', [] },
        node! { 4, 'e', [] },
    ] });

    assert_eq!(actual_declarative_node, expected_declarative_node);
}

#[test]
fn test_remove_node_with_children() {
    let mut declarative_tree =
        DeclarativeTree::<_, char>::from_declarative_node(Some(&node! { 0, 'a', [
            node! { 1, 'b', [] },
            node! { 2, 'c', [
                node! { 3, 'd', [] }
            ] },
            node! { 4, 'e', [] },
        ] }));

    assert_eq!(declarative_tree.remove(2), Some('c'));

    let actual_declarative_node = declarative_tree.into_declarative_node();
    let expected_declarative_node = Some(node! { 0, 'a', [
        node! { 1, 'b', [] },
        node! { 4, 'e', [] },
    ] });

    assert_eq!(actual_declarative_node, expected_declarative_node);
}
