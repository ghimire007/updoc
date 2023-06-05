use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub enum Data {
    Vec(Vec<String>),
    HashMap,
}

#[derive(Debug, Serialize)]
pub struct SucessResponse {
    pub message: String,
    pub data: Option<Data>,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub message: String,
    pub token: String,
}
