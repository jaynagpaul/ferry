use failure::Error;
use provider::Provider;

/// Params is used to pass data to sessions for authorization. An existing
/// implementation, and the one most likely to be used, is `url.Values`.
pub trait Params {
    fn get(&self, &'static str) -> &'static str;
}

/// Session needs to be implemented as part of the provider package.
/// It will be marshaled and persisted between requests to "tie"
/// the start and the end of the authorization process with a
/// 3rd party provider.
pub trait Session {
    // auth_url returns the URL for the authentication end-point for the provider.
    fn auth_url(&self) -> String;
    // Marshal generates a string representation of the Session for storing between requests.
    fn marshal(&self) -> String;
    // Authorize should validate the data from the provider and return an access token
    // that can be stored for later access to the provider.
    fn authorize(&self, &Provider, &Params) -> Result<String, Error>;
}
