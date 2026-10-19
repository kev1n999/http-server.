use crate::{
    server::handler::functions::{calc, home, sum},
    types::{
        headers::{
            content_type::ContentType,
            method::{AllowedMethods, Method},
            route::Route,
        },
        server::r#static::StaticFile,
    },
};

pub fn routes() -> Vec<Route> {
    vec![
        Route::new(AllowedMethods::Unique(Method::Get), "/".to_string(), home),
        Route::new(
            AllowedMethods::Unique(Method::Get),
            "/calc".to_string(),
            calc,
        ),
        Route::new(
            AllowedMethods::Unique(Method::Post),
            "/sum".to_string(),
            sum,
        ),
    ]
}

pub fn static_files() -> Vec<StaticFile> {
    vec![
        StaticFile::new("/js/calc.js".to_string(), ContentType::JavaScript),
        StaticFile::new("/css/calc.css".to_string(), ContentType::Css),
    ]
}
