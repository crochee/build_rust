use actix_web::{post, web, Responder};
use serde::Deserialize;

use crate::err::ResponseError;
use crate::service::impl_plugin;

#[post("/build/plugin/{name}")]
pub async fn build_plugin(
    name: web::Path<String>,
    plugin_content: web::Json<PluginContent>,
) -> impl Responder {
    let plugin_boxed_raw = impl_plugin::load_plugin(name.0,
                                                    plugin_content.id,
                                                    plugin_content.path.to_string(),
                                                    plugin_content.content.to_string());
    let plugin_impl = unsafe {
        Box::from_raw(plugin_boxed_raw)
    };
    plugin_impl.setup();
    plugin_impl.run();
    plugin_impl.teardown();
    web::Json(ResponseError::new(500, String::from("recovery")))
}

#[derive(Deserialize)]
pub struct PluginContent {
    pub id: usize,
    pub path: String,
    pub content: String,
}