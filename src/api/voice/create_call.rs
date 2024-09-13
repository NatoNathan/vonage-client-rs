use serde::{Deserialize, Serialize};

use crate::voice::{
    webhooks::Direction, AudioFormat, EventMethod, MachineDetection, SipHeaders,
    SipStandardHeaders, WebsocketHeaders, NCCO,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCall {
    Ncco {
        ncco: NCCO,
        to: Vec<CreateCallTo>,
        #[serde(skip_serializing_if = "Option::is_none")]
        from: Option<From>,
        #[serde(skip_serializing_if = "Option::is_none")]
        random_from_number: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        event_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        event_method: Option<EventMethod>,
        #[serde(skip_serializing_if = "Option::is_none")]
        machine_detection: Option<MachineDetection>,
        #[serde(skip_serializing_if = "Option::is_none")]
        advanced_machine_detection: Option<AdvancedMachineDetection>,
        #[serde(skip_serializing_if = "Option::is_none")]
        length_timer: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        ringing_timer: Option<usize>,
    },
    AnswerUrl {
        answer_url: String,
        to: CreateCallTo,
        #[serde(skip_serializing_if = "Option::is_none")]
        from: Option<From>,
        #[serde(skip_serializing_if = "Option::is_none")]
        random_from_number: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        answer_method: Option<EventMethod>,
        #[serde(skip_serializing_if = "Option::is_none")]
        event_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        event_method: Option<EventMethod>,
        #[serde(skip_serializing_if = "Option::is_none")]
        machine_detection: Option<MachineDetection>,
        #[serde(skip_serializing_if = "Option::is_none")]
        advanced_machine_detection: Option<AdvancedMachineDetection>,
        #[serde(skip_serializing_if = "Option::is_none")]
        length_timer: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        ringing_timer: Option<usize>,
    },
}

impl CreateCall {
    /// Build a new CreateCall object with an NCCO payload, using a builder pattern.
    ///
    /// # Example
    /// ```rust
    /// use vonage_client::client::CreateCall;
    /// use vonage_client::NCCO;
    ///
    /// let ncco = NCCO::new()
    ///    .add_text("Hello, world!");
    /// let create_call = CreateCall::build_ncco()
    ///    .ncco(ncco)
    ///    .to(To::Phone(ToPhone::new("some-number".into()))
    ///    .random_from_number(true)
    ///    .build();
    /// ```
    pub fn build_ncco() -> CreateCallNccoBuilder {
        CreateCallNccoBuilder::new()
    }

    /// Build a new CreateCall object with an answer URL, using a builder pattern.
    ///
    /// # Example
    /// ```rust
    /// use vonage_client::client::CreateCall;
    ///
    /// let create_call = CreateCall::build_answer_url()
    ///   .answer_url("https://example.com/answer")
    ///   .to(To::Phone(ToPhone::new("some-number".into()))
    ///   .random_from_number(true)
    ///   .build();
    /// ```
    pub fn build_answer_url() -> CreateCallAnswerUrlBuilder {
        CreateCallAnswerUrlBuilder::new()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CreateCallResponse {
    uuid: String,
    conversation_uuid: String,
    status: CreateCallStatus,
    direction: Direction,
}

impl CreateCallResponse {
    pub fn uuid(&self) -> &str {
        &self.uuid
    }

    pub fn conversation_uuid(&self) -> &str {
        &self.conversation_uuid
    }

    pub fn status(&self) -> &CreateCallStatus {
        &self.status
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
#[cfg(feature = "mocking")]
impl CreateCallResponse {
    pub fn new(
        uuid: String,
        conversation_uuid: String,
        status: CreateCallStatus,
        direction: Direction,
    ) -> Self {
        Self {
            uuid,
            conversation_uuid,
            status,
            direction,
        }
    }
}

pub struct CreateCallNccoBuilder {
    ncco: Option<NCCO>,
    to: Vec<CreateCallTo>,
    from: Option<From>,
    random_from_number: Option<bool>,
    event_url: Option<String>,
    event_method: Option<EventMethod>,
    machine_detection: Option<MachineDetection>,
    advanced_machine_detection: Option<AdvancedMachineDetection>,
    length_timer: Option<usize>,
    ringing_timer: Option<usize>,
}

impl CreateCallNccoBuilder {
    pub fn new() -> Self {
        Self {
            ncco: None,
            to: Vec::new(),
            from: None,
            random_from_number: None,
            event_url: None,
            event_method: None,
            machine_detection: None,
            advanced_machine_detection: None,
            length_timer: None,
            ringing_timer: None,
        }
    }

    pub fn ncco(&mut self, ncco: NCCO) -> &mut Self {
        self.ncco = Some(ncco);
        self
    }

    pub fn with_new_ncco(&mut self) -> &mut NCCO {
        self.ncco = Some(NCCO::new());
        self.ncco.as_mut().unwrap()
    }

    pub fn to(&mut self, to: CreateCallTo) -> &mut Self {
        if self.to.len() > 1 {
            panic!("You can only set one To object");
        }
        self.to.push(to);
        self
    }

    pub fn from(&mut self, from: From) -> &mut Self {
        if self.random_from_number.is_some() {
            panic!("You cannot can set from or random_from_number, but not both");
        }
        self.from = Some(from);
        self
    }

    pub fn random_from_number(&mut self, random_from_number: bool) -> &mut Self {
        if self.from.is_some() {
            panic!("You cannot can set random_from_number or from, but not both");
        }
        self.random_from_number = Some(random_from_number);
        self
    }

    pub fn event_url(&mut self, event_url: String) -> &mut Self {
        self.event_url = Some(event_url);
        self
    }

    pub fn event_method(&mut self, event_method: EventMethod) -> &mut Self {
        self.event_method = Some(event_method);
        self
    }

    pub fn machine_detection(&mut self, machine_detection: MachineDetection) -> &mut Self {
        if self.advanced_machine_detection.is_some() {
            panic!("you can only set one of machine_detection or advanced_machine_detection");
        }
        self.machine_detection = Some(machine_detection);
        self
    }

    pub fn advanced_machine_detection(
        &mut self,
        advanced_machine_detection: AdvancedMachineDetection,
    ) -> &mut Self {
        if self.machine_detection.is_some() {
            panic!("you can only set one of advanced_machine_detection or machine_detection");
        }
        self.advanced_machine_detection = Some(advanced_machine_detection);
        self
    }

    pub fn length_timer(&mut self, length_timer: usize) -> &mut Self {
        self.length_timer = Some(length_timer);
        self
    }

    pub fn ringing_timer(&mut self, ringing_timer: usize) -> &mut Self {
        self.ringing_timer = Some(ringing_timer);
        self
    }

    pub fn build(&self) -> CreateCall {
        let ncco = self.ncco.clone().expect("ncco is required");
        if self.to.len() != 1 {
            panic!("You must set one To object");
        };
        if self.from.is_none() && self.random_from_number.is_none() {
            panic!("You must set either from or random_from_number");
        }
        CreateCall::Ncco {
            ncco,
            to: self.to.clone(),
            from: self.from.clone(),
            random_from_number: self.random_from_number,
            event_url: self.event_url.clone(),
            event_method: self.event_method.clone(),
            machine_detection: self.machine_detection.clone(),
            advanced_machine_detection: self.advanced_machine_detection.clone(),
            length_timer: self.length_timer,
            ringing_timer: self.ringing_timer,
        }
    }
}

pub struct CreateCallAnswerUrlBuilder {
    to: Option<CreateCallTo>,
    from: Option<From>,
    random_from_number: Option<bool>,
    answer_url: Option<String>,
    answer_method: Option<EventMethod>,
    event_url: Option<String>,
    event_method: Option<EventMethod>,
    machine_detection: Option<MachineDetection>,
    advanced_machine_detection: Option<AdvancedMachineDetection>,
    length_timer: Option<usize>,
    ringing_timer: Option<usize>,
}

impl CreateCallAnswerUrlBuilder {
    fn new() -> Self {
        Self {
            to: None,
            from: None,
            random_from_number: None,
            answer_url: None,
            answer_method: None,
            event_url: None,
            event_method: None,
            machine_detection: None,
            advanced_machine_detection: None,
            length_timer: None,
            ringing_timer: None,
        }
    }

    pub fn to(&mut self, to: CreateCallTo) -> &mut Self {
        self.to = Some(to);
        self
    }

    pub fn from(&mut self, from: From) -> &mut Self {
        if self.random_from_number.is_some() {
            panic!("Cannot set both from and random_from_number");
        }
        self.from = Some(from);
        self
    }

    pub fn random_from_number(&mut self, random_from_number: bool) -> &mut Self {
        if self.from.is_some() {
            panic!("Cannot set both from and random_from_number");
        }
        self.random_from_number = Some(random_from_number);
        self
    }

    pub fn answer_url(&mut self, answer_url: String) -> &mut Self {
        self.answer_url = Some(answer_url);
        self
    }

    pub fn answer_method(&mut self, answer_method: EventMethod) -> &mut Self {
        self.answer_method = Some(answer_method);
        self
    }

    pub fn event_url(&mut self, event_url: String) -> &mut Self {
        self.event_url = Some(event_url);
        self
    }

    pub fn event_method(&mut self, event_method: EventMethod) -> &mut Self {
        self.event_method = Some(event_method);
        self
    }

    pub fn machine_detection(&mut self, machine_detection: MachineDetection) -> &mut Self {
        if self.advanced_machine_detection.is_some() {
            panic!("Cannot set both machine_detection and advanced_machine_detection");
        }
        self.machine_detection = Some(machine_detection);
        self
    }

    pub fn advanced_machine_detection(
        &mut self,
        advanced_machine_detection: AdvancedMachineDetection,
    ) -> &mut Self {
        if self.machine_detection.is_some() {
            panic!("Cannot set both machine_detection and advanced_machine_detection");
        }
        self.advanced_machine_detection = Some(advanced_machine_detection);
        self
    }

    pub fn length_timer(&mut self, length_timer: usize) -> &mut Self {
        self.length_timer = Some(length_timer);
        self
    }

    pub fn ringing_timer(&mut self, ringing_timer: usize) -> &mut Self {
        self.ringing_timer = Some(ringing_timer);
        self
    }

    pub fn build(&self) -> CreateCall {
        let answer_url = self.answer_url.clone().expect("answer_url is required");
        let to = self.to.clone().expect("to is required");
        CreateCall::AnswerUrl {
            answer_url,
            to,
            from: self.from.clone(),
            random_from_number: self.random_from_number,
            answer_method: self.answer_method.clone(),
            event_url: self.event_url.clone(),
            event_method: self.event_method.clone(),
            machine_detection: self.machine_detection.clone(),
            advanced_machine_detection: self.advanced_machine_detection.clone(),
            length_timer: self.length_timer,
            ringing_timer: self.ringing_timer,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum CreateCallTo {
    Phone(CallToPhone),
    Sip(CallToSip),
    Websocket(CallToWebsocket),
    Vbc(ToVbc),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallToPhone {
    number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    dtmf_answer: Option<String>,
}

impl CallToPhone {
    pub fn new(number: String) -> Self {
        Self {
            number,
            dtmf_answer: None,
        }
    }

    pub fn dtmf_answer(&mut self, dtmf_answer: String) -> &mut Self {
        self.dtmf_answer = Some(dtmf_answer);
        self
    }

    pub fn build(&self) -> CreateCallTo {
        CreateCallTo::Phone(self.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallToSip {
    uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<SipHeaders>,
    #[serde(skip_serializing_if = "Option::is_none")]
    standard_headers: Option<SipStandardHeaders>,
}

impl CallToSip {
    pub fn new(uri: String) -> Self {
        Self {
            uri,
            headers: None,
            standard_headers: None,
        }
    }

    pub fn headers(&mut self, headers: SipHeaders) -> &mut Self {
        self.headers = Some(headers);
        self
    }

    pub fn standard_headers(&mut self, standard_headers: SipStandardHeaders) -> &mut Self {
        self.standard_headers = Some(standard_headers);
        self
    }

    pub fn build(&self) -> CreateCallTo {
        CreateCallTo::Sip(self.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CallToWebsocket {
    uri: String,
    content_type: AudioFormat,
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<WebsocketHeaders>,
}

impl CallToWebsocket {
    pub fn new(uri: String, content_type: AudioFormat) -> Self {
        Self {
            uri,
            content_type,
            headers: None,
        }
    }

    pub fn headers(&mut self, headers: WebsocketHeaders) -> &mut Self {
        self.headers = Some(headers);
        self
    }

    pub fn build(&self) -> CreateCallTo {
        CreateCallTo::Websocket(self.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToVbc {
    extension: String,
}

impl ToVbc {
    pub fn new(extension: String) -> Self {
        Self { extension }
    }

    pub fn build(&self) -> CreateCallTo {
        CreateCallTo::Vbc(self.clone())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename = "phone", rename_all = "camelCase")]
pub struct From {
    number: String,
}

impl From {
    pub fn new(number: String) -> Self {
        Self { number }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedMachineDetection {
    behavior: MachineDetection,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<AdvancedMachineDetectionMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    beep_timeout: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AdvancedMachineDetectionMode {
    Default,
    Detect,
    DetectBeep,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CreateCallStatus {
    Started,
    Ringing,
    Answered,
    Completed,
    // TODO: Add more statuses for Failed calls
}

#[cfg(test)]
mod tests {
    extern crate pretty_env_logger;
    use super::*;
    use log::{debug, info, LevelFilter::Debug};
    use serde_json::json;

    fn init() {
        let _ = pretty_env_logger::formatted_builder()
            .filter_level(Debug)
            .is_test(true)
            .try_init();
    }

    #[test]
    fn test_ncco_create_call() {
        init();
        let ncco = NCCO::new().talk("Hello World!".into());

        let create_call = CreateCall::build_ncco()
            .ncco(ncco)
            .to(CallToPhone::new("to-number".into()).build())
            .random_from_number(true)
            .build();

        info!("NccoCreateCall: {:?}", create_call);
        debug!(
            "NccoCreateCall: {:?}",
            serde_json::to_string(&create_call).unwrap()
        );

        let json = json!({
            "ncco": [
                {
                    "action": "talk",
                    "text": "Hello World!"
                }
            ],
            "to": {
                "type": "phone",
                "number": "to-number"
            },
            "random_from_number": true
        });

        assert_eq!(serde_json::to_value(&create_call).unwrap(), json);
    }

    #[test]
    fn test_answer_url_create_call() {
        init();
        let create_call = CreateCall::build_answer_url()
            .answer_url("answer-url".into())
            .to(CreateCallTo::Phone(CallToPhone::new("to-number".into())))
            .build();
        info!("AnswerUrlCreateCall: {:?}", create_call);
        debug!(
            "AnswerUrlCreateCall: {:?}",
            serde_json::to_string(&create_call).unwrap()
        );

        let json = json!({
            "answer_url": "answer-url",
            "to": {
                "type": "phone",
                "number": "to-number"
            }
        });

        assert_eq!(serde_json::to_value(&create_call).unwrap(), json);
    }

    #[test]
    fn test_create_call_response() {
        init();

        let incomming_response = json!({
               "uuid": "CALL-UUID",
               "status": "completed",
               "direction": "outbound",
               "conversation_uuid": "CONVERSATION-UUID",
        });

        let expected_response = CreateCallResponse {
            uuid: "CALL-UUID".into(),
            status: CreateCallStatus::Completed,
            direction: Direction::Outbound,
            conversation_uuid: "CONVERSATION-UUID".into(),
        };
        assert_eq!(
            serde_json::from_value::<CreateCallResponse>(incomming_response).unwrap(),
            expected_response
        );
    }
}
