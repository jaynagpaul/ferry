use failure::Error;
use oauth2::Token;
use session::Session;
use user::User;
/// Provider needs to be implemented for each 3rd party authentication provider
/// e.g. Facebook, Twitter, etc...
pub trait Provider {
	fn name(&self) -> &'static str;
	fn begin_auth(&self, state: &'static str) -> Result<Box<Session>, Error>;
	fn unmarshal_session(&self, &'static str) -> Result<Box<Session>, Error>;
	fn fetch_user(&self, &Session) -> Result<User, Error>;
	fn refresh_token(&self, &'static str) -> Result<&'static Token, Error>; //Get new access token based on the refresh token
	fn refresh_token_available(&self) -> bool; //Refresh token is provided by auth provider or not
}
