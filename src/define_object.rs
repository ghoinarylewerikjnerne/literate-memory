#[allow(dead_code)]
pub trait Class {}

#[macro_export]
macro_rules! class {
    ($(#[$outer:meta])* $ojbvis:vis $object_name:ident $(implements $($bounds:tt)+)?) => {
        class!(@inner $(#[$outer])* $ojbvis $object_name {T} $($($bounds)+)?);
    };
    ($(#[$outer:meta])* $ojbvis:vis $object_name:ident extends $parent_name:ident $(implements $($bounds:tt)+)?) => {
        class!(@inner $(#[$outer])* $ojbvis $object_name $parent_name {$parent_name<T>} $($($bounds)+)?);
    };
    (@inner $(#[$outer:meta])* $ojbvis:vis $object_name:ident $($parent_name:ident)? {$itype:ident$(<$itgen:tt>)?} $($($bounds:tt)+)?) => {
        $(#[$outer])*
        $ojbvis struct $object_name<T>$( where T: $($bounds)+)? {
            instance: $itype$(<$itgen>)?,
        }

        impl<T> $object_name<T>$( where T: $($bounds)+)? {
            $ojbvis fn new(instance: T) -> $object_name<T> {
                $object_name {
                    instance$(: $parent_name::new(instance) )?
                }
            }
        }

        impl<T> crate::define_object::Class for $object_name<T>$( where T: $($bounds)+)? {}

        impl<T> ::std::ops::Deref for $object_name<T>$( where T: $($bounds)+)? {
            type Target = $itype$(<$itgen>)?;
            fn deref(&self) -> &Self::Target {
                &self.instance
            }
        }

        impl<T> ::std::ops::DerefMut for $object_name<T>$( where T: $($bounds)+)? {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.instance
            }
        }
    };
}
