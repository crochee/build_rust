use actix_web::{post, web, Responder};
use serde::Deserialize;
use libloading as lib;
use crochee_plugin::Plugin;

use crate::err::ResponseError;
use crate::service::impl_plugin;

#[post("/build/plugin/{name}")]
pub async fn build_plugin(
    name: web::Path<String>,
    plugin_content: web::Json<PluginContent>,
) -> impl Responder {
    // let plugin_boxed_raw = call_dynamic(name.0,
    //                                     plugin_content.id,
    //                                     plugin_content.path.to_string(),
    //                                     plugin_content.content.to_string());
    let plugin_impl = impl_plugin::load_plugin(name.0,
                                               plugin_content.id,
                                               plugin_content.path.to_string(),
                                               plugin_content.content.to_string());
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

fn call_dynamic(name: String, plu: PluginContent) -> lib::Result<impl Plugin> {
    let lib = lib::Library::new("./service/plugin_impl/target/debug/plugin_impl.dll")?;
    unsafe {
        let plugin_func: lib::Symbol<unsafe fn(String, usize, String, String) -> impl Plugin>
            = lib.get(b"load_plugin")?;
        let plugin_impl = plugin_func(name, plu.id, plu.path, plu.content);
        Ok(plugin_impl)
    }
}