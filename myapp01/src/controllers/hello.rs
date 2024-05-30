#![allow(clippy::unused_async)]
use loco_rs::prelude::*;

use crate::views;

/// Renders the dashboard home page
///
/// # Errors
///
/// This function will return an error if render fails
pub async fn render_index(ViewEngine(v): ViewEngine<TeraView>) -> Result<Response> {
    // fisrt in views , and then in  controller
    views::hello::index(&v)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("hello")
        .add("/", get(render_index))
}