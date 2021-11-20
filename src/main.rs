extern crate pretty_env_logger;
#[macro_use]
extern crate rocket;

use envconfig::Envconfig;
use rocket::http::Status;
use rocket::State;
use teloxide::Bot;
use teloxide::prelude::{AutoSend, Requester, RequesterExt};

#[derive(Envconfig)]
struct AppConfig {
    #[envconfig(from = "BOT_TOKEN")]
    pub bot_token: String,
    #[envconfig(from = "CHAT_ID")]
    pub chat_id: i64,
}

#[post("/notify?<message>")]
async fn notify(message: Option<String>, bot: &State<AutoSend<Bot>>, config: &State<AppConfig>) -> Status {
    let text = match message {
        Some(value) => value,
        None => "Done".to_string(),
    };
    match bot.send_message(config.chat_id, text).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    teloxide::enable_logging!();

    let config: AppConfig = AppConfig::init_from_env().unwrap();

    let bot = Bot::new(&config.bot_token).auto_send();

    rocket::build()
        .manage(bot)
        .manage(config)
        .mount("/", routes![notify])
        .launch()
        .await
}