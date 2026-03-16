use crate::discord::{Context, Error};

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This Bot can control the Minecraft MiXBot",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let res = "Not implemented yet";
    ctx.say(res).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let res = "Not implemented yet";
    ctx.say(res).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn msg(ctx: Context<'_>) -> Result<(), Error> {
    let res = "Not implemented yet";
    ctx.say(res).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let res = "Pong!";
    ctx.say(res).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn status(ctx: Context<'_>) -> Result<(), Error> {
    let res = "Not implemented yet";
    ctx.say(res).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn version(ctx: Context<'_>) -> Result<(), Error> {
    let res = format!("Current Bot Version: {} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    ctx.say(res).await?;
    Ok(())
}
