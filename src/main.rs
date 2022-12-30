use dotenv::dotenv;
use std::env;
use openapi::apis::{configuration::Configuration, default_api as twilio_api};


#[tokio::main]
async fn main() {
  dotenv().expect("Error reading .env file");
  let account_sid = env::var("TWILIO_ACCOUNT_SID").expect("Error reading Twilio Account SID");
  let api_key_sid = env::var("TWILIO_API_KEY_SID").expect("Error reading Twilio API key");
  let api_key_secret = env::var("TWILIO_API_KEY_SECRET").expect("Error reading Twilio API SID");
  let from = env::var("TWILIO_PHONE_NUMBER").expect("Error reading Twilio Phone Number");
  let to = env::var("TO_NUMBER").expect("Error reading outbound phone number");

  // Create a new, default client configuration.
  let mut twilio_config = Configuration::default();
  // Apply your Basic Auth credentials to the client configuration.
  twilio_config.basic_auth = Some((api_key_sid, Some(api_key_secret)));

  let body = "Ce soir, il n'était pas très inspiré. Mais demain est un AUTRE jour ! ".to_string();
  
  let message = twilio_api::create_message(
    &twilio_config,
    &account_sid,
    &to,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(&from),
    std::option::Option::None,
    Some(&body), 
    std::option::Option::None
  )
  .await;

  let result = match message {
    Ok(result) => result,
    Err(error) => panic!("Something went wrong, {:?}", error),
  };

  println!("{:?}", result.sid);
}
