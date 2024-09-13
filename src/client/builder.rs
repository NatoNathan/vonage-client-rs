use reqwest::header::{HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use url::Url;

use crate::client_types::SensitiveUnwrap;

use super::client_types::{PrivateKey, Token, TokenGenerator, VonageRegion};
use super::VonageClient;

pub struct VonageClientBuilder {
    app_id: Option<String>,
    private_key: Option<PrivateKey>,
    region: Option<VonageRegion>,
    base_url: Option<String>,
    token_refresh: Option<usize>,
}

#[allow(dead_code)] // TODO: Remove this
#[derive(Debug)]
pub enum VonageClientBuilderError {
    MissingAppId,
    MissingPrivateKey,
    ErrorGeneratingToken(jsonwebtoken::errors::Error),
    ErrorParsingUrl(url::ParseError),
    ErrorBuildingHttpClient(reqwest::Error),
}

impl VonageClientBuilder {
    pub fn new() -> Self {
        log::trace!("Creating VonageClientBuilder");
        VonageClientBuilder {
            app_id: None,
            private_key: None,
            region: None,
            base_url: None,
            token_refresh: None,
        }
    }
    /// Set the Vonage Application ID
    /// @param app_id The Vonage Application ID
    pub fn app_id(mut self, app_id: String) -> Self {
        self.app_id = Some(app_id);
        self
    }

    /// Set the private key for the Vonage Application
    /// @param private_key The private key for the Vonage Application
    /// This is a secure version of the private key setter
    pub fn private_key(mut self, private_key: PrivateKey) -> Self {
        self.private_key = Some(private_key);
        self
    }

    /// Set the private key for the Vonage Application
    /// @param private_key The private key for the Vonage Application
    /// This is an unsecure version of the private key setter
    pub fn private_key_unsecure(mut self, private_key: String) -> Self {
        log::warn!("Using unsecure private key setter");
        self.private_key = Some(private_key.into());
        self
    }

    /// Set the Vonage Region
    /// @param region The Vonage Region
    pub fn region(mut self, region: VonageRegion) -> Self {
        self.region = Some(region);
        self
    }

    /// Set the base URL for the Vonage API
    /// This is useful if you are using a different Vonage API endpoint like a specific region or in testing
    /// @param base_url The base URL for the Vonage API
    pub fn base_url(mut self, base_url: String) -> Self {
        self.base_url = Some(base_url);
        self
    }

    /// Set the token refresh time
    /// This is the time in seconds before the token expires to refresh the token
    /// @param token_refresh The time in seconds before the token expires to refresh the token
    pub fn token_refresh(mut self, token_refresh: usize) -> Self {
        self.token_refresh = Some(token_refresh);
        self
    }

    pub fn build(self) -> Result<VonageClient, VonageClientBuilderError> {
        log::debug!("Building Vonage Client");
        let app_id = self.app_id.ok_or(VonageClientBuilderError::MissingAppId)?;
        let private_key = self
            .private_key
            .ok_or(VonageClientBuilderError::MissingPrivateKey)?;

        // Default to US region if not specified
        let base_url = self.base_url.unwrap_or(match self.region {
            Some(VonageRegion::EU) => "https://api-eu.vonage.com".into(),
            Some(VonageRegion::AP) => "https://api-ap.vonage.com".into(),
            Some(VonageRegion::US) | None => "https://api-us.vonage.com".into(),
        });
        let base_url = Url::parse(&base_url).map_err(|e| {
            log::error!("Error parsing base URL: {}", e);
            VonageClientBuilderError::ErrorParsingUrl(e)
        })?;
        log::trace!("Building Vonage Client with app_id: {}, private_key: {:?}, region: {:?}, base_url: {}, token_refresh: {:?}",
            app_id, private_key, self.region, base_url, self.token_refresh);

        let token_gen = TokenGenerator::new(app_id, private_key, self.token_refresh);

        let (token, exp) = token_gen.generate_token().map_err(|e| {
            log::error!("Error generating token: {}", e);
            VonageClientBuilderError::ErrorGeneratingToken(e)
        })?;

        let client = build_http_client(token.clone()).map_err(|e| {
            log::error!("Error building reqwest client: {}", e);
            VonageClientBuilderError::ErrorBuildingHttpClient(e)
        })?;

        log::debug!("Vonage Client built successfully");
        Ok(VonageClient {
            client,
            base_url,
            token_expiry: exp,
            token_gen,
            token_refresh: self.token_refresh,
        })
    }
}
/// Private function to build the HTTP client
pub(super) fn build_http_client(token: Token) -> Result<reqwest::Client, reqwest::Error> {
    log::debug!("Building HTTP client");
    let user_agent = format!("VonageServerClient,{}/Rust", env!("CARGO_PKG_VERSION"));
    let mut headers = reqwest::header::HeaderMap::new();
    let mut auth_header =
        HeaderValue::from_str(&format!("Bearer {}", token.clone().unwrap())).unwrap();
    auth_header.set_sensitive(true);
    headers.insert(AUTHORIZATION, auth_header);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    log::trace!(
        "Building HTTP client with user agent: {}, headers: {:?}, with token: {}",
        user_agent,
        headers,
        token
    );
    reqwest::Client::builder()
        .user_agent(user_agent)
        .default_headers(headers)
        .build()
}
