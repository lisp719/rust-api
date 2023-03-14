use crate::schema::posts;
use async_graphql::{SimpleObject, ID};
use diesel::prelude::*;

#[derive(Clone, SimpleObject)]
pub struct FileInfo {
    pub id: ID,
    pub url: String,
}

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub memo: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
