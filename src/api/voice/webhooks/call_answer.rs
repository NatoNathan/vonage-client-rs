use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Webhook payload for a call answer event from the Vonage Voice API
///
/// ## ServerCall
///
/// The payload for a server call event from the Client SDKs for Web, iOS, and Android.
///
/// ## InboundCall
///
/// The payload for an inbound call event from PSTN, SIP, Websocket, or VBC.
///
///
/// More Information:
///  [Vonage Voice API Reference](https://developer.vonage.com/en/voice/voice-api/webhook-reference#answer-webhook)
///
/// ## Example
///
/// ```
/// use crate::types::webhooks::{CallAnswerPayload, CallAnswerPayload::*};
/// fn handle_call_answer(payload: CallAnswerPayload) {
///    match payload {
///       ServerCall { to, from_user, uuid, conversation_uuid, region_url, custom_data } => {
///          println!("Server call from {} to {}", from_user, to);
///      },
///     InboundCall { to, from, uuid, conversation_uuid, region_url, sip_headers } => {
///        println!("Inbound call from {} to {}", from, to);
///     }
///   }
/// }
/// ```
///
///
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum VoiceAnswerPayload {
    ServerCall {
        /// The number the call was made to, in E.164 format this will almost always be a LVN unless you set a `to` property in the Server Call Context
        to: String,
        /// The user who initiated the call
        from_user: String,
        /// The unique identifier for the call/leg
        uuid: String,
        /// The unique identifier for the conversation
        conversation_uuid: String,
        /// The region URL for the call
        region_url: String,
        /// Custom data sent with the server call context in the Client SDKs
        custom_data: HashMap<String, Value>,
    },
    InboundCall {
        /// The number the call was made to, in E.164 format this will always be a LVN
        to: String,
        /// The number the call was made from
        from: String,
        /// The unique identifier for the call/leg
        uuid: String,
        /// The unique identifier for the conversation
        conversation_uuid: String,
        /// The region URL for the call
        region_url: String,
        /// SIP headers sent with the inbound call
        #[serde(flatten)]
        sip_headers: HashMap<String, String>,
    },
}

#[cfg(test)]
mod tests {
    extern crate pretty_env_logger;
    use super::{VoiceAnswerPayload, VoiceAnswerPayload::*};
    use log::info;
    use serde_json::json;

    pub fn init() {
        let _ = pretty_env_logger::formatted_builder()
            .filter_level(log::LevelFilter::Info)
            .is_test(true)
            .try_init();
    }
    #[test]
    fn answer_server_call() {
        init();
        info!("Testing answer_server_call");

        let payload = json!({
            "to": "447700900000",
            "from_user": "alice",
            "uuid": "xxx-xxx-xxx-xxx",
            "conversation_uuid": "yyy-yyy-yyy-yyy",
            "region_url": "https://api.nexmo.com",
            "custom_data": {
                "my_key": "my_value"
            }
        });

        let call_answer: VoiceAnswerPayload = serde_json::from_value(payload).unwrap();

        info!("Payload: {:?}", call_answer);

        match call_answer {
            ServerCall {
                to,
                from_user,
                uuid,
                conversation_uuid,
                region_url,
                custom_data,
            } => {
                assert_eq!(to, "447700900000");
                assert_eq!(from_user, "alice");
                assert_eq!(uuid, "xxx-xxx-xxx-xxx");
                assert_eq!(conversation_uuid, "yyy-yyy-yyy-yyy");
                assert_eq!(region_url, "https://api.nexmo.com");
                assert_eq!(
                    custom_data.get("my_key").unwrap().as_str().unwrap(),
                    "my_value"
                );
            }
            _ => panic!("Unexpected payload"),
        }
    }

    #[test]
    fn answer_inbound_call() {
        init();
        info!("Testing answer_inbound_call");

        let payload = json!({
            "to": "447700900000",
            "from": "447700900001",
            "uuid": "xxx-xxx-xxx-xxx",
            "conversation_uuid": "yyy-yyy-yyy-yyy",
            "region_url": "https://api.nexmo.com",
            "SipHeader_X-User-to_User": "my_value"
        });

        let call_answer: VoiceAnswerPayload = serde_json::from_value(payload).unwrap();

        info!("Payload: {:?}", call_answer);

        match call_answer {
            InboundCall {
                to,
                from,
                uuid,
                conversation_uuid,
                region_url,
                sip_headers,
            } => {
                assert_eq!(to, "447700900000");
                assert_eq!(from, "447700900001");
                assert_eq!(uuid, "xxx-xxx-xxx-xxx");
                assert_eq!(conversation_uuid, "yyy-yyy-yyy-yyy");
                assert_eq!(region_url, "https://api.nexmo.com");
                assert_eq!(
                    sip_headers.get("SipHeader_X-User-to_User").unwrap(),
                    "my_value"
                );
            }
            _ => panic!("Unexpected payload"),
        }
    }
}
