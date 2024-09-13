mod create_call;
mod ncco;

pub mod webhooks;
use async_trait::async_trait;
pub use create_call::*;
pub use ncco::*;

use crate::client::{VonageClient, VonageClientError};

#[async_trait]
pub trait VoiceApi {
    const API_PATH: &'static str;
    /// Create Call
    /// This function creates an outbound call using the Vonage API
    /// @param create_call The call to create
    /// @return The response from the Vonage API
    async fn create_outbound_call(
        &mut self,
        create_call: CreateCall,
    ) -> Result<CreateCallResponse, VonageClientError>;
}

#[async_trait]
impl VoiceApi for VonageClient {
    const API_PATH: &'static str = "/v1/calls";
    async fn create_outbound_call(
        &mut self,
        create_call: CreateCall,
    ) -> Result<CreateCallResponse, VonageClientError> {
        log::debug!("Creating outbound call: {:?}", create_call);
        let path = Self::API_PATH;
        self.post(path, create_call).await
    }
}
