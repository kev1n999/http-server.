use crate::types::{
    headers::{content_type::ContentType, status_code::StatusCode},
    server::{context::Context, response::Response},
};

pub fn response_method_not_allowed(ctx: &mut Context) -> Result<(), std::io::Error> {
    let content = format!(
        r#"
        <h1><br>{}</br>Method not allowed!</h1>
    "#,
        ctx.request.method.to_string()
    );

    let response = Response::new(
        StatusCode::MethodNotAllowed,
        ContentType::Html,
        content.as_str(),
    );

    ctx.send(response)
}
