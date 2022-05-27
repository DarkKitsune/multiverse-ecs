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
                    #[allow(unreachable_patterns)]
                    match type_id {
                        $(const { std::any::TypeId::of::<$type>() } => Some(&self.$field as &dyn std::any::Any),)*
                        _ => None,
                    }
                }

                fn component_mut(&mut self, type_id: std::any::TypeId) -> Option<&mut dyn std::any::Any> {
                    #[allow(unreachable_patterns)]
                    match type_id {
                        $(const { std::any::TypeId::of::<$type>() } => Some(&mut self.$field as &mut dyn std::any::Any),)*
                        _ => None,
                    }
                }
            }
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

pub trait ClassDynComponent {
    fn component<C: 'static>(&self) -> Option<&C>;
}

impl<T: Class> ClassDynComponent for T {
    fn component<C: 'static>(&self) -> Option<&C> {
        Class::component(self, TypeId::of::<C>()).map(|c| c.downcast_ref().unwrap())
    }
}