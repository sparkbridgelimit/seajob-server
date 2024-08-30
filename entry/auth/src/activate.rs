use actix_web::{post, web, Error, HttpResponse};
use log::error;
use seajob_common::auth::{Authenticate, UserRole};
use seajob_common::response::ApiResponse;
use seajob_dto::req::activate::{ConsumeActivateCodeReq, CreateActivateCodeReq};
use seajob_service::activate::ActivateService;

#[post("/activate/create")]
pub async fn create(json: web::Json<CreateActivateCodeReq>) -> Result<HttpResponse, Error> {
    match ActivateService::create(json.into_inner()).await {
        Ok(data) => {
            let response = ApiResponse::success(data);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            error!("Failed to create activate code: {:?}", e);
            let error_response = ApiResponse::fail();
            Ok(HttpResponse::Ok().json(error_response))
        }
    }
}

#[post("/activate/consume")]
pub async fn consume(user: Authenticate<UserRole>, json: web::Json<ConsumeActivateCodeReq>) -> Result<HttpResponse, Error> {
    match ActivateService::consume(user.user_id, json.into_inner()).await {
        Ok(data) => {
            let response = ApiResponse::success(data);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            error!("Failed to consume code: {:?}", e);
            let error_response = ApiResponse::fail();
            Ok(HttpResponse::Ok().json(error_response))
        }
    }
}