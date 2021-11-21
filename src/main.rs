extern crate pretty_env_logger;
#[macro_use]
extern crate rocket;

use async_trait::async_trait;
use envconfig::Envconfig;
use rocket::{Request, request, State};
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use teloxide::Bot;
use teloxide::prelude::{AutoSend, Requester, RequesterExt};

#[derive(Envconfig)]
struct AppConfig {
    #[envconfig(from = "BOT_TOKEN")]
    pub bot_token: String,
    #[envconfig(from = "CHAT_ID")]
    pub chat_id: i64,
    #[envconfig(from = "AUTH_TOKEN")]
    pub auth_token: String,
}

struct Token(String);

#[derive(Debug)]
enum ApiTokenError {
    Missing,
    Invalid,
}

#[async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ApiTokenError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");
        match token {
            Some(token) => {
                if !token.starts_with("Bearer ") {
                    return Outcome::Failure((Status::Unauthorized, ApiTokenError::Invalid));
                }
                Outcome::Success(Token(token.strip_prefix("Bearer ").unwrap().to_string()))
            }
            None => Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing)),
        }
    }
}

#[post("/notify?<message>")]
async fn notify(message: Option<String>, token: Token, bot: &State<AutoSend<Bot>>, config: &State<AppConfig>) -> Status {
    if token.0 != config.auth_token {
        return Status::Unauthorized;
    }
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