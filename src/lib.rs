extern crate chrono;
extern crate failure;
extern crate oauth2;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod provider;
mod session;
mod user;

pub use provider::Provider;
pub use session::{Params, Session};
pub use user::User;

pub struct Ferry {
    pub providers: Vec<&'static provider::Provider>,
}

impl Ferry {
    pub fn new(providers: Vec<&'static provider::Provider>) -> Ferry {
        Ferry {
            providers: providers,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn add_provider() {
    //     let mut ferry = Ferry::new(vec![Provider{}])
    // }
}
