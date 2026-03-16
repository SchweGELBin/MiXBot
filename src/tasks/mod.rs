mod idle;
use crate::State;
use azalea::Client;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum BotTask {
    #[default]
    None,
    Guard,
}

pub fn tick(bot: Client, state: State) -> anyhow::Result<()> {
    let task = *state.task.lock();
    match task {
        BotTask::Guard => {}
        _ => {
            idle::tick(&bot, &state)?;
        }
    }
    Ok(())
}
