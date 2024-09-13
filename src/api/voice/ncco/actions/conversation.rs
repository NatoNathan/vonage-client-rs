use super::EventMethod;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Conversation {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_on_hold_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_on_enter: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_on_exit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_speak: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_hear: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription: Option<Transcription>,
}

impl Conversation {
    pub fn music_on_hold_url(&mut self, music_on_hold_url: String) -> &mut Self {
        self.music_on_hold_url = Some(music_on_hold_url);
        self
    }

    pub fn start_on_enter(&mut self, start_on_enter: bool) -> &mut Self {
        self.start_on_enter = Some(start_on_enter);
        self
    }

    pub fn end_on_exit(&mut self, end_on_exit: bool) -> &mut Self {
        self.end_on_exit = Some(end_on_exit);
        self
    }

    pub fn record(&mut self, record: bool) -> &mut Self {
        self.record = Some(record);
        self
    }

    pub fn can_speak(&mut self, can_speak: Vec<String>) -> &mut Self {
        self.can_speak = Some(can_speak);
        self
    }

    pub fn can_hear(&mut self, can_hear: Vec<String>) -> &mut Self {
        self.can_hear = Some(can_hear);
        self
    }

    pub fn mute(&mut self, mute: bool) -> &mut Self {
        self.mute = Some(mute);
        self
    }

    pub fn transcription(&mut self, transcription: Transcription) -> &mut Self {
        self.transcription = Some(transcription);
        self
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transcription {
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_method: Option<EventMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sentiment_analysis: Option<bool>,
}
