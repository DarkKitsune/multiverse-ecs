use std::fmt::{self, Debug, Formatter};

use crate::unique::Unique;

/// A map from unique handles to values.
#[derive(Debug)]
pub(crate) struct HandleMap<T> {
    unique: Unique,
    values: Vec<HandleMapValue<T>>,
}

impl<T> HandleMap<T> {
    /// Creates a new, empty HandleMap
    pub fn new() -> Self {
        HandleMap {
            unique: Unique::new(),
            values: Vec::new(),
        }
    }

    /// Inserts a value into the map.
    pub fn insert(&mut self, value: T) -> Handle {
        let index = self.first_none();
        if let Some(index) = index {
            let handle_unique = Unique::new();
            self.values[index] = HandleMapValue::Some(handle_unique.clone(), value);
            Handle {
                map_unique: self.unique.clone(),
                unique: handle_unique,
                index,
            }
        } else {
            let index = self.values.len();
            let handle_unique = Unique::new();
            self.values
                .push(HandleMapValue::Some(handle_unique.clone(), value));
            Handle {
                map_unique: self.unique.clone(),
                unique: handle_unique,
                index,
            }
        }
    }

    /// Removes a value from the map.
    pub fn remove(&mut self, handle: &Handle) -> Option<T> {
        self.values
            .get_mut(handle.index)
            .and_then(|value| value.take())
            .map(|(_, value)| value)
    }

    /// Gets a value from the map.
    pub fn get(&self, handle: &Handle) -> Option<&T> {
        let v = self.values[handle.index].as_ref();
        if let Some((unique, value)) = v {
            if unique == &handle.unique {
                Some(value)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Gets a mutable reference to a value in the map.
    pub fn get_mut<'a>(&'a mut self, handle: &Handle) -> Option<&'a mut T> {
        let v = self.values[handle.index].as_mut();
        if let Some((unique, value)) = v {
            if unique == &handle.unique {
                Some(value)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Checks if the map contains a value with the given handle.
    pub fn contains(&self, handle: &Handle) -> bool {
        let v = self.values[handle.index].as_ref();
        if let Some((unique, _)) = v {
            unique == &handle.unique
        } else {
            false
        }
    }

    /// Finds the first index in the internal vector that points to a value of None.
    fn first_none(&self) -> Option<usize> {
        self.values.iter().position(|value| value.is_none())
    }

    /// Gets the number of values in the map.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns true if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Gets an iterator over the values in the map.
    pub fn values(&self) -> HandleMapValues<T> {
        HandleMapValues {
            values: self.values.iter(),
        }
    }

    /// Gets an iterator over mutable references to the values in the map.
    pub fn values_mut(&mut self) -> HandleMapValuesMut<T> {
        HandleMapValuesMut {
            values: self.values.iter_mut(),
        }
    }

    pub fn handles(&self) -> impl Iterator<Item = Handle> + '_ {
        self.values.iter().enumerate().filter_map(|(index, value)| {
            value.as_ref().map(|(handle_unique, _)| Handle {
                map_unique: self.unique.clone(),
                unique: handle_unique.clone(),
                index,
            })
        })
    }

    pub fn handles_mut(&mut self) -> impl Iterator<Item = Handle> + '_ {
        self.values
            .iter_mut()
            .enumerate()
            .filter_map(|(index, value)| {
                value.as_mut().map(|(handle_unique, _)| Handle {
                    map_unique: self.unique.clone(),
                    unique: handle_unique.clone(),
                    index,
                })
            })
    }
}

/// A handle to a value in a HandleMap.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Handle {
    map_unique: Unique,
    unique: Unique,
    index: usize,
}

/// A value that can be stored in a HandleMap.
#[derive(Debug)]
enum HandleMapValue<T> {
    Some(Unique, T),
    None,
}

impl<T> HandleMapValue<T> {
    fn is_none(&self) -> bool {
        match self {
            HandleMapValue::Some(_, _) => false,
            HandleMapValue::None => true,
        }
    }

    fn as_ref(&self) -> Option<(&Unique, &T)> {
        match self {
            HandleMapValue::Some(unique, value) => Some((unique, value)),
            HandleMapValue::None => None,
        }
    }

    fn as_mut(&mut self) -> Option<(&mut Unique, &mut T)> {
        match self {
            HandleMapValue::Some(unique, value) => Some((unique, value)),
            HandleMapValue::None => None,
        }
    }

    fn take(&mut self) -> Option<(Unique, T)> {
        match self {
            HandleMapValue::Some(unique, value) => {
                let value = std::mem::replace(self, HandleMapValue::None);
                match value {
                    HandleMapValue::Some(unique, value) => Some((unique, value)),
                    HandleMapValue::None => None,
                }
            }
            HandleMapValue::None => None,
        }
    }
}

pub struct HandleMapValues<'a, T> {
    values: std::slice::Iter<'a, HandleMapValue<T>>,
}

impl<'a, T> Iterator for HandleMapValues<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.values
            .next()
            .and_then(|value| value.as_ref().map(|(_, value)| value))
    }
}

pub struct HandleMapValuesMut<'a, T> {
    values: std::slice::IterMut<'a, HandleMapValue<T>>,
}

impl<'a, T> Iterator for HandleMapValuesMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.values
            .next()
            .and_then(|value| value.as_mut().map(|(_, value)| value))
    }
}
