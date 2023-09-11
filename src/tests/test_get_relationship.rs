use crate::{
    decl_tree::{
        make_reverse_key_map,
        make_tree_and_key_map,
        node,
    },
    Relationship,
};

#[test]
fn test_get_relationship() {
    let tests = [
        ((None, 0, 1), None),
        ((Some(node! { 0 }), 1, 2), None),
        (
            (Some(node! { 0, [node! { 1 }] }), 0, 1),
            Some(Relationship::Ancestral {
                ancestor_key: 0,
                descendent_key: 1,
            }),
        ),
        (
            (Some(node! { 0, [node! { 1 }] }), 0, 0),
            Some(Relationship::Same),
        ),
        (
            (Some(node! { 0, [node! { 1 }] }), 1, 0),
            Some(Relationship::Ancestral {
                ancestor_key: 0,
                descendent_key: 1,
            }),
        ),
        (
            (
                Some(node! {
                    0,
                    [
                        node! { 1 },
                        node! { 4 },
                    ],
                }),
                1,
                4,
            ),
            Some(Relationship::Siblings {
                common_ancestor_key: 0,
            }),
        ),
        (
            (
                Some(node! {
                    0,
                    [
                        node! { 1 },
                        node! { 4 },
                    ],
                }),
                4,
                1,
            ),
            Some(Relationship::Siblings {
                common_ancestor_key: 0,
            }),
        ),
    ];

    for ((decl_tree, key_1, key_2), expected_relationship) in tests {
        let (tree, key_map) = make_tree_and_key_map(decl_tree.as_ref());

        let key_1 = key_map.get(&key_1).copied().unwrap_or_default();
        let key_2 = key_map.get(&key_2).copied().unwrap_or_default();

        let reverse_map = make_reverse_key_map(&key_map);

        let actual_relationship = tree.get_relationship(key_1, key_2);
        let actual_relationship =
            actual_relationship.map(|actual_relationship| match actual_relationship {
                Relationship::Same => Relationship::Same,
                Relationship::Siblings {
                    common_ancestor_key,
                } => {
                    let common_ancestor_key = *reverse_map.get(&common_ancestor_key).unwrap();
                    Relationship::Siblings {
                        common_ancestor_key,
                    }
                }
                Relationship::Ancestral {
                    ancestor_key,
                    descendent_key,
                } => {
                    let ancestor_key = *reverse_map.get(&ancestor_key).unwrap();
                    let descendent_key = *reverse_map.get(&descendent_key).unwrap();
                    Relationship::Ancestral {
                        ancestor_key,
                        descendent_key,
                    }
                }
            });

        assert_eq!(actual_relationship, expected_relationship);
    }
}
