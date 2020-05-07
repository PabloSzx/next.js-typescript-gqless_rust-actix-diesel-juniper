use diesel::*;
use juniper::{graphql_object, FieldResult};

extern crate diesel;

extern crate r2d2;

use crate::schema;

pub struct Mutation {}

use crate::graphql::schema::Context;

use crate::type_defs;

#[graphql_object(
    Context = Context
)]
impl Mutation {
    fn createPost(
        ctx: &Context,
        post: type_defs::posts::NewPost,
    ) -> FieldResult<type_defs::posts::Post> {
        ctx.db_connection(|db| {
            let new_post = diesel::insert_into(schema::posts::table)
                .values(&post)
                .get_result(&*db)
                .expect("error saving");

            Ok(new_post)
        })
    }
    fn createUser(
        ctx: &Context,
        user: type_defs::users::NewUser,
    ) -> FieldResult<type_defs::users::User> {
        let db = ctx.pool.get().unwrap();

        let new_user: type_defs::users::UserSQL = diesel::insert_into(schema::users::table)
            .values(type_defs::users::UserSQLInsert {
                id: user.id,
                email: user.email,
                name: user.name,
                password: user.password,
            })
            .get_result(&*db)?;

        Ok(type_defs::users::User {
            id: new_user.id,
            email: new_user.email,
            name: new_user.name,
            password: new_user.password,
            role: type_defs::users::from_string_to_role(new_user.role),
        })
    }
    fn updateUser(
        ctx: &Context,
        user: type_defs::users::UpdateUser,
    ) -> FieldResult<Option<type_defs::users::User>> {
        let db = ctx.pool.get().unwrap();
        use schema::users::dsl::users;

        let updated_user: Option<type_defs::users::UserSQL> = diesel::update(users.find(user.id))
            .set(type_defs::users::UserSQLUpdate {
                id: user.id,
                email: user.email,
                name: user.name,
                password: user.password,
            })
            .get_result(&*db)
            .optional()?;

        Ok(updated_user.and_then(|v| {
            Some(type_defs::users::User {
                id: v.id,
                email: v.email,
                name: v.name,
                password: v.password,
                role: type_defs::users::from_string_to_role(v.role),
            })
        }))
    }
}
