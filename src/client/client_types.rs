use std::fmt::Debug;

use super::VonageJwt;

#[derive(Debug, Clone)]
pub enum VonageRegion {
    US,
    EU,
    AP,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct TokenGenerator {
    app_id: String,
    private_key: Sensitive<String>,
    expiry: usize,
}

impl TokenGenerator {
    pub fn new(app_id: String, private_key: PrivateKey, expiry: Option<usize>) -> Self {
        log::trace!("Creating TokenGenerator for app_id: {}", app_id);
        TokenGenerator {
            app_id,
            private_key,
            expiry: expiry.unwrap_or(3600),
        }
    }

    pub(crate) fn generate_token(&self) -> Result<(Token, usize), jsonwebtoken::errors::Error> {
        VonageJwt::new(self.app_id.clone())
            .set_exp(self.expiry)
            .generate(self.private_key.clone())
    }
}

pub type PrivateKey = Sensitive<String>;
pub type Token = Sensitive<String>;

pub trait SensitiveUnwrap<T> {
    fn unwrap(self) -> T;
}

#[derive(Clone, PartialEq, Eq)]
pub struct Sensitive<T>(T);

impl<T> Sensitive<T> {
    pub fn new(value: T) -> Self {
        Sensitive(value)
    }
}

impl From<String> for Sensitive<String> {
    fn from(value: String) -> Self {
        Sensitive(value)
    }
}

impl From<&str> for Sensitive<String> {
    fn from(value: &str) -> Self {
        Sensitive(value.to_string())
    }
}

impl<T> SensitiveUnwrap<T> for Sensitive<T> {
    fn unwrap(self) -> T {
        self.0
    }
}

impl<T: Debug> Debug for Sensitive<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if cfg!(debug_assertions) {
            f.debug_tuple("Sensitive").field(&self.0).finish()
        } else {
            f.debug_tuple("Sensitive").field(&"<********>").finish()
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Sensitive<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if cfg!(debug_assertions) {
            write!(f, "Sensitive<{}>", self.0)
        } else {
            write!(f, "<********>")
        }
    }
}
