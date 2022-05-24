#![feature(inline_const_pat)]
#![feature(const_type_id)]

pub mod class;
pub mod handle_map;
pub mod node;
pub mod unique;
pub mod universe;

#[cfg(test)]
mod tests {
    use std::convert::identity;

    use crate::{
        define_class,
        unique::Unique,
        universe::{NodesIter, Universe},
    };

    #[test]
    fn unique() {
        // Create two unique handles
        let unique1 = Unique::new();
        let unique2 = Unique::new();
        // Assert that the handles are equal to themselves
        assert_eq!(unique1, unique1);
        assert_eq!(unique2, unique2);
        // Assert that a handle is equal to its clone
        assert_eq!(unique1, unique1.clone());
        // Assert that two individually created handles are not equal
        assert_ne!(unique1, unique2);
    }

    #[test]
    fn node_lookup() {
        // Create a universe
        let mut universe = Universe::new();
        // Create node 1
        let node_handle1 = universe.create_node(None, ());
        // Create node 2 as a child node for node 1
        let node_handle2 = universe.create_node(Some(&node_handle1), ());
        // Create 2 children nodes for node 2
        let node_handle3 = universe.create_node(Some(&node_handle2), ());
        let node_handle4 = universe.create_node(Some(&node_handle2), ());
        // Assert that node 2 has node 1 as its parent
        assert_eq!(
            universe.node(&node_handle2).unwrap().parent(),
            Some(&node_handle1)
        );
        // Assert that node 1 has node 2 as its only child
        assert_eq!(
            universe.node(&node_handle1).unwrap().children(),
            &[node_handle2.clone()]
        );
        // Assert that node 2 has node 3 and node 4 as its only children
        assert_eq!(
            universe.node(&node_handle2).unwrap().children(),
            &[node_handle3, node_handle4]
        );
    }

    #[test]
    fn node_component_lookup() {
        // Define some components
        #[derive(Debug, PartialEq)]
        struct Name(String);
        #[derive(Debug, PartialEq)]
        struct Age(u32);
        // Define some classes of components
        define_class! {
            class Cat {
                name: Name,
                age: Age,
            }

            class Dog {
                name: Name,
                age: Age,
            }
        }
        // Create a universe
        let mut universe = Universe::new();
        // Create a cat node
        let cat_node_handle = universe.create_node(
            None,
            Cat {
                name: Name("Garfield".to_string()),
                age: Age(5),
            },
        );
        // Create two dog nodes
        let dog_node_handle1 = universe.create_node(
            None,
            Dog {
                name: Name("Odie".to_string()),
                age: Age(3),
            },
        );
        let dog_node_handle2 = universe.create_node(
            None,
            Dog {
                name: Name("Boomer".to_string()),
                age: Age(5),
            },
        );
        // Assert that the cat node has a Name component
        assert_eq!(
            universe.node(&cat_node_handle).unwrap().component::<Name>(),
            Some(&Name("Garfield".to_string()))
        );
        // Assert that the cat node has an Age component
        assert_eq!(
            universe.node(&cat_node_handle).unwrap().component::<Age>(),
            Some(&Age(5))
        );
        // Test search the universe for nodes with a Name component
        assert_eq!(
            universe
                .nodes()
                .with_component::<Name>()
                .map(|(node, _name)| node)
                .handles()
                .collect::<Vec<_>>(),
            &[&cat_node_handle, &dog_node_handle1, &dog_node_handle2]
        );
        // Test search the universe for nodes with a name of "Odie"
        assert_eq!(
            universe
                .nodes()
                .with_component::<Name>()
                .filter_map(|(node, name)| if &name.0 == "Odie" { Some(node) } else { None })
                .handles()
                .collect::<Vec<_>>(),
            &[&dog_node_handle1]
        );
        // Test search the universe for dog nodes
        assert_eq!(
            universe
                .nodes()
                .with_class::<Dog>()
                .handles()
                .collect::<Vec<_>>(),
            &[&dog_node_handle1, &dog_node_handle2]
        );
        // Test search the universe for dog nodes with an age of 5
        assert_eq!(
            universe
                .nodes()
                .with_class::<Dog>()
                .filter(|node| node.component::<Age>() == Some(&Age(5)))
                .handles()
                .collect::<Vec<_>>(),
            &[&dog_node_handle2]
        );
    }
}
