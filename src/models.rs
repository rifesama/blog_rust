
// Permitimos a las estructuras manipular datos en formato JSON
use serde::{Deserialize, Serialize };
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;


// Estructura para obtener solo algunos campos de los registros
#[derive(Queryable, Debug, Deserialize, Serialize)]
pub struct PostSimplificado {
    pub title: String,
    pub body: String,   

}
// Estructura para obtener los registros completos desde la BBDD
#[derive(Queryable, Debug, Deserialize, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,   

}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewPostHandler {
    pub title: String,
    pub body: String
}

use diesel::prelude::*;
use super::schema::posts;


// Estructura para crear registros en la BBDD con el formato de un post
#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub slug: &'a str,

}

impl Post {

    pub fn slugify(title: &String) -> String {
        return title.replace(" ", "-").to_lowercase();

    }

   pub fn create_post<'a> (
   mut conn: PooledConnection<ConnectionManager<PgConnection>>, 
    post: &NewPostHandler
) -> Result<Post, diesel::result::Error> {

    let slug = Post::slugify(&post.title.clone());

    let new_post = NewPost{
        title: &post.title,
        slug: &slug,
        body: &post.body

    };

   diesel::insert_into(posts::table).values(new_post).get_result::<Post>(&mut conn)
    

   }


}