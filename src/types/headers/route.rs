use crate::types::{headers::method::AllowedMethods, server::handler::Handler};

#[derive(Debug)]
pub struct Route {
    pub method: AllowedMethods,
    pub path: String,
    pub handler: Handler,
}

impl Route {
    pub fn new(method: AllowedMethods, path: String, handler: Handler) -> Self {
        Self {
            method,
            path,
            handler,
        }
    }
}
