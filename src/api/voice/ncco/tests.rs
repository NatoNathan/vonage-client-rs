use super::*;

extern crate pretty_env_logger;

use crate::compose;
use log::info;
use serde_json::json;
use std::collections::HashMap;

pub fn init() {
    let _ = pretty_env_logger::formatted_builder().is_test(true).try_init();
}

#[test]
fn talk_ncco() {
    init();
    info!("Testing talk_ncco");

    let ncco = NCCO::new()
        .talk_with(
            "Hello World".into(),
            compose!(
                barge_in(true),
                event_method(EventMethod::GET),
                event_url("https://example.com".into()),
                premium(true)
            ),
        )
        .conversation_with("name".into(), |conversation| {
            conversation.start_on_enter(true).end_on_exit(true);
        });

    info!("NCCO: {:?}", ncco);

    let expected = json!([{
        "action": "talk",
        "text": "Hello World",
        "bargeIn": true,
        "eventMethod": "GET",
        "eventUrl": "https://example.com",
        "premium": true
    },
    {
        "action": "conversation",
        "name": "name",
        "startOnEnter": true,
        "endOnExit": true
    }]);

    assert_eq!(serde_json::to_value(ncco).unwrap(), expected);
}

#[test]
fn connect_phone_ncco() {
    init();
    info!("Testing connect_phone_ncco");
    let to = "447700900000";
    let from = "447700900001";

    let ncco = NCCO::new().connect_phone_with(to.into(), |connect| {
        connect.options().from(from.into());
    });

    info!("NCCO: {:?}", ncco);

    let expected = json!([{
        "action": "connect",
        "from": "447700900001",
        "endpoint": [{
            "type": "phone",
            "number": "447700900000"
        }]
    }]);

    assert_eq!(serde_json::to_value(ncco).unwrap(), expected);
}

#[test]
fn connect_sip_ncco() {
    init();
    info!("Testing connect_sip_ncco");
    let to = "sip:user@sip.vonage-client.com";
    let from = "447700900001";

    let ncco = NCCO::new()
        .talk("Hello World".into())
        .connect_sip_with(to.into(), |connect| {
            connect.options().from(from.into());
        });

    info!("NCCO: {:?}", ncco);

    let expected = json!([
        {
            "action": "talk",
            "text": "Hello World"
        },
        {
            "action": "connect",
            "from": "447700900001",
            "endpoint": [{
                "type": "sip",
                "uri": "sip:user@sip.vonage-client.com"
            }]
        }
    ]);

    assert_eq!(serde_json::to_value(ncco).unwrap(), expected);
}

#[test]
fn connect_sip_ncco_with_headers() {
    init();
    info!("Testing connect_sip_ncco_with_headers");
    let to = "sip:user@sip.vonage-client.com";
    let from = "447700900001";

    let mut headers = HashMap::new();
    headers.insert("X-Header".into(), "Value".into());

    let ncco = NCCO::new()
        .talk("Hello World".into())
        .connect_sip_with(to.into(), |connect| {
            connect
                .with_endpoint(|endpoint| {
                    endpoint
                        .headers(headers.clone())
                        .user_to_user("my_value".into());
                })
                .with_options(|options| {
                    options.from(from.into());
                });
        });

    info!("NCCO: {:?}", ncco);

    let expected = json!([
        {
            "action": "talk",
            "text": "Hello World"
        },
        {
            "action": "connect",
            "from": "447700900001",
            "endpoint": [{
                "type": "sip",
                "uri": "sip:user@sip.vonage-client.com",
                "headers": {
                    "X-Header": "Value"
                },
                "standardHeaders": {
                    "User-to-User": "my_value"
                }
            }]
        }
    ]);

    assert_eq!(serde_json::to_value(ncco).unwrap(), expected);
}
