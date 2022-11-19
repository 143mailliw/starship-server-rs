use async_graphql::{Error, ErrorExtensionValues};

pub fn create_user_input_error(message: &str, code: &str) -> Error {
    let mut extensions = ErrorExtensionValues::default();

    extensions.set("type", "INVALID_USER_INPUT");
    extensions.set("code", code);

    Error {
        message: message.into(),
        source: None,
        extensions: Some(extensions),
    }
}

pub fn create_not_found_error() -> Error {
    let mut extensions = ErrorExtensionValues::default();

    extensions.set("type", "NOT_FOUND");
    extensions.set("code", "NOT_FOUND");

    Error {
        message: "Not found.".to_string(),
        source: None,
        extensions: Some(extensions),
    }
}

pub fn create_internal_server_error(message: Option<&str>, code: &str) -> Error {
    let mut extensions = ErrorExtensionValues::default();

    extensions.set("type", "INTERNAL_SERVER_ERROR");
    extensions.set("code", code);

    Error {
        message: message.unwrap_or("Internal server error.").to_string(),
        source: None,
        extensions: Some(extensions),
    }
}
