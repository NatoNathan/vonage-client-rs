use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct AclRules {
    #[serde(skip_serializing_if = "Option::is_none")]
    methods: Option<Vec<AclMethod>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AclMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct VonageAcl {
    paths: HashMap<String, AclRules>,
}

impl VonageAcl {
    pub fn new() -> Self {
        VonageAcl {
            paths: HashMap::new(),
        }
    }

    /// Add a path to the ACL with optional methods
    pub fn add_path(&mut self, path: String, methods: Option<Vec<AclMethod>>) -> &mut Self {
        self.paths.insert(path, AclRules { methods });
        self
    }
}

impl Default for VonageAcl {
    /// Create a default Vonage ACL with the default rules for the Client SDK to work
    fn default() -> Self {
        let mut acl = VonageAcl::new();
        acl.add_path("/*/sessions/**".to_string(), None)
            .add_path("/*/users/**".to_string(), None)
            .add_path("/*/conversations/**".to_string(), None)
            .add_path("/*/image/**".to_string(), None)
            .add_path("/*/media/**".to_string(), None)
            .add_path("/*/knocking/**".to_string(), None)
            .add_path("/*/push/**".to_string(), None)
            .add_path("/*/devices/**".to_string(), None)
            .add_path("/*/applications/**".to_string(), None)
            .add_path("/*/legs/**".to_string(), None);
        acl
    }
}

#[cfg(test)]
mod tests {
    extern crate pretty_env_logger;
    use super::*;
    use log::info;
    use log::LevelFilter::Debug;
    use serde_json::json;
    fn init() {
        let _ = pretty_env_logger::formatted_builder()
            .filter_level(Debug)
            .is_test(true)
            .try_init();
    }
    #[test]
    fn test_acl_empty_rules() {
        init();
        let expected_json = json!(
            {
             "paths": {
                 "/*/rtc/**": {},
                }
            }
        );

        let mut acl = VonageAcl::new();
        acl.add_path("/*/rtc/**".to_string(), None);

        let acl_json = serde_json::to_value(&acl).unwrap();
        info!("ACL JSON: {}", acl_json);
        assert_eq!(acl_json, expected_json);
    }

    #[test]
    fn test_acl_with_rules() {
        let expected_json = json!(
            {
             "paths": {
                 "/*/rtc/**": {
                     "methods": ["GET", "POST"]
                 },
                 "/v1/users/**": {
                     "methods": ["GET", "POST", "PUT", "DELETE", "PATCH"]
                 }
                }
            }
        );

        let mut acl = VonageAcl::new();
        acl.add_path(
            "/*/rtc/**".to_string(),
            Some(vec![AclMethod::GET, AclMethod::POST]),
        );
        acl.add_path(
            "/v1/users/**".to_string(),
            Some(vec![
                AclMethod::GET,
                AclMethod::POST,
                AclMethod::PUT,
                AclMethod::DELETE,
                AclMethod::PATCH,
            ]),
        );

        let acl_json = serde_json::to_value(&acl).unwrap();
        assert_eq!(acl_json, expected_json);
    }
}
