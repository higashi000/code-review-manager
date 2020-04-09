use actix_web::{web, App, HttpServer};
use code_review_manager::server::make_project;
use code_review_manager::server::post_review;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            //.data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/post_review").route(web::post().to(post_review::catch_req)))
            .service(web::resource("/make_project").route(web::post().to(make_project::catch_req)))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
}
