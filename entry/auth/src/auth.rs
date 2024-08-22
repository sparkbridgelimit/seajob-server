use actix_web::{get, Error, HttpResponse, post, web};
use log::error;
use seajob_common::auth::Authenticate;
use seajob_common::response::ApiResponse;
use seajob_dto::req::auth::{SignInPayload, SignUpRequest};
use seajob_service::auth;

#[get("/check")]
async fn check(_user: Authenticate) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .append_header(("x-user-id", _user.user_id))
        .append_header(("tenant_id", "seajob"))
        .json(ApiResponse::success_only()))
}

#[post("/sign_up")]
async fn sign_up(json: web::Json<SignUpRequest>) -> Result<HttpResponse, Error> {
    match auth::sign_up(json.into_inner()).await {
        Ok(data) => {
            let response = ApiResponse::success(data);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            error!("Failed to sign up: {:?}", e);
            let error_response = ApiResponse::fail();
            Ok(HttpResponse::Ok().json(error_response))
        }
    }
}

#[post("/sign_in")]
async fn sign_in(json: web::Json<SignInPayload>) -> Result<HttpResponse, Error> {
    match auth::sign_in(json.into_inner()).await {
        Ok(res) => {
            let response = ApiResponse::success(res);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            error!("Failed to sign in: {:?}", e);
            let error_response = ApiResponse::fail();
            Ok(HttpResponse::Ok().json(error_response))
        }
    }
}

#[post("/sign_out")]
async fn sign_out(user: Authenticate) -> Result<HttpResponse, Error> {
        match auth::sign_out(user.user_id).await {
        Ok(_) => {
            let response = ApiResponse::success_only();
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            error!("Failed to sign in: {:?}", e);
            let error_response = ApiResponse::fail();
            Ok(HttpResponse::Ok().json(error_response))
        }
    }
}