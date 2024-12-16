extern crate pretty_env_logger;
#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use rocket_ws::Message;
use vonage_client::{
    compose, voice::webhooks::{CallEventPayload, VoiceAnswerPayload}, AudioFormat, EventMethod, NCCO
};

static USER_TO_CALL: &str = "bob";

#[post("/voice/answer", data = "<answer>")]
fn answer_server_call(
    answer: Json<VoiceAnswerPayload>,
    public_url: &rocket::State<PublicUrl>,
) -> Json<NCCO> {
    log::info!("Answer Payload: {:?}", answer);
    let public_url = public_url.0.as_ref().unwrap();

    let ncco = NCCO::new()
        .talk_with(
            "Hello World, We are getting rusty with Rocket and Vonage!".to_string(),
            compose!(
                event_method(EventMethod::POST),
                event_url(format!("{}/voice/event", public_url.clone())),
                premium(true)
            ),
        )
        .connect_app(USER_TO_CALL.into())
        // .connect_websocket(format!("{}/echo", public_url.clone()).replace("http", "ws"), AudioFormat::L16_16K)
        .conversation("CON-14".into());

    log::info!("NCCO: {:?}", ncco);

    Json(ncco)
}

#[post("/voice/event", data = "<event>")]
fn event_server_call(event: Json<CallEventPayload>) {
    log::info!("Event Payload: {:?}", event);
}

#[get("/echo")]
fn echo(ws: rocket_ws::WebSocket) -> rocket_ws::Channel<'static> {
    use rocket::futures::{SinkExt, StreamExt};

    ws.channel(move |mut stream| Box::pin(async move {
        while let Some(message) = stream.next().await {
            match message {
                Ok(Message::Text(message_text)) => {
                    log::info!("Received message: {}", message_text);
                }
                Ok(Message::Binary(message_audio)) => {
                    log::info!("Received binary message");
                    stream.send(Message::Binary(message_audio)).await.unwrap();
                }
                Ok(Message::Close(_)) => {
                    log::info!("Connection closed");
                    break;
                }
                Ok(msg) => {
                    log::info!("Received message: {:?}", msg);
                }
                Err(e) => {
                    log::error!("Error: {:?}", e);
                    break;
                }
            }
        }
        Ok(())
    }))
}


struct PublicUrl(Option<String>);

#[launch]
fn rocket() -> _ {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    rocket::build()
        .manage(PublicUrl(std::env::var("PUBLIC_URL").ok()))
        .mount("/", routes![answer_server_call, event_server_call, echo])
}
