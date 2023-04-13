use super::{component, dummy};
use crate::errors::create_user_input_error;
use async_graphql::Error;

fn find_component(name: &str) -> Result<impl component::Trait, Error> {
    match name {
        "dummy" => Ok(dummy::Component::default()),
        _ => Err(create_user_input_error(
            "That component type isn't valid.",
            "INVALID_TYPE",
        )),
    }
}
