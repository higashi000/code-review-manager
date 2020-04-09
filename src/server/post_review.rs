use crate::data_model;
use crate::database;
use actix_web::{web, HttpResponse};

pub async fn catch_req(obj: web::Json<data_model::post_review::ReviewData>) -> HttpResponse {
    //    let project_name: &str = &obj.project_name;
    //    database::post_preview::save_database(project_name, &obj);

    HttpResponse::Ok().json("{\"status\":\"true\"}")
}
