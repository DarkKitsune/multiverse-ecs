use std::{
    any::{Any, TypeId},
    fmt::Debug,
};

use crate::{class::Class, handle_map::Handle};

pub struct Node {
    handle: Option<Handle>,
    parent_handle: Option<Handle>,
    children_handles: Vec<Handle>,
    class: Box<dyn Class>,
}

impl Node {
    pub(crate) fn __new<C: Class + 'static>(parent_handle: Option<&Handle>, class: C) -> Self {
        Self {
            handle: None,
            parent_handle: parent_handle.cloned(),
            children_handles: Vec::new(),
            class: Box::new(class),
        }
    }

    pub(crate) fn __set_handle(&mut self, handle: Handle) {
        self.handle = Some(handle);
    }

    pub(crate) fn __push_child_handle(&mut self, handle: Handle) {
        self.children_handles.push(handle);
    }

    /// Returns the handle of this node's parent, if it has one.
    pub fn parent(&self) -> Option<&Handle> {
        self.parent_handle.as_ref()
    }

    /// Returns a slice containing the handles of the children of this node.
    pub fn children(&self) -> &[Handle] {
        &self.children_handles
    }

    /// Returns the node's unique handle
    pub fn handle(&self) -> &Handle {
        self.handle.as_ref().expect("Handle not set!")
    }

    /// Returns the component of type T belonging to this node, if it has one.
    pub fn component<T: 'static>(&self) -> Option<&T> {
        self.class
            .component(TypeId::of::<T>())
            .map(|cmp| cmp.downcast_ref::<T>().unwrap())
    }

    /// Returns the component of type T belonging to this node, if it has one.
    pub fn component_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.class
            .component_mut(TypeId::of::<T>())
            .map(|cmp| cmp.downcast_mut::<T>().unwrap())
    }

    /// Returns the class object of this node.
    pub fn class(&self) -> &dyn Class {
        &*self.class
    }

    /// Returns whether the node has the given type of class
    pub fn class_is<T: Class>(&self) -> bool {
        self.class.as_any().is::<T>()
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("handle", &self.handle)
            .field("parent_handle", &self.parent_handle)
            .field("children_handles", &self.children_handles)
            .field("class", &self.class.name())
            .finish()
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl Eq for Node {}
