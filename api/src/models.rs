use diesel::*;
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(Queryable, GraphQLObject)]
#[graphql(description = "Post type")]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

use super::schema::*;

#[graphql(description = "New post input type")]
#[derive(Insertable, GraphQLInputObject)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published: Option<bool>,
}

#[derive(GraphQLObject)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub password: String,
    pub role: Role,
}

#[derive(Queryable)]
pub struct UserSQL {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub password: String,
    pub role: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct UserSQLInsert {
    pub id: i32,
    pub email: String,
    pub name: Option<String>,
    pub password: String,
}

#[derive(AsChangeset)]
#[table_name = "users"]
pub struct UserSQLUpdate {
    pub id: i32,
    pub email: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
}

#[graphql(description = "New user type")]
#[derive(GraphQLInputObject)]
pub struct NewUser {
    pub id: i32,
    pub email: String,
    pub name: Option<String>,
    pub password: String,
}

#[graphql(description = "New user type")]
#[derive(GraphQLInputObject)]
pub struct UpdateUser {
    pub id: i32,
    pub email: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub role: Option<Role>,
}

#[derive(GraphQLEnum)]
pub enum Role {
    Admin,
    User,
}

pub const ADMIN: &'static str = "ADMIN";
pub const USER: &'static str = "USER";

pub fn from_string_to_role(role_str: String) -> Role {
    match &role_str[..] {
        ADMIN => Role::Admin,
        USER => Role::User,
        _ => Role::User,
    }
}
