use crate::config::config::get_env;
use crate::config::db::DB;
use crate::config::jwt::create_token;
use crate::error::AppError;
use crate::models::file::{self, Entity as File,Model,ActiveModel};
use crate::models::users::{Model as UserModel};
use crate::schemas::file::{create::CreateFileRequest,update::UpdateFileRequest};
use crate::schemas::response::{SucessResponse, TokenResponse};
use axum::{Json, Extension,extract::{Path, State},};
use bcrypt::{hash, verify, DEFAULT_COST};
use sea_orm::{entity::*, query::*, DbBackend};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::result::Result;
use std::time::Duration;
use validator::Validate;
use uuid::Uuid;


pub async fn create_file(Extension(user):Extension<UserModel>,request: Json<CreateFileRequest>) -> 
Result<Json<SucessResponse>, AppError> {

    let mut new_file = ActiveModel {
        filename: Set(request.filename.to_owned()),
        content:  Set(request.content.to_owned()),
        user_id:Set(user.id),
        ..Default::default()
    };
    
    let inserted_user = new_file.insert(&*DB).await.map_err(|error| AppError::InternalServerError)
    .unwrap();
    Ok(Json(SucessResponse {
        message: "file added sucessfully".to_string(),
        data: None,
    }))




}

pub async fn update_file(
Path(id): Path<i64>,
Extension(user):Extension<UserModel>,
Json(request): Json<UpdateFileRequest>) -> 
Result<Json<SucessResponse>, AppError> {

request
.validate()
.map_err(|err| AppError::ValidationError(err.to_string()))?;

let mut cur_file=File::find_by_id(id)
   .one(&*DB).await.
    map_err(|error| AppError::InternalServerError)?
    .ok_or( AppError::NotFound("File not found".to_owned()))?;

if (user.id != cur_file.user_id){
    return Err(AppError::Forbidden);
}

let mut cur_file=cur_file.into_active_model();

if let Some(filename)=request.filename{
    if(filename.is_none()){
       return  Err(AppError::ValidationError("name cant be empty".to_owned()));
    }
    cur_file.filename=Set(
        filename.unwrap()
       
    )
    //cur_file.filename=Set(filename)
}

if let Some(content)=request.content{
    cur_file.content=Set(content)
}

cur_file.save(&*DB).await.map_err(|error| {
    AppError::Forbidden
});

Ok(Json(SucessResponse {
    message: "file added sucessfully".to_string(),
    data: None,
}))
}



pub async fn delete_file(
    Path(id): Path<i64>,
    Extension(user):Extension<UserModel>,) -> 
    Result<Json<SucessResponse>, AppError> {
    
    let mut cur_file=File::find_by_id(id)
       .one(&*DB).await.
        map_err(|error| AppError::InternalServerError)?
        .ok_or( AppError::NotFound("File not found".to_owned()))?;
    
    if (user.id != cur_file.user_id){
        return Err(AppError::Forbidden);
    }
    File::delete(
        cur_file.into_active_model()
    ).exec(&*DB).await.map_err(|error| AppError::InternalServerError)?;
    
    Ok(Json(SucessResponse {
        message: "file deleted sucessfully".to_string(),
        data: None,
    }))
    }
    