mod discord;
mod minecraft;
use std::env::{args, var};

#[tokio::main]
async fn main() {
    tokio::join!(discord::start(), minecraft::start());
}

#[derive(Clone, Debug, Default)]
pub struct Args {
    pub accounts: Vec<(String, bool)>,
    pub host: String,
    pub owner: String,
    pub prefix: char,
    pub dcid: String,
    pub dcprefix: char,
    pub dctoken: String
}

fn parse_args() -> Args {
    let mut accounts = minecraft::parse_accounts(var("MIXBOT_ACCOUNTS").unwrap_or("MiXBot,false".to_string()));
    let mut host = var("MIXBOT_HOST").unwrap_or("localhost".to_string());
    let mut owner = var("MIXBOT_OWNER").unwrap_or(String::new());
    let mut prefix = var("MIXBOT_PREFIX").unwrap_or("!".to_string());
    let mut dcid = var("MIXBOT_DISCORD_ID").unwrap_or(String::new());
    let mut dcprefix = var("MIXBOT_DISCORD_PREFIX").unwrap_or("!".to_string());
    let mut dctoken = var("MIXBOT_DISCORD_TOKEN").unwrap_or(String::new());

    let mut args = args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-A" | "--accounts" => {
                accounts = minecraft::parse_accounts(args.next().expect("Missing accounts argument"));
            }
            "-H" | "--host" => {
                host = args.next().expect("Missing host argument");
            }
            "-O" | "--owner" => {
                owner = args.next().expect("Missing owner argument");
            }
            "-P" | "--prefix" => {
                prefix = args.next().expect("Missing prefix argument");
            }
            "--dcid" => {
                dcid = args.next().expect("Missing dcid argument");
            }
            "--dcprefix" => {
                dcprefix = args.next().expect("Missing dcprefix argument");
            }
            "--dctoken" => {
                dctoken = args.next().expect("Missing dctoken argument");
            }
            _ => {}
        }
    }

    let prefix = prefix.chars().next().expect("Weird prefix");
    let dcprefix = dcprefix.chars().next().expect("Weird dcprefix");

    Args{
        accounts,
        host,
        owner,
        prefix,
        dcid,
        dcprefix,
        dctoken
    }
}
