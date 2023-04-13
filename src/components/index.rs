use crate::errors::create_user_input_error;
use async_graphql::Error;

pub fn create_component(component: &str, _planet: String, _owner: String) -> Result<String, Error> {
    match component {
        "dummy" => Ok("dummy".to_string()),
        _ => Err(create_user_input_error(
            "That component type isn't valid.",
            "INVALID_TYPE",
        )),
    }
}
