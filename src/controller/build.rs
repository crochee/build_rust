use actix_web::{post, web, Responder};
use serde::Serialize;
use std::collections::HashMap;

#[post("/build/plugin/{name}")]
pub async fn build_plugin(
    web::Path((name)): web::Path<(String)>,
    plugin_content: web::Json<PluginContent>,
) -> impl Responder {
    web::Json()
}

#[derive(Serialize)]
struct PluginContent {
    id:usize,
    path:String,
    content:String,
}