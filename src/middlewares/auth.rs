use axum::{
    Router,
    http::{StatusCode,Request,header},
    middleware::{self, Next},
    response::Response,
    routing::get,
    Json, body::Body
};
use sea_orm::EntityTrait;
use std::result::Result;
use crate::config::jwt::validate_token;
use crate::error::AppError;
use crate::schemas::response::{SucessResponse};
use crate::models::users::{self, Entity as User};
use crate::config::db::DB;
use axum::response::IntoResponse;



pub async fn auth<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, AppError> {
    let token = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| {
            let parts: Vec<&str> = header.split(' ').collect();
            if parts.len() == 2 && parts[0] == "Bearer" {
                Some(parts[1])
            } else {
                None
            }
        });;

    let Some(token) = token else { return Err(AppError::UnAuthorized(
        "Invalid Token".to_owned(),
    ))};

    let token_resp =validate_token(token.to_owned())
    .map_err(|error| AppError::UnAuthorized("Invalid token".to_owned()))?;
            
    let mut user= User::find_by_id(token_resp.claims.id)
            .one(&*DB)
            .await
            .map_err(|error| AppError::InternalServerError)?;
    
    let user=user.ok_or_else(|| AppError::UnAuthorized("Invalid token".to_owned()))?;
    req.extensions_mut().insert(user);
    Ok(next.run(req).await)

}

// pub async fn auth<B,D>(
//     mut request: Request<B>,
//     next: Next<B>,
// )->
// // Result<D, AppError>
// // where D: IntoResponse
  
// Result<Response<Body>, AppError>
// //Result<Json<SucessResponse>, AppError>
//  {
//     let mut token=request.headers()
//     .get(header::AUTHORIZATION)
//     .and_then(|val| val.to_str().ok());

//     let Some(token) = token else { return Err(AppError::UnAuthorized(
//         "Invalid".to_owned(),
//     ))};
//     let token_resp =validate_token(token.to_owned())
//             .map_err(|error| AppError::InternalServerError)?;
            
//     let mut req_user= User::find_by_id(token_resp.claims.id)
//             .one(&*DB)
//             .await
//             .map_err(|error| AppError::InternalServerError)?;
            
//     let req_user=req_user.ok_or_else(|| AppError::UnAuthorized("Invalid token".to_owned()))?;
    
        
//     request.extensions_mut().insert(req_user);
//     Ok(
//         next.run(request).await
//     )

    
//     }



    //user_from_token=validate_token(auth.token());

