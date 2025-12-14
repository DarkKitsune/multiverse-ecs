#![allow(incomplete_features)]
#![feature(const_type_id)]

pub mod class;
pub mod node;
pub mod prelude;
pub mod universe;

#[cfg(test)]
mod tests {
    use crate::{
        define_class,
        universe::{NodesIter, Universe},
    };

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
            &[node_handle3.clone(), node_handle4]
        );

        // Change node 2's parent to node 3
        universe.change_parent(&node_handle2, Some(&node_handle3));

        // Assert that node 2 has node 3 as its parent
        assert_eq!(
            universe.node(&node_handle2).unwrap().parent(),
            Some(&node_handle3)
        );

        // Assert that node 3 has node 2 as its only child
        assert_eq!(
            universe.node(&node_handle3).unwrap().children(),
            &[node_handle2]
        );

        // Assert that node 1 has no children
        assert_eq!(universe.node(&node_handle1).unwrap().children(), &[]);
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
