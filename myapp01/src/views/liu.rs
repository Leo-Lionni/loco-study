use loco_rs::prelude::*;
use serde_json::json;

/// liu view
///
/// # Errors
///
/// This function will return an error if render fails
pub fn index(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "liu/liu.html", json!({}))
}