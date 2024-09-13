
extern crate pretty_env_logger;
#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use vonage_client::{compose, voice::webhooks::{CallEventPayload, VoiceAnswerPayload}, EventMethod, NCCO};

static USER_TO_CALL: &str = "bob";

lazy_static::lazy_static! {
    static ref PUBLIC_URL: String = std::env::var("PUBLIC_URL").unwrap_or_else(|_| "http://localhost:8000".into());
}

#[post("/voice/answer", data = "<answer>")]
fn answer_server_call(answer: Json<VoiceAnswerPayload>) -> Json<NCCO> {
    log::info!("Answer Payload: {:?}", answer);


    let ncco = NCCO::new()
        .talk_with(
            "Hello World, We are getting rusty with Rocket and Vonage!".to_string(),
            compose!(
                barge_in(true),
                event_method(EventMethod::POST),
                event_url(format!("{}/voice/event", *PUBLIC_URL)),
                premium(true)
            ),
        )
        .connect_app(USER_TO_CALL.into());

    log::info!("NCCO: {:?}", ncco);

    Json(ncco)
}

#[post("/voice/event", data = "<event>")]
fn event_server_call(event: Json<CallEventPayload>) {
    log::info!("Event Payload: {:?}", event);
}

#[launch]
fn rocket() -> _ {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    rocket::build().mount("/", routes![answer_server_call, event_server_call])
}