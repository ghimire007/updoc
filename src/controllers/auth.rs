use crate::config::config::get_env;
use crate::config::db::DB;
use crate::config::jwt::create_token;
use crate::error::AppError;
use crate::models::users::{self, Entity as User,Model,ActiveModel};
use crate::schemas::auth::{login::LoginRequest, signup::SignupRequest};
use crate::schemas::response::{SucessResponse, TokenResponse};
use axum::{Json, Extension};
use bcrypt::{hash, verify, DEFAULT_COST};
use sea_orm::{entity::*, query::*, DbBackend};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::result::Result;
use std::time::Duration;
use validator::Validate;

//use serde_json::{json,Value};

pub async fn signup_user(request: Json<SignupRequest>) -> Result<Json<TokenResponse>, AppError> {
    request
        .validate()
        .map_err(|err| AppError::ValidationError(err.to_string()))?;

    let mut prev_user = User::find()
        .filter(users::Column::Username.eq(request.username.to_owned()))
        .one(&*DB)
        .await
        .map_err(|error| AppError::InternalServerError)?;

    if prev_user.is_some() {
        return Err(AppError::BadRequestError(
            "user with provided username already exists".to_owned(),
        ));
    }

    let mut new_user = users::ActiveModel {
        username: Set(request.username.to_owned()),
        password: Set(hash(request.password.to_owned(), DEFAULT_COST).unwrap()),
        ..Default::default()
    };

    //new_user=new_user.save(&*DB).await.unwrap();

    let inserted_user = new_user.insert(&*DB).await.map_err(|error| AppError::InternalServerError)
    .unwrap();

    //let user =User::insert(new_user).exec(&*DB).await.unwrap();
    //println!("{:?}",new_user);

    Ok(Json(TokenResponse {
        message: "success".to_string(),
        token: create_token(inserted_user.id),
    }))
}

pub async fn login_user(request: Json<LoginRequest>) -> Result<Json<TokenResponse>, AppError> {
    let mut user = User::find()
        .filter(users::Column::Username.eq(request.username.to_owned()))
        .one(&*DB)
        .await
        .map_err(|error| AppError::InternalServerError)?;

    if user.is_none() {
        return Err(AppError::BadRequestError(
            "Either username or password is wrong".to_owned(),
        ));
    }
    let user = user.unwrap();
    let valid =
        verify(&request.password, &user.password).map_err(|error| AppError::InternalServerError)?;
    if !valid {
        return Err(AppError::BadRequestError(
            "Either username or password is wrong".to_owned(),
        ));
    }

    Ok(Json(TokenResponse {
        message: "success".to_string(),
        token: create_token(user.id),
    }))
}

pub async fn test_user(Extension(user):Extension<Model>,request: Json<LoginRequest>) -> Result<Json<TokenResponse>, AppError> {
    println!("{:?}",user);
    println!("{:?}",request.username);
    Ok(Json(TokenResponse {
        message: "success".to_string(),
        token: "jsjgbjvjkdbvdjvbdj".to_string(),
    }))




}
