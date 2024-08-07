use chrono::NaiveDateTime;
// src/models.rs
use diesel::prelude::*;

use crate::schema::posts;

#[derive(Queryable, Identifiable, Clone, Debug)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub content: &'a str,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}
