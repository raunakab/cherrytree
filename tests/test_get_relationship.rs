#[path = "../common/mod.rs"]
mod common;

use tinytree::Relationship;
use common::DeclarativeTree;

#[test]
fn test_get_relationship_from_empty_tree() {
    let declarative_tree = DeclarativeTree::<_, char>::from_declarative_node(None);

    assert!(declarative_tree.get_relationship(0, 1).is_none());
    assert!(declarative_tree.get_relationship(1, 0).is_none());
}

#[test]
fn test_get_relationship_from_single_element_tree() {
    let declarative_tree =
        DeclarativeTree::<_, char>::from_declarative_node(Some(&node! { 0, 'a', [] }));

    assert!(declarative_tree.get_relationship(0, 1).is_none());
    assert!(declarative_tree.get_relationship(1, 0).is_none());
}

#[test]
fn test_get_relationship_from_multi_element_tree_with_non_existent_keys() {
    let declarative_tree =
        DeclarativeTree::<_, char>::from_declarative_node(Some(&node! { 0, 'a', [
            node! { 1, 'b', [] },
            node! { 2, 'c', [
                node! { 3, 'd', [] },
            ] },
        ] }));

    assert!(declarative_tree.get_relationship(100, 200).is_none());
    assert!(declarative_tree.get_relationship(200, 100).is_none());
}

#[test]
fn test_get_relationship_from_multi_element_sibling() {
    let declarative_tree =
        DeclarativeTree::<_, char>::from_declarative_node(Some(&node! { 0, 'a', [
            node! { 1, 'b', [] },
            node! { 2, 'c', [
                node! { 3, 'd', [
                    node! { 4, 'e', [] },
                    node! { 5, 'f', [
                        node! { 6, 'g', [] },
                        node! { 7, 'h', [] },
                    ] }
                ] },
                node! { 8, 'i', [
                    node! { 9, 'j', [] },
                ] },
            ] },
        ] }));

    assert_eq!(
        declarative_tree.get_relationship(7, 9),
        Some(Relationship::Siblings {
            common_ancestor_key: 2
        })
    );
    assert_eq!(
        declarative_tree.get_relationship(9, 7),
        Some(Relationship::Siblings {
            common_ancestor_key: 2
        })
    );
}

#[test]
fn test_get_relationship_from_multi_element_ancestral() {
    let declarative_tree =
        DeclarativeTree::<_, char>::from_declarative_node(Some(&node! { 0, 'a', [
            node! { 1, 'b', [] },
            node! { 2, 'c', [
                node! { 3, 'd', [
                    node! { 4, 'e', [] },
                    node! { 5, 'f', [
                        node! { 6, 'g', [] },
                        node! { 7, 'h', [] },
                    ] }
                ] },
                node! { 8, 'i', [
                    node! { 9, 'j', [] },
                ] },
            ] },
        ] }));

    assert_eq!(
        declarative_tree.get_relationship(2, 7),
        Some(Relationship::Ancestral {
            ancestor_key: 2,
            descendent_key: 7
        })
    );
    assert_eq!(
        declarative_tree.get_relationship(7, 2),
        Some(Relationship::Ancestral {
            ancestor_key: 2,
            descendent_key: 7
        })
    );
}
