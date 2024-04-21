#[macro_use] extern crate lazy_static;
use anyhow::{Ok, Result};
use regex::Regex;
use std::{fs::File, io::{Read, Write}, path::Path};
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref RE: Regex = Regex::new(r#"og:description" content="([^"]*)""#).expect("Invalid regex");
}

#[derive(Serialize, Deserialize)]
struct Config {
    discord_token: String,
    discord_id: String,
    vanities: Vec<String>,
    workers_each: u16
}

#[derive(Serialize, Deserialize)]
struct SetVanity {
    code: String
}

#[derive(Serialize, Deserialize)]
struct SetVanityResponse {
    code: Option<u16>,
    message: Option<String>,

}


async fn is_vanity_taken(client: &reqwest::Client, vanity: &str) -> Result<bool> {

    let body = client.get(format!("https://discord.com/invite/{}", vanity))
        .send()
        .await?
        .text()
        .await?;

    match RE.captures(body.as_str()) {
        Some(captures) => {
            let description = format!("{}", captures.get(1).map_or("", |m| m.as_str()));
            if description.starts_with("Discord is the easiest way") {
                return Ok(false)
            }
            return Ok(true)
        },
        None => {
            println!("Couldn't find metadata for {}. Response: {}", vanity, body);
            return Ok(true)
        }
    }
}

async fn set_vanity(vanity: &str, discord_token: &str, discord_id: &str) -> Result<bool> {
    let request_body_json = SetVanity {
        code: vanity.to_string()
    };
    let request_body = serde_json::to_string(&request_body_json)?;

    let client = reqwest::Client::new();
    let response = client.patch(format!("https://discord.com/api/v9/guilds/{}/vanity-url", discord_id))
        .header("Authorization", discord_token)
        .header("Content-Type", "application/json")
        .body(request_body)
        .send()
        .await?;

    if response.status().as_u16() == 200 {
        return Ok(true);
    }

    Ok(false)
}

fn get_config() -> Result<Config> {
    let config_path = Path::new("config.json");
    
    if !config_path.exists() {
        let config = Config {
            discord_token: "".to_string(),
            discord_id: "".to_string(),
            vanities: Vec::new(),
            workers_each: 1
        };
        let j = serde_json::to_string(&config)?;

        let mut file = File::create_new("config.json")?;
        file.write_all(j.as_bytes())?;

        return Ok(config);
    }

    let mut file = File::open("config.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let config: Config = serde_json::from_str(contents.as_str())?;

    Ok(config)
}

async fn run(vanity: &str, discord_token: &str, discord_id: &str) -> Result<()> {
    let client = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36")
        .build()?;

    loop {
        if !is_vanity_taken(&client, vanity).await? {
            set_vanity(vanity, discord_token, discord_id).await?;
        }
        println!("Checked discord.gg/{}", vanity.to_string());
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = get_config()?;
    if config.discord_token == "" || config.vanities.len() == 0 || config.discord_id == "" || config.workers_each < 1 {
        println!("Discord token, discord server id or vanities not set! See config.json.");
        return Ok(());
    }

    let mut handles = Vec::new();

    for vanity in config.vanities {
        let discord_token = config.discord_token.clone();
        let discord_id = config.discord_id.clone();

        for _ in 0..config.workers_each {
            let task_vanity = vanity.clone();
            let task_token = discord_token.clone();
            let task_id = discord_id.clone();

            let handle = tokio::spawn(async move {
                let _ = run(task_vanity.as_str(), task_token.as_str(), task_id.as_str()).await;
            });
            handles.push(handle);
        }
    }

    for handle in handles {
        handle.await.unwrap();
    }

    Ok(())
}
