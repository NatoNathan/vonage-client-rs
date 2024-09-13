use super::{ConnectOptions, ConnectType, Endpoint};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct AppConnect {
    pub endpoint: Vec<AppEndpoint>,
    #[serde(flatten)]
    pub connect_options: ConnectOptions,
}

impl ConnectType<AppEndpoint> for AppConnect {
    fn with_endpoint(&mut self, endpoint: impl FnOnce(&mut AppEndpoint)) -> &mut Self {
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
#[serde(rename_all = "camelCase", tag = "type", rename = "app")]
pub struct AppEndpoint {
    pub user: String,
}

impl Endpoint for AppEndpoint {}
