#[macro_use] // Se utilizarán macros
extern crate diesel; // Se utilizará diesel de manera intensiva

pub mod schema;
pub mod models;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

fn main() {
    dotenv().ok(); // carga del .env

    let db_url = env::var("DATABASE_URL").expect("db url variable not found");

    let conn = PgConnection::establish(&db_url).expect("Connection refused");

    use self::models::{ Post };
    // use self::schema::posts;
    use self::schema::posts::dsl::*;

    // let new_post = NewPost {
    //     title: "Mi segundo blogpost",
    //     body: "Lorem ipsum sit amet",
    //     slug: "primer-post",
    // };

    // // Insersión en BD
    // diesel::insert_into(posts::table).values(new_post).get_result::<Post>(&conn).expect("Insertion failure");
    // Select * from posts

    let _ = diesel::update(posts.filter(id.eq(1))).set(title.eq("Mi primer blogpost")).get_results::<Post>(&conn).expect("Error en el update");

    let posts_result = posts.load::<Post>(&conn).expect("Query error");

    for post in posts_result {
        print!("{}", post.title);
    }

    diesel::delete(posts.filter(id.eq(1))).execute(&conn).expect("Ha fallado la eliminación del post");

}

