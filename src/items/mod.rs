use std::collections::HashMap;

use crate::create_actions;
use crate::ActionId;
use crate::ItemData;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Item {
    Console,
    Imac,
    // ArmChair,
    // BottleOfWatter,
    // Drums,
    // GuitarAmp,
    // Mic,
    NoItem,
}

struct StudioGear {
    mix_console: ItemData,
    imac: ItemData,
    // arm_chair: ItemData,
    // bottle_of_water: ItemData,
    // drums: ItemData,
    // guitar_amp: ItemData,
    // mic: ItemData,
}

fn get_gear() -> StudioGear {
    let mix_console: ItemData = ItemData {
        title: "Mixing console".to_owned(),
        allowed_actions: create_actions(vec![
            ActionId::SpillBeerOnTheConsole,
            ActionId::PushRandomButton,
            ActionId::MakeAllFadersUp,
            ActionId::StaringAtTheDeskWithTheDumbFace,
            ActionId::DropItem,
        ]),
    };
    let imac: ItemData = ItemData {
        title: "iMac".to_owned(),
        allowed_actions: create_actions(vec![
            ActionId::TurnOffImac,
            ActionId::BreakScreenOfImac,
            ActionId::CloseOpenedProToolsSession,
            ActionId::TouchImacDisplayWithFinger,
            ActionId::DropItem,
        ]),
    };
    let studio_gear = StudioGear { mix_console, imac };

    studio_gear
}

pub fn get_items() -> HashMap<Item, ItemData> {
    let studio_gear = get_gear();
    let mut current_item_to_item_data: HashMap<Item, ItemData> = HashMap::new();
    current_item_to_item_data.insert(Item::Console, studio_gear.mix_console);
    current_item_to_item_data.insert(Item::Imac, studio_gear.imac);
    current_item_to_item_data
}
