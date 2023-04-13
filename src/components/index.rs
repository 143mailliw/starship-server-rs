use crate::errors::create_user_input_error;
use async_graphql::Error;

pub fn create_component(name: &str, planet: String) -> Result<String, Error> {
    match name {
        "dummy" => Ok("dummy".to_string()),
        _ => Err(create_user_input_error(
            "That component type isn't valid.",
            "INVALID_TYPE",
        )),
    }
}
