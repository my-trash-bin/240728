mod models;
mod schema;

use self::models::{NewPost, Post as DieselPost};
use self::schema::posts::dsl::*;
use application_interface::{CreatePostInput, IApplication, Post as ApplicationPost, PostId};
use chrono::{TimeZone, Utc};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use futures::future::ready;
use uuid::Uuid;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub struct DieselApplication {
    pool: DbPool,
}

impl DieselApplication {
    pub fn new(database_url: &str) -> DieselApplication {
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        {
            let mut conn = pool
                .get()
                .expect("Failed to get DB connection for migrations");
            run_migrations(&mut conn).expect("Failed to run migrations");
        }

        DieselApplication { pool }
    }
}

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn map_diesel_post_to_application_post(diesel_post: DieselPost) -> ApplicationPost {
    ApplicationPost {
        id: PostId(diesel_post.id),
        title: diesel_post.title,
        content: diesel_post.content,
        create_time: Utc.from_utc_datetime(&diesel_post.create_time).to_rfc3339(),
        update_time: Utc.from_utc_datetime(&diesel_post.update_time).to_rfc3339(),
    }
}

fn run_migrations(
    conn: &mut SqliteConnection,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    conn.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

impl IApplication for DieselApplication {
    fn create_post<'a, 'b>(
        &mut self,
        input: &'a CreatePostInput<'b>,
    ) -> impl std::future::Future<Output = ApplicationPost> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        let new_post = NewPost {
            id: &Uuid::new_v4().to_string(),
            title: input.title,
            content: input.content,
            create_time: Utc::now().naive_utc(),
            update_time: Utc::now().naive_utc(),
        };

        diesel::insert_into(posts)
            .values(&new_post)
            .execute(&mut conn)
            .expect("Error inserting new post");

        let post = map_diesel_post_to_application_post(
            posts.find(id).first(&mut conn).expect("Error loading post"),
        );

        ready(post)
    }

    fn get_post(&self, post_id: &PostId) -> impl futures::Future<Output = Option<ApplicationPost>> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        let post_id = post_id.0.clone();

        let result = posts
            .filter(id.eq(post_id))
            .first::<DieselPost>(&mut conn)
            .ok()
            .map(map_diesel_post_to_application_post);

        ready(result)
    }

    fn get_latest_posts(
        &self,
        count: usize,
    ) -> impl futures::Future<Output = Vec<ApplicationPost>> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");

        let results = posts
            .order(update_time.desc())
            .limit(count as i64)
            .load::<DieselPost>(&mut conn)
            .expect("Error loading posts");

        ready(
            results
                .into_iter()
                .map(map_diesel_post_to_application_post)
                .collect(),
        )
    }
}
