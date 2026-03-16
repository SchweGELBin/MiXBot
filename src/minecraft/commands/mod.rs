mod movement;
use crate::minecraft::State;
use azalea::{
    brigadier::prelude::*, chat::ChatPacket, ecs::prelude::*, entity::metadata::Player,
    player::GameProfileComponent, Client,
};
use parking_lot::Mutex;

pub type Ctx = CommandContext<Mutex<CommandSource>>;

pub struct CommandSource {
    pub bot: Client,
    pub state: State,
    pub chat: ChatPacket,
}

impl CommandSource {
    pub fn reply(&self, message: impl Into<String>) {
        let message = message.into();
        if self.chat.is_whisper() {
            self.bot
                .chat(format!("/w {} {message}", self.chat.sender().unwrap()));
        } else {
            self.bot.chat(message);
        }
    }

    pub fn entity(&self) -> Option<azalea::EntityRef> {
        let username = self.chat.sender()?;
        self.bot
            .any_entity_by::<&GameProfileComponent, With<Player>>(
                |profile: &GameProfileComponent| profile.name == username,
            )
    }
}

pub fn register_commands(commands: &mut CommandDispatcher<Mutex<CommandSource>>) {
    movement::register(commands);
}
