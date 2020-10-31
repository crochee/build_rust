use crate::err::ResponseError;
use actix_web::{post, web, HttpResponse};
use serde::Deserialize;

#[post("/build/plugin/{name}")]
pub async fn build_plugin(
    name: web::Path<String>,
    plugin_content: web::Json<PluginContent>,
) -> HttpResponse {
    println!("{},{:#?}", name, plugin_content);
    let err = ResponseError::new(500, String::from("recovery"));
    HttpResponse::from_error(err)
}

#[derive(Debug, Deserialize)]
pub struct PluginContent {
    pub id: usize,
    pub path: String,
    pub content: String,
}
