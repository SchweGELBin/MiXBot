use super::{CommandSource, Ctx};
use azalea::brigadier::prelude::*;
use parking_lot::Mutex;

pub fn register(commands: &mut CommandDispatcher<Mutex<CommandSource>>) {
    commands.register(
        literal("jump")
            .executes(|ctx: &Ctx| {
                let source = ctx.source.lock();
                source.bot.jump();
                source.reply("ok");
                1
            })
            .then(argument("enabled", bool()).executes(|ctx: &Ctx| {
                let jumping = get_bool(ctx, "enabled").unwrap();
                let source = ctx.source.lock();
                source.bot.set_jumping(jumping);
                1
            })),
    );
}
