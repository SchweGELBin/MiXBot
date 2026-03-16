#![warn(clippy::str_to_string)]

mod commands;
use crate::Args;
use poise::serenity_prelude as serenity;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    test: Mutex<()>,
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Discord | Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Discord   | Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Discord   | Error while handling error: {}", e)
            }
        }
    }
}

pub async fn start() {
    let args = crate::parse_args();
    if !args.dctoken.is_empty() {
        run(args).await;
    }
}

pub async fn run(args: Args) {
    tracing_subscriber::fmt::init();

    let options = poise::FrameworkOptions {
        commands: vec![commands::help(), commands::join(), commands::leave(), commands::msg(), commands::ping(), commands::status(), commands::version()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(args.dcprefix.to_string()),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3600),
            ))),
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Discord   | Executing command {}...", ctx.command().qualified_name);
            })
        },
        post_command: |ctx| {
            Box::pin(async move {
                println!("Discord   | Executed command {}!", ctx.command().qualified_name);
            })
        },
        command_check: Some(|ctx| {
            Box::pin(async move {
                let id = crate::parse_args().dcid;
                if id.is_empty() || ctx.author().id.to_string() == id {
                    return Ok(true);
                }
                Ok(false)
            })
        }),
        skip_checks_for_owners: false,
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                println!(
                    "Discord   | Got an event in event handler: {:?}",
                    event.snake_case_name()
                );
                Ok(())
            })
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                println!("Discord   | Logged in as {}", ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    test: Mutex::new(()),
                })
            })
        })
        .options(options)
        .build();

    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::ClientBuilder::new(args.dctoken, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap()
}
