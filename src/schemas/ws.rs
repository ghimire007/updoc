use serde::{Serialize,Deserialize};

#[derive(Debug, Deserialize,Serialize,Clone)]
pub struct Wmsg {
    pub user_id: String,
    pub key: String,
    pub content:String,
    pub file_id:String
}

