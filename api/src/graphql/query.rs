use diesel::*;
use juniper::{graphql_object, FieldResult};

use crate::schema::posts::dsl::*;
use crate::schema::users::dsl::*;

extern crate diesel;

extern crate r2d2;

use super::schema::Context;

use crate::type_defs;

pub struct Query {}

#[graphql_object(
    Context = Context
)]
impl Query {
    fn all_posts(ctx: &Context) -> FieldResult<Vec<type_defs::posts::Post>> {
        let db = ctx.pool.get().unwrap();

        let all_posts = posts.load::<type_defs::posts::Post>(&*db).expect("asd");

        Ok(all_posts)
    }
    fn all_users(ctx: &Context) -> FieldResult<Vec<type_defs::users::User>> {
        let db = ctx.pool.get().unwrap();

        let all_users = users.load::<type_defs::users::UserSQL>(&*db).expect("asd");

        Ok(all_users
            .into_iter()
            .map(|v| type_defs::users::User {
                id: v.id,
                email: v.email,
                name: v.name,
                password: v.password,
                role: type_defs::users::from_string_to_role(v.role),
            })
            .rev()
            .collect())
    }
}
