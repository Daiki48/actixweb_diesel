#[macro_use]
extern crate diesel;

use diesel::{
    ExpressionMethods,
    QueryDsl,
    RunQueryDsl,
};

use actix_web::web::Data;
use actix_web::{
    get,
    post,
    put,
    web,
    App,
    HttpResponse,
    HttpServer,
    Responder,
    Result,
};

mod db;
mod model;
mod schema;

#[get("/users/{id}")]
async fn get(db: web::Data<db::Pool>, path: web::Path<i32>) -> Result<impl Responder> {
    let mut conn = db.get().unwrap();
    let id = path.into_inner();
    let user = schema::users::table
    .select(schema::users::email)
    .filter(schema::users::id.eq(id))
    .load::<String>(&mut conn)
    .expect("error");

    Ok(web::Json(user))
}

#[post("/users")]
async fn post (db: web::Data<db::Pool>, item: web::Json<model::User>) -> Result<impl Responder> {
    let mut conn = db.get().unwrap();
    let new_user = model::User {
        email: item.email.to_string(),
    };
    diesel::insert_into(schema::users::dsl::users)
    .values(&new_user)
    .execute(&mut conn)
    .expect("Error saving new post");

    Ok(HttpResponse::Created().body("get ok"))
}

#[put("/users/{id}")]
async fn put(
    db: web::Data<db::Pool>,
    path: web::Path<i32>,
    item: web::Json<model::User>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let mut conn = db.get().unwrap();
    let target = schema::users::dsl::users.filter(schema::users::dsl::id.eq(id));

    diesel::update(target)
        .set(schema::users::dsl::email.eq(item.email.to_string()))
        .execute(&mut conn)
        .expect("Error updating new post");

    Ok(HttpResponse::Created().body("update ok"))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = db::establish_connection();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(get)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
