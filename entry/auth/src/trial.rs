use actix_web::{post, web, Error, HttpResponse};
use log::error;
use seajob_common::response::ApiResponse;
use seajob_dto::req::trail::CreateTrailAccountReq;
use seajob_service::trial::TrialAccount;

/// 查询用户的会员资格
#[post("/trail/create")]
pub async fn create(req: web::Json<CreateTrailAccountReq>) -> Result<HttpResponse, Error> {
    // 查询用户是否有membership记录
    match TrialAccount::create(req.days).await {
        Ok(data) => {
            let response = ApiResponse::success(data);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            error!("Failed to get membership info: {:?}", e);
            let response = ApiResponse::success_only();
            Ok(HttpResponse::Ok().json(response))
        }
    }
}