#[derive(Queryable, Debug)] // Se puede convertir en un row de SQL
pub struct PostSimplificado {
    pub title: String,
    pub body: String,
}

#[derive(Queryable)] // Se puede convertir en un row de SQL
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
}

use super::schema::posts;

#[derive(Insertable)] // Insersión de SQL
#[table_name="posts"] // Tabla dónde hará insersión
pub struct NewPost<'a> { // habilita el uso de &str
    pub title: &'a str,
    pub body: &'a str,
    pub slug: &'a str,
}