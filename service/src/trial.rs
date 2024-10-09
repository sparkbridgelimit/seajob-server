use crate::activate::ActivateService;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use seajob_common::auth::validate_token;
use seajob_dto::req::activate::{ConsumeActivateCodeReq, CreateActivateCodeReq};
use seajob_dto::req::auth::SignUpRequest;
use crate::auth;
use crate::err::ServiceError;

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountInfo {
    pub username: String,
    pub password: String,
}

pub struct TrialAccount;

impl TrialAccount {
    // 创建测试账号
    pub async fn create(days: i64) ->  Result<AccountInfo, ServiceError>  {
        // 1. 创建账号

        // 生成4位随机数
        let mut rng = thread_rng();
        let random_number: u16 = rng.gen_range(1000..9999); // 生成1000-9999之间的随机数

        // 生成账号名称
        let username = format!("user_{:04}", random_number);

        // 生成随机密码
        let password: String = "password".to_string();

        // 调用注册服务创建账号
        let sign_up_response = auth::sign_up(SignUpRequest {
            username: username.clone(),
            password: password.clone(),
        })
        .await.map_err(|e| ServiceError::SystemError(format!("注册失败: {:?}", e)))?;


        let user_id = match validate_token(&sign_up_response.token) {
            Ok(data) => data.claims.user_id,
            Err(e) => return Err(ServiceError::SystemError(format!("解码 token 失败: {:?}", e))),
        };

        // 2. 创建激活码
        let code_req = CreateActivateCodeReq { biz_code: "seajob|user|try".to_string(), duration: days };
        let code = match ActivateService::create(code_req).await {
            Ok(code_model) => code_model.code,
            Err(e) => return Err(ServiceError::SystemError(format!("激活码创建失败: {:?}", e))),
        };

        // 3. 消费激活码
        let consume_req = ConsumeActivateCodeReq {
            code: code.clone(),
        };

        match ActivateService::consume(user_id, consume_req).await {
            Ok(_) => {} // 消费激活码成功
            Err(e) => return Err(ServiceError::SystemError(format!("激活码消费失败: {:?}", e))),
        };

        // 返回账号信息
        Ok(AccountInfo {
            username,
            password,
        })
    }
}