use actix_web::{Error, error, HttpRequest};
use actix_web::dev::Payload;
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserContext {
    pub user_id: i64,
}

impl actix_web::FromRequest for UserContext {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();

        let result = req
            .headers()
            .get("x-user-id")
            .ok_or_else(|| error::ErrorUnauthorized("x-user-id Not Found"))
            .and_then(|header_value| {
                header_value
                    .to_str()
                    .map_err(|_| error::ErrorUnauthorized("Invalid x-user-id header"))
            })
            .and_then(|header_str| {
                header_str
                    .parse::<i64>()
                    .map_err(|_| error::ErrorUnauthorized("x-user-id must be a valid i64"))
            })
            .map(|user_id| UserContext { user_id });

        ready(result)
    }
}