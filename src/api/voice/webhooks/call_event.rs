use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum CallEventPayload {
    CallStatus(CallStatusEvent),
    Input(InputEvent),
    Transfer(TransferEvent),
    Play(PlayEvent),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum CallStatusEvent {
    Started {
        from: String,
        to: String,
        uuid: String,
        conversation_uuid: String,
        direction: Direction,
        timestamp: String,
    },
    Ringing {
        from: String,
        to: String,
        uuid: String,
        conversation_uuid: String,
        direction: Direction,
        timestamp: String,
    },
    Answered {
        from: String,
        to: String,
        uuid: String,
        conversation_uuid: String,
        direction: Direction,
        timestamp: String,
        rate: f64,
        network: Option<String>,
    },
    Busy {
        from: String,
        to: String,
        uuid: String,
        conversation_uuid: String,
        direction: Direction,
        timestamp: String,
    },
    Cancelled {
        from: String,
        to: String,
        uuid: String,
        conversation_uuid: String,
        direction: Direction,
        timestamp: String,
    },
    Unanswered {
        from: String,
        to: String,
        uuid: String,
        conversation_uuid: String,
        direction: Direction,
        timestamp: String,
        detail: UnansweredDetail,
    },
    Disconnected {
        from: String,
        to: String,
        uuid: String,
        conversation_uuid: String,
        direction: Direction,
        timestamp: String,
        duration: String,
    },
    Redirected {
        from: String,
        to: String,
        uuid: String,
        conversation_uuid: String,
        direction: Direction,
        timestamp: String,
        detail: RejectedDetail,
    },
    Rejected {
        from: String,
        to: String,
        uuid: String,
        conversation_uuid: String,
        direction: Direction,
        timestamp: String,
        detail: RejectedDetail,
    },
    Failed {
        from: String,
        to: String,
        uuid: String,
        conversation_uuid: String,
        direction: Direction,
        timestamp: String,
        detail: FailureDetail,
    },
    Human {
        from: String,
        to: String,
        uuid: String,
        conversation_uuid: String,
        direction: Direction,
        timestamp: String,
        sub_state: HumanMachineSubState,
    },
    Machine {
        from: String,
        to: String,
        uuid: String,
        conversation_uuid: String,
        direction: Direction,
        timestamp: String,
        sub_state: HumanMachineSubState,
    },
    Timeout {
        from: String,
        to: String,
        uuid: String,
        conversation_uuid: String,
        direction: Direction,
        timestamp: String,
    },
    Completed {
        from: String,
        to: String,
        uuid: String,
        conversation_uuid: String,
        direction: Direction,
        timestamp: String,
        duration: usize,
        network: Option<String>,
        end_time: String,
        rate: f64,
        price: f64,
        disconnected_by: CompletedByDisconnectionBy,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Inbound,
    Outbound,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UnansweredDetail {
    Timeout,
    Unavailable,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RejectedDetail {
    InvalidNumber,
    Declined,
    Restricted,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum FailureDetail {
    CannotRoute,
    NumberOutOfService,
    InternalError,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum HumanMachineSubState {
    BeepStart,
    BeepTimeout,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CompletedByDisconnectionBy {
    User,
    Platform,
}
// Input Events
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum InputEvent {
    DTMF {
        from: String,
        to: String,
        uuid: String,
        conversation_uuid: String,
        timestamp: String,
        dtmf: DTMFPayload,
    },
    Speech {
        from: String,
        to: String,
        uuid: String,
        conversation_uuid: String,
        timestamp: String,
        speech: SpeechPayload,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DTMFPayload {
    dtmf: String,
    timed_out: bool,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SpeechPayload {
    Success {
        recording_url: String,
        timeout_reason: Option<String>,
        result: Option<Vec<SpeechResult>>,
    },
    Failure {
        recording_url: String,
        timeout_reason: Option<String>,
        error: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpeechResult {
    text: String,
    confidence: f64,
}

// Transfer Events
#[derive(Serialize, Deserialize, Debug)]
pub struct TransferEvent {
    pub conversation_uuid_from: String,
    pub conversation_uuid_to: String,
    pub uuid: String,
    pub timestamp: String,
}

// Talk/Stream Events
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PlayEvent {
    Talk {
        timestamp: String,
        uuid: String,
        conversation_uuid: String,
        status: PlayStatus,
    },
    Stream {
        timestamp: String,
        uuid: String,
        conversation_uuid: String,
        status: PlayStatus,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PlayStatus {
    Stopped,
    Finished,
    Interrupted,
}

#[cfg(test)]
mod tests {
    extern crate pretty_env_logger;
    use super::CallEventPayload;
    use super::*;
    use log::info;
    use serde_json::json;
    use CallEventPayload::*;
    use CallStatusEvent::*;
    pub fn init() {
        let _ = pretty_env_logger::formatted_builder().is_test(true).try_init();
    }

    #[test]
    fn call_status_event() {
        init();
        info!("Testing call_status_event");

        let payload = json!({
            "status": "started",
            "from": "447700900000",
            "to": "447700900001",
            "uuid": "xxx-xxx-xxx-xxx",
            "conversation_uuid": "yyy-yyy-yyy-yyy",
            "direction": "inbound",
            "timestamp": "2021-01-01T00:00:00Z"
        });

        let call_event: CallEventPayload = serde_json::from_value(payload).unwrap();

        info!("Payload: {:?}", call_event);

        match call_event {
            CallStatus(Started {
                from,
                to,
                uuid,
                conversation_uuid,
                direction,
                timestamp,
            }) => {
                assert_eq!(from, "447700900000");
                assert_eq!(to, "447700900001");
                assert_eq!(uuid, "xxx-xxx-xxx-xxx");
                assert_eq!(conversation_uuid, "yyy-yyy-yyy-yyy");
                assert_eq!(direction, Direction::Inbound);
                assert_eq!(timestamp, "2021-01-01T00:00:00Z");
            }
            _ => panic!("Unexpected payload"),
        }
    }

    #[test]
    fn call_status_event_ringing() {
        init();
        info!("Testing call_status_event_ringing");

        let payload = json!({
            "status": "ringing",
            "from": "447700900000",
            "to": "447700900001",
            "uuid": "xxx-xxx-xxx-xxx",
            "conversation_uuid": "yyy-yyy-yyy-yyy",
            "direction": "outbound",
            "timestamp": "2021-01-01T00:00:00Z"
        });

        let call_event: CallEventPayload = serde_json::from_value(payload).unwrap();

        info!("Payload: {:?}", call_event);

        match call_event {
            CallStatus(Ringing {
                from,
                to,
                uuid,
                conversation_uuid,
                direction,
                timestamp,
            }) => {
                assert_eq!(from, "447700900000");
                assert_eq!(to, "447700900001");
                assert_eq!(uuid, "xxx-xxx-xxx-xxx");
                assert_eq!(conversation_uuid, "yyy-yyy-yyy-yyy");
                assert_eq!(direction, Direction::Outbound);
                assert_eq!(timestamp, "2021-01-01T00:00:00Z");
            }
            _ => panic!("Unexpected payload"),
        }
    }

    #[test]
    fn call_transfer_event() {
        init();
        info!("Testing call_transfer_event");

        let payload = json!({
            "conversation_uuid_from": "yyy-yyy-yyy-yyy",
            "conversation_uuid_to": "zzz-zzz-zzz-zzz",
            "uuid": "xxx-xxx-xxx-xxx",
            "timestamp": "2021-01-01T00:00:00Z"
        });

        let call_event: CallEventPayload = serde_json::from_value(payload).unwrap();

        info!("Payload: {:?}", call_event);

        match call_event {
            Transfer(TransferEvent {
                conversation_uuid_from,
                conversation_uuid_to,
                uuid,
                timestamp,
            }) => {
                assert_eq!(conversation_uuid_from, "yyy-yyy-yyy-yyy");
                assert_eq!(conversation_uuid_to, "zzz-zzz-zzz-zzz");
                assert_eq!(uuid, "xxx-xxx-xxx-xxx");
                assert_eq!(timestamp, "2021-01-01T00:00:00Z");
            }
            _ => panic!("Unexpected payload"),
        }
    }
}
