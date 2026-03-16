use crate::State;
use azalea::{
    core::game_type::GameMode,
    ecs::prelude::*,
    entity::{metadata::Player, Dead, LocalEntity},
    inventory::{components, default_components, operations, ItemStack},
    local_player::LocalGameMode,
    prelude::*,
};

pub fn tick(bot: &Client, _state: &State) -> anyhow::Result<()> {
    look_at_nearest_player(&bot)?;
    eat(&bot)?;
    Ok(())
}

fn look_at_nearest_player(bot: &Client) -> anyhow::Result<()> {
    let nearest_player = bot
        .nearest_entity_by::<(), (With<Player>, Without<LocalEntity>, Without<Dead>)>(|_: ()| true);
    if let Some(player) = nearest_player {
        player.look_at();
    }
    Ok(())
}

pub fn eat(bot: &Client) -> anyhow::Result<()> {
    if bot.component::<LocalGameMode>().current != GameMode::Survival || bot.hunger().food >= 20 {
        return Ok(());
    }
    let inventory = bot.open_inventory().unwrap();
    let food_slot: u8 = 4;
    if let ItemStack::Present(food_slot_item) = inventory.menu().unwrap().slot(food_slot as usize).unwrap() &&
        default_components::get_default_component::<components::Food>(food_slot_item.kind).is_some()
    {} else {
        for (index, slot) in inventory.menu().unwrap().slots().iter().enumerate() {
            let ItemStack::Present(item) = slot else {
                continue;
            };
            if default_components::get_default_component::<components::Food>(item.kind).is_some() {
                inventory.click(operations::SwapClick {
                    source_slot: index as u16,
                    target_slot: food_slot,
                });
                break;
            }
        }
    }
    inventory.close();
    bot.set_selected_hotbar_slot(food_slot);
    bot.start_use_item();
    Ok(())
}
