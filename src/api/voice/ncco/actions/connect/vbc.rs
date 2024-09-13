use super::{ConnectOptions, ConnectType, Endpoint};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct VbcConnect {
    pub endpoint: Vec<VbcEndpoint>,
    #[serde(flatten)]
    pub connect_options: ConnectOptions,
}

impl ConnectType<VbcEndpoint> for VbcConnect {
    fn with_endpoint(&mut self, endpoint: impl FnOnce(&mut VbcEndpoint)) -> &mut Self {
        endpoint(&mut self.endpoint[0]);
        self
    }
    fn with_options(&mut self, options: impl FnOnce(&mut ConnectOptions)) -> &mut Self {
        options(&mut self.connect_options);
        self
    }
    fn options(&mut self) -> &mut ConnectOptions {
        &mut self.connect_options
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", rename = "vbc")]
pub struct VbcEndpoint {
    pub extension: String,
}
impl Endpoint for VbcEndpoint {}
