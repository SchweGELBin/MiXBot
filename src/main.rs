mod commands;
mod tasks;

use azalea::{
    ClientInformation,
    brigadier::command_dispatcher::CommandDispatcher,
    prelude::*,
    swarm::prelude::*
};
use commands::{CommandSource, register_commands};
use parking_lot::Mutex;
use std::{
    env::{args,var},
    sync::Arc,
    time::Duration
};
use tasks::BotTask;

#[tokio::main]
pub async fn main() -> AppExit {
    let args = parse_args();

    let mut builder = SwarmBuilder::new()
        .set_handler(handle)
        .set_swarm_handler(swarm_handle);

    for acc in &args.accounts {
        let account = if acc.1 {
            Account::microsoft(&acc.0).await.unwrap()
        } else {
            Account::offline(&acc.0)
        };
        builder = builder.add_account_with_state(account, State::new());
    };
    let host = args.host.clone();

    let mut commands = CommandDispatcher::new();
    register_commands(&mut commands);

    builder
        .join_delay(Duration::from_millis(100))
        .set_swarm_state(SwarmState{
            args,
            commands: Arc::new(commands),
        })
        .start(host)
        .await
}

#[derive(Clone, Component, Default)]
pub struct State {
    pub task: Arc<Mutex<BotTask>>,
}

impl State {
    fn new() -> Self {
        Self {
            task: Arc::new(Mutex::new(BotTask::None)),
        }
    }
}

#[derive(Clone, Default, Resource)]
struct SwarmState {
    pub args: Args,
    pub commands: Arc<CommandDispatcher<Mutex<CommandSource>>>,
}

async fn handle(bot: Client, event: azalea::Event, state: State) -> anyhow::Result<()> {
    let swarm = bot.resource::<SwarmState>();
    match event {
        azalea::Event::Init => {
            bot.set_client_information(ClientInformation{
                view_distance: 16,
                ..Default::default()
            });
        }
        azalea::Event::Chat(chat) => {
            let (Some(username), content) = chat.split_sender_and_content() else { return Ok(()); };
            if !swarm.args.owner.is_empty() && username != swarm.args.owner { return Ok(()); };

            let command = if chat.is_whisper() { Some(content) } else { content.strip_prefix(swarm.args.prefix).map(|s| s.to_owned()) };
            if let Some(command) = command {
                match swarm.commands.execute(command, Mutex::new(CommandSource{
                    bot: bot.clone(),
                    chat: chat.clone(),
                    state: state.clone()
                })) {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("{err:?}");
                        let command_source = CommandSource{ bot, chat:chat.clone(), state:state.clone() };
                        command_source.reply(format!("{err:?}"));
                    }
                }
            }
        }
        azalea::Event::Tick => {
            tasks::tick(bot, state)?;
        }
        azalea::Event::Login => {
            println!("Logging in...");
        }
        _ => {}
    }

    Ok(())
}

async fn swarm_handle(_swarm: Swarm, event: SwarmEvent, _state: SwarmState) -> anyhow::Result<()> {
    match &event {
        SwarmEvent::Disconnect(account, _join_opts) => {
            println!("{} disconnected.", account.username());
        }
        SwarmEvent::Chat(chat) => {
            if chat.message().to_string() == "The particle was not visible for anybody" {
                return Ok(());
            }
            println!("{}", chat.message().to_ansi());
        }
        _ => {}
    }

    Ok(())
}

#[derive(Clone, Debug, Default)]
pub struct Args {
    pub accounts: Vec<(String, bool)>,
    pub host: String,
    pub owner: String,
    pub prefix: char,
}

fn parse_args() -> Args {
    let mut accounts = parse_accounts(var("MIXBOT_ACCOUNTS").unwrap_or("MiXBot,false".to_string()));
    let mut host = var("MIXBOT_HOST").unwrap_or("localhost".to_string());
    let mut owner = var("MIXBOT_OWNER").unwrap_or(String::new());
    let mut prefix = var("MIXBOT_PREFIX").unwrap_or("!".to_string());

    let mut args = args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-A" | "--accounts" => {
                accounts = parse_accounts(args.next().expect("Missing accounts argument"));
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
            _ => {}
        }
    }

    let prefix = prefix.chars().next().expect("Weird prefix");

    Args{
        accounts,
        host,
        owner,
        prefix,
    }
}

fn parse_accounts(arg: String) -> Vec<(String, bool)> {
    let mut accounts: Vec<(String, bool)> = Vec::new();
    for account in arg.split(';') {
        let mut parts = account.split(',');
        let name = parts.next().unwrap().to_string();
        let online = match parts.next().unwrap_or("false") {
            "true" | "1" => true,
            _ => false,
        };
        accounts.push((name, online));
    };
    accounts
}
