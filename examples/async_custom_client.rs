use frakti::client_cyper::Bot;
use frakti::AsyncTelegramApi;

static BASE_API_URL: &str = "https://api.telegram.org/bot";

fn custom_client() -> Bot {
    let token = std::env::var("BOT_TOKEN").expect("Should have BOT_TOKEN as environment variable");

    let client = frakti::cyper::ClientBuilder::new().build();
    let api_url = format!("{BASE_API_URL}{token}");

    Bot::builder().api_url(api_url).client(client).build()
}

#[compio::main]
async fn main() {
    let bot = custom_client();

    match bot.get_me().await {
        Ok(response) => {
            let user = response.result;
            println!(
                "Hello, I'm @{}, https://t.me/{}",
                user.first_name,
                user.username.expect("The bot must have a username.")
            );
        }
        Err(error) => {
            eprintln!("Failed to get me: {error:?}");
        }
    }
}
