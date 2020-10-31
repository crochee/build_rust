use actix_web::{post, web, Responder};
use serde::Serialize;
use crate::err::ResponseError;

#[post("/build/plugin/{name}")]
pub async fn build_plugin(
    name: web::Path<String>,
    plugin_content: web::Json<PluginContent>,
) -> impl Responder {
    web::Json(ResponseError::new(500, String::from("recovery")))
}

#[derive(Serialize)]
struct PluginContent {
    id:usize,
    path:String,
    content:String,
}