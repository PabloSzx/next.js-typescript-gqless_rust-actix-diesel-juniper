use crate::schema::posts;
use diesel::*;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(Queryable, GraphQLObject)]
#[graphql(description = "Post type")]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[graphql(description = "New post input type")]
#[derive(Insertable, GraphQLInputObject)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published: Option<bool>,
}
