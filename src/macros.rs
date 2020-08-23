pub use quiet_macros::test;

macro_rules! env {
    ($name:expr, $or:expr) => {
        std::env::var($name).unwrap_or(String::from($or))
    };

    ($name:expr) => {
        std::env::var($name).ok()
    };

    ($name:expr; required) => {
        std::env::var($name).expect(&format!(
            "Cannot find required environment variable {}",
            $name
        ))
    };
}

/*macro_rules! gen_model_builder {
    ($name:ident {$($field:ident: $type:ty),*}) => {
        #[derive(Default, Debug, ::serde::Serialize, ::serde::Deserialize)]
        pub struct $name {
            $(
                pub $field: $type
            ),*
        }

        impl $name {
            pub fn new() -> Self {
                std::default::Default::default()
            }

            $(
                pub fn $field(mut self, val: $type) -> Self {
                    self.$field = val;
                    self
                }
            )*
        }
    };
}*/
