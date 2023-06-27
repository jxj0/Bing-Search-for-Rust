use std::env;
use std::ops::Index;
use reqwest::{self, header::ACCEPT};
use serde_json::{Result, Value};
use serde_json::Value::Object;
use std::io::{stdin, stdout, Write};


const AUTHORIZATION_HEADER: &str = "Ocp-Apim-Subscription-Key";

#[tokio::main]
async fn main() {
    // Construct a request
    let client = reqwest::Client::new();

    loop {
        // export BING_SEARCH_V7_SUBSCRIPTION_KEY="xxxxxx" 
        let subscription_key = env::var("BING_SEARCH_V7_SUBSCRIPTION_KEY").unwrap();
        // export BING_SEARCH_V7_ENDPOINT="xxxxxx"
        let endpoint = env::var("BING_SEARCH_V7_ENDPOINT").unwrap();
        let mkt = "en-US";


        let query = input("Query: ");

        let url = format!(
            "{endpoint}/v7.0/search?q={query}&mkt={mkt}"
        );
        
        let response = client.get(url)
        .header(AUTHORIZATION_HEADER, subscription_key)
        .header(ACCEPT, "*/*")
        .send()
        .await
        .unwrap()
        .text()
        .await;

        match response {
            Ok(resp) => {
                println!("|=============================================================|");
                untyped_json(resp).unwrap();
                println!("|=============================================================|");
            },
            _ => {
                println!("Uh oh! It failed!");
            }
        }
    }
        
}

fn untyped_json(payload: String) -> Result<()> {
    let data = payload; 
    let v: Value = serde_json::from_str(&data)?;

    for num in 0.. {
        match &v["webPages"]["value"][num] {
            Object(obj) => {
                println!("name: {}", &obj.index("name"));
                println!("Url: {}\n", &obj.index("displayUrl"));
                
            },
            _ => break,
        }
    }
    Ok(())
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    let mut input = String::new();

    stdout().flush().expect("Failed to flush stdout");
    stdin().read_line(&mut input).expect("Failed to read line");
    input.pop();
    return input;
}