use std::marker::PhantomData;

use crate::{
    class::Class,
    handle_map::{Handle, HandleMap, HandleMapValues, HandleMapValuesMut},
    node::Node,
};

/// A universe which contains any number of nodes.
#[derive(Debug)]
pub struct Universe {
    /// The nodes in the universe
    nodes: HandleMap<Node>,
}

impl Universe {
    /// Creates a new universe.
    pub fn new() -> Self {
        let mut nodes = HandleMap::new();

        Universe { nodes }
    }

    /// Creates a new node in the universe. Returns the node's unique Handle.
    pub fn create_node<C: Class + 'static>(
        &mut self,
        parent_handle: Option<&Handle>,
        class: C,
    ) -> Handle {
        let node = Node::__new(parent_handle, class);
        let node_handle = self.nodes.insert(node);
        self.nodes
            .get_mut(&node_handle)
            .unwrap()
            .__set_handle(node_handle.clone());
        if let Some(parent_handle) = parent_handle {
            self.nodes
                .get_mut(parent_handle)
                .unwrap()
                .__push_child_handle(node_handle.clone());
        }
        node_handle
    }

    /// Find a node in the Universe by its unique handle.
    pub fn node(&self, handle: &Handle) -> Option<&Node> {
        self.nodes.get(handle)
    }

    /// Find a node in the Universe by its handle.
    pub fn node_mut(&mut self, handle: &Handle) -> Option<&mut Node> {
        self.nodes.get_mut(handle)
    }

    /// Returns whether the universe contains a node with the given handle.
    pub fn contains_node(&self, handle: &Handle) -> bool {
        self.nodes.contains(handle)
    }

    /// Returns an iterator over all the nodes in the universe.
    pub fn nodes(&self) -> HandleMapValues<Node> {
        self.nodes.values()
    }

    /// Returns an iterator over all the nodes in the universe.
    pub fn nodes_mut(&mut self) -> HandleMapValuesMut<Node> {
        self.nodes.values_mut()
    }
}

pub trait NodesIter<'a>: Sized + Iterator<Item = &'a Node> {
    /// Filter the iterator to only include nodes with the given class.
    fn with_class<C: Class>(self) -> NodesWithClass<'a, Self, C>;
    /// Filter the iterator to only include nodes with the given component.
    fn with_component<C>(self) -> NodesWithComponent<'a, Self, C>;
    /// Retrieve the handles of the nodes this iterator yields.
    fn handles(self) -> NodesToHandles<'a, Self>;
}

impl<'a, I: Iterator<Item = &'a Node>> NodesIter<'a> for I {
    fn with_class<C: Class>(self) -> NodesWithClass<'a, I, C> {
        NodesWithClass {
            iter: self,
            __marker: PhantomData,
        }
    }

    fn with_component<C>(self) -> NodesWithComponent<'a, Self, C> {
        NodesWithComponent {
            iter: self,
            __marker: PhantomData,
        }
    }

    fn handles(self) -> NodesToHandles<'a, Self> {
        NodesToHandles { iter: self }
    }
}

/// An iterator over nodes in a universe, filtered to a specific class.
pub struct NodesWithClass<'a, I: Iterator<Item = &'a Node>, C: Class + 'a> {
    iter: I,
    __marker: std::marker::PhantomData<C>,
}

impl<'a, I: Iterator<Item = &'a Node>, C: Class + 'a> Iterator for NodesWithClass<'a, I, C> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.iter.next() {
            if node.class_is::<C>() {
                return Some(node);
            }
        }
        None
    }
}

/// An iterator over nodes in a universe, filtered to a specific component.
pub struct NodesWithComponent<'a, I: Iterator<Item = &'a Node>, C: 'static> {
    iter: I,
    __marker: std::marker::PhantomData<C>,
}

impl<'a, I: Iterator<Item = &'a Node>, C: 'static> Iterator for NodesWithComponent<'a, I, C> {
    type Item = (&'a Node, &'a C);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.iter.next() {
            if let Some(component) = node.component::<C>() {
                return Some((node, component));
            }
        }
        None
    }
}

/// An iterator over handles of nodes in a universe.
pub struct NodesToHandles<'a, I: Iterator<Item = &'a Node>> {
    iter: I,
}

impl<'a, I: Iterator<Item = &'a Node>> Iterator for NodesToHandles<'a, I> {
    type Item = &'a Handle;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|node| node.handle())
    }
}

pub trait NodesIterMut<'a>: Sized + Iterator<Item = &'a mut Node> {
    /// Filter the iterator to only include nodes with the given class.
    fn with_class<C: Class>(self) -> NodesWithClassMut<'a, Self, C>;
    /// Filter the iterator to only include nodes with the given component.
    fn with_component<C>(self) -> NodesWithComponentMut<'a, Self, C>;
    /// Retrieve the handles of the nodes this iterator yields.
    fn handles(self) -> NodesToHandlesMut<'a, Self>;
}

impl<'a, I: Iterator<Item = &'a mut Node>> NodesIterMut<'a> for I {
    fn with_class<C: Class>(self) -> NodesWithClassMut<'a, I, C> {
        NodesWithClassMut {
            iter: self,
            __marker: PhantomData,
        }
    }

    fn with_component<C>(self) -> NodesWithComponentMut<'a, Self, C> {
        NodesWithComponentMut {
            iter: self,
            __marker: PhantomData,
        }
    }

    fn handles(self) -> NodesToHandlesMut<'a, Self> {
        NodesToHandlesMut { iter: self }
    }
}

/// An iterator over nodes in a universe, filtered to a specific class.
pub struct NodesWithClassMut<'a, I: Iterator<Item = &'a mut Node>, C: Class + 'a> {
    iter: I,
    __marker: std::marker::PhantomData<C>,
}

impl<'a, I: Iterator<Item = &'a mut Node>, C: Class + 'a> Iterator for NodesWithClassMut<'a, I, C> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.iter.next() {
            if node.class_is::<C>() {
                return Some(node);
            }
        }
        None
    }
}

/// An iterator over nodes in a universe, filtered to a specific component.
pub struct NodesWithComponentMut<'a, I: Iterator<Item = &'a mut Node>, C: 'static> {
    iter: I,
    __marker: std::marker::PhantomData<C>,
}

impl<'a, I: Iterator<Item = &'a mut Node>, C: 'static> Iterator for NodesWithComponentMut<'a, I, C> {
    type Item = &'a mut Node;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.iter.next() {
            if let Some(component) = node.component::<C>() {
                return Some(node);
            }
        }
        None
    }
}

/// An iterator over handles of nodes in a universe.
pub struct NodesToHandlesMut<'a, I: Iterator<Item = &'a mut Node>> {
    iter: I,
}

impl<'a, I: Iterator<Item = &'a mut Node>> Iterator for NodesToHandlesMut<'a, I> {
    type Item = &'a Handle;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|node| node.handle())
    }
}