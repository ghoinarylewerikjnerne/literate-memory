#[macro_export]
macro_rules! define_concrete_class {
    (
        $data_name:ident {
            $($field_vis:vis $field_name:ident : $field_type:ty),* $(,)?
        },
        $(
            impl $trait_name:ident {
                $($method:item)*
            }
        ),* $(,)?
    ) => {
        // Step 1: Define the data struct
        pub struct $data_name {
            $(
                $field_vis $field_name : $field_type,
            )*
        }

        // Step 2: Implement the traits for the data struct
        $(
            impl $trait_name for $data_name {
                $(
                    $method
                )*
            }
        )*
    };
}
