use std::{future::Future, ops::Deref};

macro_rules! define_id_type {
    ($type_name: ident) => {
        #[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Post {
    pub id: PostId,
    pub title: String,
    pub content: String,
    pub create_time: String,
    pub update_time: String,
}

pub struct CreatePostInput<'a> {
    pub title: &'a str,
    pub content: &'a str,
}

pub trait IApplication {
    fn create_post<'a, 'b>(&mut self, input: &'a CreatePostInput<'b>)
        -> impl Future<Output = Post>;

    fn get_post(&self, id: &PostId) -> impl Future<Output = Option<Post>>;

    fn get_latest_posts(&self, count: usize) -> impl Future<Output = Vec<Post>>;
}
