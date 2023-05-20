use web::server::{
    response,
    routing::{get, post},
    Router,
};

use crate::controller::AuthController;

pub mod auth {
    use super::*;

    //
    // NOTE: /auth/...
    //
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route(
                "/auth/login",
                get(AuthController::show_login_page).post(AuthController::login),
            )
            .route(
                "/auth/register",
                get(AuthController::show_register_page).post(AuthController::register),
            )
    }
}

pub mod message {
    //
    // NOTE: /message/...
    //
    pub fn routes() -> Router<AppState> {
        // code...
    }
}
