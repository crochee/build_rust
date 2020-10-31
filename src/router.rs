use actix_web::web::{ServiceConfig, scope};

use crate::controller::build;

pub fn router(cfg: &mut ServiceConfig) {
    cfg
        .service(
            scope("/api/v1")
                .service(build::build_plugin)
        );
}