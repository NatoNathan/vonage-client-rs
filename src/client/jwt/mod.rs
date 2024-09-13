mod acl;
mod jti;
mod time;
use jsonwebtoken::Algorithm;
use log::info;
use serde::{Deserialize, Serialize};

// local imports
pub(crate) use jti::generate_jti;
pub(crate) use time::now_timestamp;
// local test imports
#[cfg(test)]
pub(crate) use jti::set_mock_jti;
#[cfg(test)]
pub(crate) use time::set_mock_time;

#[allow(unused_imports)]
pub use acl::{AclMethod, VonageAcl};

use crate::client_types::{Sensitive, SensitiveUnwrap, Token};

use super::client_types::PrivateKey;
const VONAGE_JWT_ALGORITHM: Algorithm = Algorithm::RS256;

/// Vonage JWT
/// This struct represents the claims of a Vonage JWT and is used to generate a JWT token for Vonage API requests
///
/// ## Example
/// ```rust
/// use vonage_client::client::jwt::VonageJwt;
/// let private_key = std::env::var("VONAGE_PRIVATE_KEY").unwrap();
/// let (token, exp) = VonageJwt::new("application_id".to_string()).generate(private_key)?;
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct VonageJwt {
    application_id: String,
    exp: usize,
    iat: usize,
    jti: String,
    nbf: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    acl: Option<VonageAcl>,
}
impl VonageJwt {
    /// Create a new Vonage JWT Claims object for an application token
    /// @param application_id The Vonage Application ID
    /// ```rust
    /// use vonage_client::client::jwt::VonageJwt;
    /// let claims = VonageJwt::new("application_id".to_string());
    /// ```
    pub fn new(application_id: String) -> Self {
        let now = now_timestamp();

        info!("Now: {}, exp: {}", now, now + 300);
        VonageJwt {
            application_id,
            exp: now + 300, // 5 minutes
            iat: now,
            jti: generate_jti(),
            nbf: now,
            sub: None,
            acl: None,
        }
    }

    /// Create a new Vonage JWT Claims object for a user token with a sub claim and optional expiry
    /// @param application_id The Vonage Application ID
    /// @param sub The subject of the token
    /// @param exp The expiry time of the token in seconds
    /// ## Example
    /// ```rust
    /// use vonage_client::client::jwt::VonageJwt;
    /// let claims = VonageJwt::new_with_sub("application_id".to_string(), "sub".to_string(), Some(300));
    /// ```
    pub fn new_with_sub(application_id: String, sub: String, exp: Option<usize>) -> Self {
        let now = now_timestamp();
        VonageJwt {
            application_id,
            sub: Some(sub),
            exp: now + exp.unwrap_or(300), // 5 minutes
            iat: now,
            jti: generate_jti(),
            nbf: now,
            acl: None,
        }
    }

    /// Set the ACL for the JWT Claims
    /// @param acl The ACL to set
    ///
    /// ```rust
    /// use vonage_client::client::jwt::{VonageJwt, VonageACL, AclMethod};
    /// let mut acl = VonageACL::new();
    /// acl.add_path("/v1/users/**".to_string(), Some(vec![AclMethod::GET, AclMethod::POST]));
    /// let mut claims = VonageJwt::new("application_id".to_string());
    /// claims.set_acl(acl);
    /// ```
    pub fn set_acl(&mut self, acl: VonageAcl) -> &mut Self {
        self.acl = Some(acl);
        self
    }

    /// Set the expiry time for the JWT Claims
    /// @param exp The expiry time in seconds
    /// ```rust
    /// use vonage_client::client::jwt::VonageJwt;
    /// let mut claims = VonageJwt::new("application_id".to_string());
    /// claims.set_exp(300);
    /// ```
    pub fn set_exp(&mut self, exp: usize) -> &mut Self {
        self.exp = self.iat + exp;
        self
    }

    /// Generate the JWT token for Vonage API requests
    /// @param private_key The private key to sign the token
    /// @return The JWT token and expiry time in seconds
    /// ```rust
    /// use vonage_client::client::jwt::VonageJwt;
    /// let private_key = std::env::var("VONAGE_PRIVATE_KEY").unwrap();
    /// let claims = VonageJwt::new("application_id".to_string());
    /// let (token, exp) = claims.generate(private_key)?;
    /// log::info!("sentivate token: {}, unwrapped Token: {}, expiry: {}", token, token.0, exp);
    /// ```
    pub fn generate(
        &self,
        private_key: PrivateKey,
    ) -> Result<(Token, usize), jsonwebtoken::errors::Error> {
        log::trace!(
            "Generating JWT token, claims: {:?}, private_key: {:?}",
            self,
            private_key
        );
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::new(VONAGE_JWT_ALGORITHM),
            self,
            &jsonwebtoken::EncodingKey::from_rsa_pem(private_key.unwrap().as_bytes())?,
        )?;
        Ok((Sensitive::new(token), self.exp))
    }
}
