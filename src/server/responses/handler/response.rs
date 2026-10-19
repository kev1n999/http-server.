use crate::server::responses::default::method_not_allowed::response_method_not_allowed;
use crate::server::responses::default::response_404::response_404;
use crate::server::responses::handler::request::request_is_file;
use crate::server::routes::routes::{routes, static_files};

use crate::types::headers::method::Method;
use crate::types::server::context::Context;

pub fn response_by_route(ctx: &mut Context) {
    for route in routes() {
        if route.path != ctx.request.path {
            continue;
        }

        if !route.method.contains(&ctx.request.method) {
            response_method_not_allowed(ctx);
        }

        _ = (route.handler)(ctx);
        return;
    }
}

pub fn response_by_file(ctx: &mut Context) {
    for file in static_files() {
        if ctx.request.path == file.path && ctx.request.method == Method::Get {
            file.send(ctx).expect("An error ocurred to send the file!");
        }
    }
}

pub fn response_handler(ctx: &mut Context) -> Result<(), std::io::Error> {
    if request_is_file(&ctx.request) {
        response_by_file(ctx);
    } else {
        response_by_route(ctx);
    }

    response_404(ctx)
}
