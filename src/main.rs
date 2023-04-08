
extern crate diesel;

pub mod schema;
pub mod models;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use tera::Tera;
use dotenvy::dotenv;
use std::env;

use diesel::prelude::*;
use diesel::pg::PgConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
use diesel::r2d2::{self, ConnectionManager};
use diesel::r2d2::Pool;


use self::models::{Post, NewPostHandler};
use self::schema::posts::dsl::*;


// Endpoint GET que devuelve texto, utiliamos el Macro GET para indicar el verbo HTTP
#[get("/")]
async fn index(pool: web::Data<DbPool>, template_manager: web::Data<tera::Tera>) -> impl Responder {
    
    let mut conn = pool.get(). expect("Problemas al traerl la base de datos");

    match web::block(move || {posts.load::<Post>(&mut conn)}).await {
        Ok(data) => {
            let data = data.unwrap();


           let mut ctx = tera::Context::new();   
            ctx.insert("posts", &data);

           HttpResponse::Ok().content_type("text/html").body(
            template_manager.render("index.html", &ctx).unwrap()
        )                        

            //  HttpResponse::Ok().body(format!("{:?}", data))
        },
        Err(_err) => HttpResponse::Ok().body("Error al recibir data")

    }
       
}

#[get("/blog/{blog_slug}")]
async fn get_post(pool: web::Data<DbPool>,
     template_manager: web::Data<tera::Tera>, 
    blog_slug: web::Path<String>
) -> impl Responder {
    
    let mut conn = pool.get(). expect("Problemas al traerl la base de datos");
    
    let url_slug = blog_slug.into_inner();

    match web::block(move || {posts.filter(slug.eq(url_slug)).load::<Post>(&mut conn)}).await {
        Ok(data) => {
            let data = data.unwrap();

            if data.len() == 0 {
                return HttpResponse::NotFound().finish();
            }

            let data = &data [0];

           let mut ctx = tera::Context::new();   
            ctx.insert("post", data);

           HttpResponse::Ok().content_type("text/html").body(
            template_manager.render("post.html", &ctx).unwrap()
        )                        

            //  HttpResponse::Ok().body(format!("{:?}", data))
        },
        Err(_err) => HttpResponse::Ok().body("Error al recibir data")

    }
       
}

#[post("/new_post")]
async fn new_post(pool: web::Data<DbPool>, item: web::Json<NewPostHandler>) -> impl Responder {
    let conn = pool.get().expect("Problema al traer la base de datos");
   
      
    match web::block(move || {Post::create_post(conn, &item)}).await {
        Ok(data) => {
                                      

             HttpResponse::Ok().body(format!("{:?}", data))
        },
        Err(err) => HttpResponse::Ok().body(format!("Error al recibir data: {}", err))

    }

}


// Macro para establecer la aplicación del tipo Web, la misma debe ser asíncrona
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("db url variable no encontrada");
    let port = env::var("PORT").expect("la variable de entorno PORT no existe");
    let port: u16 = port.parse().unwrap();

    let connection = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::builder().build(connection).expect("No se pudo construir la Pool");


// Levantamos el servidor HTTP
    HttpServer::new(move || {

    // Exponemos los endpoints que indiquemos

    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

    App::new()
    .service(index)
    .service(new_post)
    .service(get_post)
    .app_data(web::Data::new(pool.clone()))
    .app_data(web::Data::new(tera.clone()))
    
    // Indicamos el host y el puerto donde escuchará el servidor
        
    }).bind(("0.0.0.0", port)).unwrap().run().await

}

