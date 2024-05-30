#![allow(clippy::unused_async)]
use loco_rs::prelude::*;

use crate::views;

/// Renders the dashboard liu page
///
/// # Errors
///
/// This function will return an error if render fails
pub async fn render_index(ViewEngine(v): ViewEngine<TeraView>) -> Result<Response> {
    // fisrt in views , and then in  controller
    views::liu::index(&v)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("liu")
        .add("/", get(render_index))
}