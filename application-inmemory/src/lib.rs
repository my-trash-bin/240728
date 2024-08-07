use application_interface::{CreatePostInput, IApplication, Post, PostId};
use chrono::Utc;
use futures::future::ready;
use uuid::Uuid;

pub struct InmemoryApplication {
    posts_sorted: Vec<Post>, // sorted by update_time in ascending order
}

impl InmemoryApplication {
    pub fn new() -> InmemoryApplication {
        InmemoryApplication {
            posts_sorted: Vec::new(),
        }
    }
}

impl IApplication for InmemoryApplication {
    fn create_post<'a, 'b>(
        &mut self,
        input: &'a CreatePostInput<'b>,
    ) -> impl std::future::Future<Output = Post> {
        let id = PostId(Uuid::new_v4().to_string());
        let new_post = Post {
            id,
            title: input.title.to_string(),
            content: input.content.to_string(),
            create_time: Utc::now().to_rfc3339(),
            update_time: Utc::now().to_rfc3339(),
        };
        self.posts_sorted.push(new_post.clone());
        ready(new_post)
    }

    fn get_post(&self, id: &PostId) -> impl futures::Future<Output = Option<Post>> {
        ready(
            self.posts_sorted
                .iter()
                .find(|x| *x.id == **id)
                .map(Post::clone),
        )
    }

    fn get_latest_posts(&self, count: usize) -> impl futures::Future<Output = Vec<Post>> {
        ready(
            self.posts_sorted[if count > self.posts_sorted.len() {
                0
            } else {
                self.posts_sorted.len() - count
            }..self.posts_sorted.len() - 1]
                .to_vec(),
        )
    }
}
