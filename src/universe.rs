use std::marker::PhantomData;

use ggutil::prelude::*;

use crate::{class::Class, node::Node};

/// A universe which contains any number of nodes.
#[derive(Debug)]
pub struct Universe {
    /// The nodes in the universe
    nodes: HandleMap<Node>,
    roots: Vec<Handle>,
}

impl Universe {
    /// Creates a new universe.
    pub fn new() -> Self {
        let nodes = HandleMap::new();
        let roots = Vec::new();

        Universe { nodes, roots }
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
        } else {
            self.roots.push(node_handle.clone());
        }
        node_handle
    }

    /// Changes a node's parent.
    /// Returns the node's old parent's unique Handle, if it had one.
    pub fn change_parent(
        &mut self,
        node_handle: &Handle,
        new_parent_handle: Option<&Handle>,
    ) -> Option<Handle> {
        let old_parent_handle = self
            .node(node_handle)
            .expect("No node pointed to by this handle to change the parent of")
            .parent()
            .cloned();
        if let Some(old_parent_handle) = &old_parent_handle {
            self.nodes
                .get_mut(old_parent_handle)
                .unwrap()
                .__remove_child_handle(node_handle);
        }
        if let Some(new_parent_handle) = new_parent_handle {
            self.nodes
                .get_mut(new_parent_handle)
                .unwrap()
                .__push_child_handle(node_handle.clone());
        }
        self.node_mut(node_handle)
            .expect("No node pointed to by this handle to change the parent of")
            .__set_parent_handle(new_parent_handle);
        old_parent_handle
    }

    /// Find a node in the Universe by its unique handle.
    pub fn node(&self, handle: &Handle) -> Option<&Node> {
        self.nodes.get(handle)
    }

    /// Find a node in the Universe by its handle.
    pub fn node_mut(&mut self, handle: &Handle) -> Option<&mut Node> {
        self.nodes.get_mut(handle)
    }

    /// Returns an iterator over the nodes with the given handles.
    pub fn nodes_with_handles<'a>(
        &'a self,
        handles: &'a [Handle],
    ) -> impl Iterator<Item = Option<&'a Node>> {
        handles.iter().map(|handle| self.nodes.get(handle))
    }

    /// Calls the given function on the nodes with the given handles.
    pub fn using_nodes_with_handles<'a, R, F: FnMut(Option<&'a Node>) -> R + 'a>(
        &'a self,
        handles: &'a [Handle],
        mut f: F,
    ) -> impl Iterator<Item = R> + 'a {
        handles.iter().map(move |handle| f(self.nodes.get(handle)))
    }

    /// Calls the given function on the nodes with the given handles.
    pub fn using_nodes_with_handles_mut<R, F: FnMut(Option<&mut Node>) -> R>(
        &mut self,
        handles: &[Handle],
        mut f: F,
    ) -> Vec<R> {
        let mut results = Vec::new();
        for handle in handles {
            results.push(f(self.nodes.get_mut(handle)));
        }
        results
    }

    /// Returns a slice containing the handles of the root nodes in the universe (nodes with no parent).
    pub fn root_node_handles(&self) -> &[Handle] {
        &self.roots
    }

    /// Calls the given function on the root nodes in the universe.
    pub fn using_root_nodes<'a, R: 'a, F: FnMut(Option<&'a Node>) -> R + 'a>(
        &'a self,
        f: F,
    ) -> impl Iterator<Item = R> + 'a {
        self.using_nodes_with_handles(self.root_node_handles(), f)
    }

    /// Calls the given function on the root nodes in the universe.
    pub fn using_root_nodes_mut<R, F: FnMut(Option<&mut Node>) -> R>(&mut self, f: F) -> Vec<R> {
        let handles = self.root_node_handles().to_owned();
        self.using_nodes_with_handles_mut(&handles, f)
    }

    /// Returns whether the universe contains a node with the given handle.
    pub fn contains_node(&self, handle: &Handle) -> bool {
        self.nodes.contains(handle)
    }

    /// Returns an iterator over all the nodes in the universe.
    pub fn nodes(&self) -> HandleMapValues<'_, Node> {
        self.nodes.values()
    }

    /// Returns an iterator over all the nodes in the universe.
    pub fn nodes_mut(&mut self) -> HandleMapValuesMut<'_, Node> {
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
    type Item = &'a mut Node;

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

impl<'a, I: Iterator<Item = &'a mut Node>, C: 'static> Iterator
    for NodesWithComponentMut<'a, I, C>
{
    type Item = &'a mut Node;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.iter.next() {
            if node.component::<C>().is_some() {
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
