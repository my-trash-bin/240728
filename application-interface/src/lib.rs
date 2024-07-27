use std::ops::Deref;

macro_rules! define_id_type {
    ($type_name: ident) => {
        pub struct $type_name(pub String);

        impl Deref for $type_name {
            type Target = String;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

define_id_type!(PostId);

pub trait IApplication {
    fn create_post(content: &str) -> PostId;
}
