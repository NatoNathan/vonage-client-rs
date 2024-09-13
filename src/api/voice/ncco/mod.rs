#[cfg(test)]
mod tests;

mod action;
mod actions;

pub use action::*;
pub use actions::{
    AppConnect, AppEndpoint, AudioFormat, ConnectType, MachineDetection, PhoneConnect,
    PhoneEndpoint, SipConnect, SipEndpoint, SipHeaders, SipStandardHeaders, VbcConnect,
    VbcEndpoint, WebsocketConnect, WebsocketEndpoint, WebsocketHeaders,
};
use actions::{Connect, Conversation, Talk};
use serde::{Deserialize, Serialize};

/// The Nexmo Call Control Object (NCCO) is a JSON array that you use to control the flow of a Vonage Voice API call flows.
///
/// [Vonage Voice API NCCO Reference](https://developer.vonage.com/voice/voice-api/ncco-reference)
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NCCO(Vec<Action>);

impl NCCO {
    pub fn new() -> Self {
        NCCO(Vec::new())
    }

    /// Add a `Talk` action to the NCCO
    ///
    /// # Example
    ///
    /// ```
    /// use vonage::ncco::{NCCO, Talk};
    /// let ncco = NCCO::new().talk("Hello".to_string());
    /// ```
    ///
    pub fn talk(self, text: String) -> Self {
        self.add_action(Action::Talk(Talk {
            text,
            ..Default::default()
        }))
    }
    /// Add a `Talk action with additional options to the NCCO
    ///
    /// # Example
    ///
    /// ```
    /// use vonage::ncco::{NCCO, Talk};
    /// let ncco = NCCO::new().talk_with("Hello".to_string(), |talk| {
    ///    talk.barge_in = Some(true);
    /// });
    /// ```
    ///
    /// # Parameters
    ///
    /// - `text` - The text to be spoken
    /// - `opt_fn` - A closure that takes a mutable reference to a `Talk` object and allows you to set additional options
    ///
    pub fn talk_with(self, text: String, talk_fn: impl FnOnce(&mut Talk)) -> Self {
        let mut talk = Talk {
            text,
            ..Default::default()
        };
        talk_fn(&mut talk);
        self.add_action(Action::Talk(talk))
    }

    /// Add a `Connect` action with a Phone endpoint to the NCCO
    pub fn connect_phone(self, number: String) -> Self {
        let connect = Connect::Phone(PhoneConnect {
            endpoint: vec![PhoneEndpoint {
                number,
                ..Default::default()
            }],
            ..Default::default()
        });
        self.add_action(Action::Connect(connect))
    }

    /// Add a `Connect` action with a Phone endpoint and additional options to the NCCO
    pub fn connect_phone_with(
        self,
        number: String,
        connect_fn: impl FnOnce(&mut PhoneConnect),
    ) -> Self {
        let mut connect = PhoneConnect {
            endpoint: vec![PhoneEndpoint {
                number,
                ..Default::default()
            }],
            ..Default::default()
        };
        connect_fn(&mut connect);
        self.add_action(Action::Connect(Connect::Phone(connect)))
    }

    /// Add a `Connect` action with an App endpoint to the NCCO
    pub fn connect_app(self, user: String) -> Self {
        let connect = Connect::App(AppConnect {
            endpoint: vec![AppEndpoint { user }],
            ..Default::default()
        });
        self.add_action(Action::Connect(connect))
    }

    /// Add a `Connect` action with an App endpoint and additional options to the NCCO
    pub fn connect_app_with(self, user: String, connect_fn: impl FnOnce(&mut AppConnect)) -> Self {
        let mut connect = AppConnect {
            endpoint: vec![AppEndpoint { user }],
            ..Default::default()
        };
        connect_fn(&mut connect);
        self.add_action(Action::Connect(Connect::App(connect)))
    }

    /// Add a `Connect` action with a Websocket endpoint to the NCCO
    pub fn connect_websocket(self, uri: String, content_type: AudioFormat) -> Self {
        let connect = Connect::Websocket(WebsocketConnect {
            endpoint: vec![WebsocketEndpoint {
                uri,
                content_type,
                ..Default::default()
            }],
            ..Default::default()
        });
        self.add_action(Action::Connect(connect))
    }

    /// Add a `Connect` action with a Websocket endpoint and additional options to the NCCO
    pub fn connect_websocket_with(
        self,
        uri: String,
        content_type: AudioFormat,
        connect_fn: impl FnOnce(&mut WebsocketConnect),
    ) -> Self {
        let mut connect = WebsocketConnect {
            endpoint: vec![WebsocketEndpoint {
                uri,
                content_type,
                ..Default::default()
            }],
            ..Default::default()
        };
        connect_fn(&mut connect);
        self.add_action(Action::Connect(Connect::Websocket(connect)))
    }

    /// Add a `Connect` action with a Sip endpoint to the NCCO
    pub fn connect_sip(self, uri: String) -> Self {
        let connect = Connect::Sip(SipConnect {
            endpoint: vec![SipEndpoint {
                uri,
                ..Default::default()
            }],
            ..Default::default()
        });
        self.add_action(Action::Connect(connect))
    }

    /// Add a `Connect` action with a Sip endpoint and additional options to the NCCO
    pub fn connect_sip_with(self, uri: String, connect_fn: impl FnOnce(&mut SipConnect)) -> Self {
        let mut connect = SipConnect {
            endpoint: vec![SipEndpoint {
                uri,
                ..Default::default()
            }],
            ..Default::default()
        };
        connect_fn(&mut connect);
        self.add_action(Action::Connect(Connect::Sip(connect)))
    }

    /// Add a `Connect` action with a Vbc endpoint to the NCCO
    pub fn connect_vbc(self, extension: String) -> Self {
        let connect = Connect::Vbc(VbcConnect {
            endpoint: vec![VbcEndpoint { extension }],
            ..Default::default()
        });
        self.add_action(Action::Connect(connect))
    }

    /// Add a `Connect` action with a Vbc endpoint and additional options to the NCCO
    pub fn connect_vbc_with(
        self,
        extension: String,
        connect_fn: impl FnOnce(&mut VbcConnect),
    ) -> Self {
        let mut connect = VbcConnect {
            endpoint: vec![VbcEndpoint { extension }],
            ..Default::default()
        };
        connect_fn(&mut connect);
        self.add_action(Action::Connect(Connect::Vbc(connect)))
    }

    /// Add a `Conversation` action to the NCCO
    pub fn conversation(self, name: String) -> Self {
        let conversation = Conversation {
            name,
            ..Default::default()
        };
        self.add_action(Action::Conversation(conversation))
    }

    /// Add a `Conversation` action with additional options to the NCCO
    pub fn conversation_with(
        self,
        name: String,
        conversation_fn: impl FnOnce(&mut Conversation),
    ) -> Self {
        let mut conversation = Conversation {
            name,
            ..Default::default()
        };
        conversation_fn(&mut conversation);
        self.add_action(Action::Conversation(conversation))
    }

    /// Add an `Action` to the NCCO
    fn add_action(mut self, action: Action) -> Self {
        self.0.push(action);
        self
    }
}

/// Compose helper functions in options closures
///
/// # Example
///
/// ```
/// let ncco = NCCO::new()
///    .connect_phone_with(to.into(), compose!(from(from), event_type(EventType::Answer)));
///
/// // is equivalent to
///
/// let ncco = NCCO::new()
///   .connect_phone_with(to.into(), |connect| {
///      connect.from(from).event_type(EventType::Answer);
///  });
/// ```
///
/// # Parameters
///
/// - `fns` - A list of functions to compose
///
/// # Returns
///
/// A closure that takes a mutable reference to a type and applies the functions in the list
#[macro_export]
macro_rules! compose {
    ($($fn:ident($arg:expr)),*) => {
        |x| {
            $(x.$fn($arg);)*
        }
    };
}
