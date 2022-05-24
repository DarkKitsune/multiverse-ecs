use std::{
    any::{Any, TypeId},
    fmt::Debug,
    marker::PhantomData,
};

use as_any::AsAny;

pub trait Class: AsAny {
    fn name(&self) -> &'static str;
    fn component(&self, type_id: TypeId) -> Option<&dyn Any>;
    fn component_mut(&mut self, type_id: TypeId) -> Option<&mut dyn Any>;
}

pub trait __DuplicateComponent {}
pub struct __DuplicateComponentAt<Class, T> {
    __marker: PhantomData<Class>,
    __marker2: PhantomData<T>,
}

#[macro_export]
macro_rules! define_class {
    ($(
        $(#[$outer:meta])*
        $pub:vis class $name:ident {
            $($field:ident: $type:ty),*
            $(,)?
        }
    )*) => {
        $(
            $(#[$outer])*
            $pub struct $name {
                $($field: $type),*
            }

            impl $crate::class::Class for $name {
                fn name(&self) -> &'static str {
                    stringify!($name)
                }

                fn component(&self, type_id: std::any::TypeId) -> Option<&dyn std::any::Any> {
                    match type_id {
                        $(const { std::any::TypeId::of::<$type>() } => Some(&self.$field as &dyn std::any::Any),)*
                        _ => None,
                    }
                }

                fn component_mut(&mut self, type_id: std::any::TypeId) -> Option<&mut dyn std::any::Any> {
                    match type_id {
                        $(const { std::any::TypeId::of::<$type>() } => Some(&mut self.$field as &mut dyn std::any::Any),)*
                        _ => None,
                    }
                }
            }

            $(impl $crate::class::__DuplicateComponent for $crate::class::__DuplicateComponentAt<$name, $type> {})*
        )*
    };
}

impl Class for () {
    fn name(&self) -> &'static str {
        "()"
    }

    fn component(&self, _type_id: TypeId) -> Option<&dyn Any> {
        None
    }

    fn component_mut(&mut self, _type_id: TypeId) -> Option<&mut dyn Any> {
        None
    }
}
