use axum::{extract::Request, middleware::Next, response::Response};
use http::{header, HeaderValue};

use crate::users::AuthSession;

const USER_EMAIL_COOKIE: &str = "user_email";

pub async fn set_user_info(request: Request, next: Next) -> Response {
    let mut user_email: Option<String> = None;

    let session = request.extensions().get::<AuthSession>();

    if let Some(auth_session) = session {
        if let Some(user) = &auth_session.user {
            user_email = Some(user.email.clone());
        };
    }

    let mut response = next.run(request).await;

    if let Some(user_email) = user_email {
        response.headers_mut().append(
            header::SET_COOKIE,
            HeaderValue::from_str(&format!("{USER_EMAIL_COOKIE}={user_email}")).unwrap(),
        );
    }

    response
}
