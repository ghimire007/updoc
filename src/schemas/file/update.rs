use serde::{Deserialize,Serialize};
use validator::{Validate, ValidationError};


#[derive(Debug,Serialize,Deserialize,Validate)]
pub struct UpdateFileRequest {

    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    #[validate(custom = "validate_name")]
    pub filename:Option<Option<String>>,

    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub content: Option<Option<String>>
}

fn validate_name(value: &str) -> Result<(), ValidationError> {
    if value.is_empty() {
        return Err(ValidationError::new("name cant be empty"));
    }
    Ok(())
}