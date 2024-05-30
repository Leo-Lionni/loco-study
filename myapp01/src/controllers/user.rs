use axum::debug_handler;
use loco_rs::prelude::*;
// 在lib.rs中， 无models 所以 使用crate；
// 这里就不需要再像 lib.rs 中那样去先声明 pub mod models 了。
use crate::{models::_entities::users, views::user::CurrentResponse};

#[debug_handler]
async fn current(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    format::json(CurrentResponse::new(&user))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/user")
        .add("/current", get(current))
}
