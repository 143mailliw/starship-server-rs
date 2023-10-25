use crate::errors;
use async_graphql::Error;

pub async fn create_component(
    component: &str,
    _planet: String,
    _owner: String,
) -> Result<String, Error> {
    match component {
        "dummy" => Ok("dummy".to_string()),
        _ => Err(errors::create_user_input_error(
            "That component type isn't valid.",
            "INVALID_TYPE",
        )),
    }
}

pub async fn delete_component(component: &str, _id: String) -> Result<bool, Error> {
    match component {
        "dummy" => Ok(true),
        _ => Err(errors::create_internal_server_error(
            None,
            "UNSUPPORTED_COMPONENT",
        )),
    }
}
