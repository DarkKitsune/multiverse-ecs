use std::any::{Any, TypeId};

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
        $pub:vis class $name:ident$(<$($lifetime:lifetime,)*$($generic:ident$(:$bound:tt$(+$add_bound:tt)*)?),*>)? {
            $($(#[$field_outer:meta])*$field:ident: $type:ty),*
            $(,)?
        }
    )*) => {
        $(
            $(#[$outer])*
            $pub struct $name$(<$($lifetime,)*$($generic$(:$bound$(+$add_bound)*)?),*>)? {
                $($(#[$field_outer])*$field: $type),*
            }

            impl$(<$($lifetime,)*$($generic$(:$bound$(+$add_bound)*)?),*>)? $crate::class::Class for $name$(<$($lifetime,)*$($generic),*>)? {
                fn name(&self) -> &'static str {
                    stringify!($name)
                }

                fn component(&self, type_id: std::any::TypeId) -> Option<&dyn std::any::Any> {
                    $(
                        if type_id == std::any::TypeId::of::<$type>() {
                            return Some(&self.$field as &dyn std::any::Any);
                        }
                    )*
                    None
                }

                fn component_mut(&mut self, type_id: std::any::TypeId) -> Option<&mut dyn std::any::Any> {
                    $(
                        if type_id == std::any::TypeId::of::<$type>() {
                            return Some(&mut self.$field as &mut dyn std::any::Any);
                        }
                    )*
                    None
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
