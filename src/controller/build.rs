use crate::err::ResponseError;
use actix_web::{post, web, Responder};
use serde::Deserialize;

#[post("/build/plugin/{name}")]
pub async fn build_plugin(
    name: web::Path<String>,
    plugin_content: web::Json<PluginContent>,
) -> impl Responder {
    println!("{},{:#?}", name, plugin_content);
    web::Json(ResponseError::new(500, String::from("recovery")))
}

#[derive(Debug, Deserialize)]
pub struct PluginContent {
    pub id: usize,
    pub path: String,
    pub content: String,
}
