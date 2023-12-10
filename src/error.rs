use std::fmt::{Display, Formatter};

/// Errors returned by Apify API
#[derive(Debug, PartialEq)]
pub enum ApifyApiError {
    // The variant here is "type" and the param is "message"
    NotFound(String),
    // We don't have types for all statuses now so we just pass a message
    // TODO: Get rid of this after implementing all possibillitites
    RawError(String),
    MaxTimeoutRetriesReached(u8),
    MaxRateLimitRetriesReached(u8),
    MaxServerFailedRetriesReached(u8),
    /// Something is broken in the API or breaking change happened
    ApiFailure(String),
    // This is already validated in the client but if there is a breaking change in the API,
    // we want to also capture it from the server
    MissingToken,
}

impl Display for ApifyApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Apify API returned an error: {:?}", self)
    }
}

impl std::error::Error for ApifyApiError {}

/// Validation errors before sending the API call
#[derive(Debug, PartialEq)]
pub enum ClientValidationError {
    MissingToken,
    InvalidResourceIdOrName(String),
}

impl Display for ClientValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot send request, invalid configuration: {:?}", self)
    }
}

impl std::error::Error for ClientValidationError {}

/// Mother of all errors
/// Errors can either be returned by Apify API, happen at client validation or happen due to wrong JSON parsing logic.
/// Only few endpoints that include parsing JSON can return Parse error
#[derive(Debug)]
pub enum ApifyClientError {
    ApifyApi(ApifyApiError),
    Parse(serde_json::error::Error),
    ClientValidation(ClientValidationError),
}

impl Display for ApifyClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApifyClientError::ApifyApi(apify_api_error) => write!(f, "{}", apify_api_error),
            ApifyClientError::ClientValidation(client_validation_error) => write!(f, "{}", client_validation_error),
            ApifyClientError::Parse(parse_error) => write!(f, "JSON parsing failed, please fix your (de)serialization {}", parse_error),
        }
    }
}

impl std::error::Error for ApifyClientError {}

impl From<serde_json::error::Error> for ApifyClientError {
    fn from(e: serde_json::error::Error) -> Self {
        ApifyClientError::Parse(e)
    }
}

impl From<ApifyApiError> for ApifyClientError {
    fn from(e: ApifyApiError) -> Self {
        ApifyClientError::ApifyApi(e)
    }
}

impl From<ClientValidationError> for ApifyClientError {
    fn from(e: ClientValidationError) -> Self {
        ApifyClientError::ClientValidation(e)
    }
}