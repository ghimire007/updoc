use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Deserialize)]
pub struct SignupRequest {
    pub username: String,

    #[validate(custom = "validate_password")]
    pub password: String,
}

fn validate_password(value: &str) -> Result<(), ValidationError> {
    if value.len() < 8 {
        return Err(ValidationError::new("must be at least 8 characters long"));
    }
    println!("here validation called");

    let has_upper = value.chars().any(|c| c.is_ascii_uppercase());
    let has_lower = value.chars().any(|c| c.is_ascii_lowercase());
    let has_digit = value.chars().any(|c| c.is_ascii_digit());

    if has_upper && has_lower && has_digit {
        Ok(())
    } else {
        return Err(ValidationError::new(
            "must contain at least one uppercase letter, one lowercase letter, and one digit",
        ));
    }
}
