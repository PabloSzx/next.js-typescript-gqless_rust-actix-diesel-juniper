#![feature(decl_macro, proc_macro_hygiene)]

use diesel::*;
use juniper::{object, FieldResult, RootNode};

use rocket::{response::*, State};

pub struct Context {
    pub pool: r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>,
}

impl juniper::Context for Context {}

impl Context {
    fn db_connection<F, T>(&self, action: F) -> T
    where
        F: Fn(r2d2::PooledConnection<ConnectionManager<PgConnection>>) -> T,
    {
        let connection = self.pool.get().unwrap();

        action(connection)
    }
}

extern crate diesel;
extern crate diesel_demo;
extern crate r2d2;

use self::diesel_demo::*;

use crate::models::*;
use diesel_demo::schema::posts::dsl::*;
use diesel_demo::schema::users::dsl::*;

pub struct Query {}

#[object(
    Context = Context
)]
impl Query {
    fn all_posts(ctx: &Context) -> FieldResult<Vec<Post>> {
        ctx.db_connection(|db| {
            let all_posts = posts.load::<Post>(&*db).expect("asd");

            Ok(all_posts)
        })
    }
    fn all_users(ctx: &Context) -> FieldResult<Vec<User>> {
        ctx.db_connection(|db| {
            let mut all_users = users.load::<UserSQL>(&*db).expect("asd");

            Ok(all_users
                .into_iter()
                .map(|v| User {
                    id: v.id,
                    email: v.email,
                    name: v.name,
                    password: v.password,
                    role: from_string_to_role(v.role),
                })
                .rev()
                .collect())
        })
    }
}

pub struct Mutation {}

#[object(
    Context = Context
)]
impl Mutation {
    fn createPost(ctx: &Context, post: NewPost) -> FieldResult<Post> {
        ctx.db_connection(|db| {
            let new_post = diesel::insert_into(schema::posts::table)
                .values(&post)
                .get_result(&*db)
                .expect("error saving");

            Ok(new_post)
        })
    }
    fn createUser(ctx: &Context, user: NewUser) -> FieldResult<User> {
        let db = ctx.pool.get().unwrap();

        let new_user: UserSQL = diesel::insert_into(schema::users::table)
            .values(UserSQLInsert {
                id: user.id,
                email: user.email,
                name: user.name,
                password: user.password,
            })
            .get_result(&*db)?;

        Ok(User {
            id: new_user.id,
            email: new_user.email,
            name: new_user.name,
            password: new_user.password,
            role: from_string_to_role(new_user.role),
        })
    }
    fn updateUser(ctx: &Context, user: UpdateUser) -> FieldResult<Option<User>> {
        let db = ctx.pool.get().unwrap();

        let updated_user: Option<UserSQL> = diesel::update(users.find(user.id))
            .set(UserSQLUpdate {
                id: user.id,
                email: user.email,
                name: user.name,
                password: user.password,
            })
            .get_result(&*db)
            .optional()?;

        Ok(updated_user.and_then(|v| {
            Some(User {
                id: v.id,
                email: v.email,
                name: v.name,
                password: v.password,
                role: from_string_to_role(v.role),
            })
        }))
    }
}

type Schema = RootNode<'static, Query, Mutation>;

#[rocket::get("/")]
fn redirect() -> Redirect {
    Redirect::to("/api/graphql")
}

#[rocket::get("/api/graphql")]
fn playground() -> content::Html<String> {
    juniper_rocket::playground_source("/api/graphql")
}

#[rocket::get("/api/graphql?<request>")]
fn get_graphql_handler(
    ctx: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &ctx)
}

#[rocket::post("/api/graphql", data = "<request>")]
fn post_graphql_handler(
    ctx: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &ctx)
}

use std::env;
extern crate dotenv;
use dotenv::dotenv;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be seta");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = r2d2::Pool::builder().max_size(15).build(manager).unwrap();

    let server = rocket::ignite()
        .manage(Context { pool })
        .manage(Schema::new(Query {}, Mutation {}))
        .mount(
            "/",
            rocket::routes![
                playground,
                get_graphql_handler,
                post_graphql_handler,
                redirect
            ],
        );

    println!("{}", server.config().port);
    server.launch();
}
