extern crate chrono;
extern crate failure;
extern crate oauth2;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod provider;
pub mod session;
pub mod user;

pub struct Harbor {
    pub providers: Vec<&'static provider::Provider>,
}

impl Harbor {
    pub fn new(providers: Vec<&'static provider::Provider>) -> Harbor {
        Harbor {
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
    //     let mut harbor = Harbor::new(vec![Provider{}])
    // }
}
