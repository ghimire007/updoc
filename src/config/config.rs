use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::env;

lazy_static! {
    static ref ENV_VARS: HashMap<String, String> = {
        dotenv().ok();
        env::vars().collect()
    };
}

pub fn get_env(parameter: &str) -> String {
    ENV_VARS
        .get(parameter)
        .map(|s| s.to_string())
        .unwrap_or_else(|| panic!("{} is not defined in the environment.", parameter))
}
