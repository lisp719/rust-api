use dotenvy::dotenv;
use serde_json::json;
use std::{env, io::stdin};

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    dotenv().expect(".env file not found");

    let api_key = env::var("SENDGRID_API_KEY").unwrap();
    let client = reqwest::Client::new();

    let base_url = "https://api.sendgrid.com/v3";
    let url = format!("{}/mail/send", base_url);

    let mut email = String::new();

    println!("Please enter your email address");
    stdin().read_line(&mut email).unwrap();

    let value = json!({
        "personalizations": [{
            "to": [{
                "email": email.trim_end()
            }],
            "subject": "Hello, World!"
        }],
        "from": {
            "email": "from_address@example.com"
        },
        "content": [{
            "type": "text/plain",
            "value": "Hello, World!"
        }]
    });

    let body = client
        .post(url)
        .bearer_auth(api_key)
        .json(&value)
        .send()
        .await?
        .text()
        .await?;

    println!("body = {:?}", body);

    Ok(())
}
