mod builder;
pub(crate) mod client_types;
pub(crate) mod jwt;

use builder::build_http_client;
use client_types::TokenGenerator;
use jwt::now_timestamp;

use reqwest::Response;
use url::Url;

pub(crate) use jwt::VonageJwt;

pub use builder::VonageClientBuilder;

pub use client_types::{PrivateKey, Sensitive, SensitiveUnwrap, Token, VonageRegion};

/// Vonage Client
/// This struct represents a Vonage Client and is used to make requests to the Vonage API
///
/// ## Example
/// ```rust
/// use vonage_client::client::{VonageClient, VonageRegion};
/// let client = VonageClient::builder()
///    .app_id("application_id".to_string())
///    .private_key(std::env::var("VONAGE_PRIVATE_KEY").unwrap())
///    .region(VonageRegion::EU)
///   .build()?;
///
/// ```
#[derive(Debug, Clone)]
pub struct VonageClient {
    client: reqwest::Client,
    base_url: Url,
    token_expiry: usize,
    token_refresh: Option<usize>,
    token_gen: TokenGenerator,
}

#[derive(Debug)]
pub enum VonageClientError {
    RequestError(reqwest::StatusCode, Response),
    ResponseParseError(reqwest::Error),
    RequestParseError(serde_json::Error),
    HttpClientError(reqwest::Error),
    TokenRefreshClientError(reqwest::Error),
    TokenRefreshError(jsonwebtoken::errors::Error),
}
impl VonageClient {
    pub fn builder() -> VonageClientBuilder {
        VonageClientBuilder::new()
    }

    /// Refresh the token if it is expired
    /// This will generate a new JWT token and update the Authorization header
    fn refresh_token(&mut self) -> Result<(), VonageClientError> {
        let now = now_timestamp();
        let build_token = &self.token_gen;
        // Refresh the token if it is 5 seconds before expiry and a refresh time is set otherwise allow the token to expire
        if let Some(token_refresh) = self.token_refresh {
            if now + 5 >= self.token_expiry - token_refresh {
                let (token, exp) = build_token.generate_token().map_err(|e| {
                    log::error!("Error generating token: {}", e);
                    VonageClientError::TokenRefreshError(e)
                })?;
                self.client = build_http_client(token.clone()).map_err(|e| {
                    log::error!("Error building reqwest client: {}", e);
                    VonageClientError::TokenRefreshClientError(e)
                })?;
                self.token_expiry = exp;
            }
        }
        Ok(())
    }

    /// Get Request
    /// This function makes a GET request to the Vonage API
    /// @param path The path to make the request to
    /// @return The response from the Vonage API
    #[inline]
    pub(crate) async fn get<R>(&mut self, path: &str) -> Result<R, VonageClientError>
    where
        R: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        log::trace!("Making GET request to {}", path);
        self.refresh_token()?;
        let res = self
            .client
            .get(self.base_url.join(path).unwrap())
            .send()
            .await
            .map(Self::trace_response("Get Response".to_string()))
            .map(Self::check_status_code)
            .map_err(Self::map_client_error)??;
        res.json::<R>()
            .await
            .map_err(Self::map_parse_error)
            .map(Self::trace_response("Get ResponseParsed".to_string()))
    }

    /// Post Request
    /// This function makes a POST request to the Vonage API
    /// @param path The path to make the request to
    /// @param body The body of the request
    /// @return The response from the Vonage API
    #[inline]
    pub(crate) async fn post<B, R>(&mut self, path: &str, body: B) -> Result<R, VonageClientError>
    where
        B: serde::Serialize,
        R: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        log::trace!("Making POST request to {}", path);
        self.refresh_token()?;
        let body = serde_json::to_string(&body).map_err(|err| {
            log::error!("Error serializing request body: {}", err);
            VonageClientError::RequestParseError(err)
        })?;
        let res = self
            .client
            .post(self.base_url.join(path).unwrap())
            .body(body)
            .send()
            .await
            .map(Self::trace_response("Post Response".to_string()))
            .map(Self::check_status_code)
            .map_err(Self::map_client_error)??;

        res.json::<R>()
            .await
            .map_err(Self::map_parse_error)
            .map(Self::trace_response("Post ResponseParsed".to_string()))
    }

    /// Put Request
    /// This function makes a PUT request to the Vonage API
    /// @param path The path to make the request to
    /// @param body The body of the request
    /// @return The response from the Vonage API
    #[inline]
    pub(crate) async fn put<B, R>(&mut self, path: &str, body: B) -> Result<R, VonageClientError>
    where
        B: serde::Serialize,
        R: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        log::trace!("Making PUT request to {}", path);
        self.refresh_token()?;
        let body = serde_json::to_string(&body).map_err(|err| {
            log::error!("Error serializing request body: {}", err);
            VonageClientError::RequestParseError(err)
        })?;
        let res = self
            .client
            .post(self.base_url.join(path).unwrap())
            .body(body)
            .send()
            .await
            .map(Self::trace_response("Put Response".to_string()))
            .map(Self::check_status_code)
            .map_err(Self::map_client_error)??;
        res.json::<R>()
            .await
            .map_err(Self::map_parse_error)
            .map(Self::trace_response("Put ResponseParsed".to_string()))
    }

    /// Patch Request
    /// This function makes a PATCH request to the Vonage API
    /// @param path The path to make the request to
    /// @param body The body of the request
    /// @return The response from the Vonage API
    #[inline]
    pub(crate) async fn patch<B, R>(&mut self, path: &str, body: B) -> Result<R, VonageClientError>
    where
        B: serde::Serialize,
        R: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        log::trace!("Making PATCH request to {}", path);
        self.refresh_token()?;
        let body = serde_json::to_string(&body).map_err(|err| {
            log::error!("Error serializing request body: {}", err);
            VonageClientError::RequestParseError(err)
        })?;
        let res = self
            .client
            .post(self.base_url.join(path).unwrap())
            .body(body)
            .send()
            .await
            .map(Self::trace_response("Patch Response".to_string()))
            .map(Self::check_status_code)
            .map_err(Self::map_client_error)??;
        res.json::<R>()
            .await
            .map_err(Self::map_parse_error)
            .map(Self::trace_response("Patch ResponseParsed".to_string()))
    }

    /// Delete Request
    /// This function makes a DELETE request to the Vonage API
    /// @param path The path to make the request to
    /// @return The response from the Vonage API
    /// @note This function does not return a response body
    #[inline]
    pub(crate) async fn delete(&mut self, path: &str) -> Result<(), VonageClientError> {
        log::trace!("Making DELETE request to {}", path);
        self.refresh_token()?;
        let _ = self
            .client
            .delete(self.base_url.join(path).unwrap())
            .send()
            .await
            .map(Self::trace_response("Delete Response".to_string()))
            .map(Self::check_status_code)
            .map_err(Self::map_client_error)?;

        log::trace!("Delete request successful");
        Ok(())
    }

    /// a check to ensure the status code of a response is in the 200 range otherwise return an error
    #[inline]
    pub(crate) fn check_status_code(res: Response) -> Result<reqwest::Response, VonageClientError> {
        match res.status() {
            code if code.as_u16() >= 200 && code.as_u16() < 300 => Ok(res),
            code => {
                log::error!("Error making request: {:?}", res);
                Err(VonageClientError::RequestError(code, res))
            }
        }
    }

    /// Map a reqwest error to a VonageClientError
    #[inline]
    pub(crate) fn map_client_error(e: reqwest::Error) -> VonageClientError {
        log::error!("Error making request: {}", e);
        VonageClientError::HttpClientError(e)
    }

    /// Map a reqwest error to a VonageClientError
    #[inline]
    pub(crate) fn map_parse_error(e: reqwest::Error) -> VonageClientError {
        log::error!("Error parsing response: {}", e);
        VonageClientError::ResponseParseError(e)
    }

    /// A curried function to log the response of a request with a message at debug level
    #[inline]
    pub(crate) fn debug_response<T>(msg: String) -> impl FnOnce(T) -> T
    where
        T: std::fmt::Debug,
    {
        move |res: T| {
            log::debug!("{}: {:?}", msg, res);
            res
        }
    }

    /// A curried function to log the response of a request with a message at trace level
    #[inline]
    pub(crate) fn trace_response<T>(msg: String) -> impl FnOnce(T) -> T
    where
        T: std::fmt::Debug,
    {
        move |res: T| {
            log::trace!("{}: {:?}", msg, res);
            res
        }
    }
}
