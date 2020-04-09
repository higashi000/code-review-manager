use crate::data_model;
use crate::database;
use actix_web::{web, HttpResponse};

/// extract `data_model::make_project::ProjectData` using serde
pub async fn catch_req(obj: web::Json<data_model::make_project::ProjectData>) -> HttpResponse {
    database::make_project::make_project(&obj);

    HttpResponse::Ok().json("{\"status\":\"true\"}")
}
