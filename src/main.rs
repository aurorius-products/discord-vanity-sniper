#[macro_use] extern crate lazy_static;
use anyhow::{Ok, Result};
use regex::Regex;
use std::{fs::File, io::{Read, Write}, path::Path};
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref RE: Regex = Regex::new(r#"og:description" content="([^"]*)""#).expect("Invalid eegex");
}

#[derive(Serialize, Deserialize)]
struct Config {
    discord_token: String,
    discord_id: String,
    vanities: Vec<String>
}


async fn is_vanity_taken(vanity: &str) -> Result<bool> {
    let body = reqwest::get(format!("https://discord.com/invite/{}", vanity))
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
            println!("Couldn't find metadata for {}", vanity);
            return Ok(true)
        }
    }
}

fn get_config() -> Result<Config> {
    let config_path = Path::new("config.json");
    
    if !config_path.exists() {
        let config = Config {
            discord_token: "".to_string(),
            discord_id: "".to_string(),
            vanities: Vec::new()
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

#[tokio::main]
async fn main() -> Result<()> {
    let config = get_config()?;
    if config.discord_token == "" || config.vanities.len() == 0 || config.discord_id == "" {
        println!("Discord token, discord server id or vanities not set! See config.json.");
        return Ok(());
    }

    Ok(())
}
