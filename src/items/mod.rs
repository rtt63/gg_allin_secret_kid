use std::collections::HashMap;

use crate::create_actions;
use crate::ActionId;
use crate::EntityData;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ItemId {
    Console,
    Imac,
    ArmChair,
    BottleOfWatter,
    Drums,
    GuitarAmp,
    Mic,
    NoItem,
}

struct StudioGear {
    mix_console: EntityData,
    imac: EntityData,
    arm_chair: EntityData,
    bottle_of_water: EntityData,
    drums: EntityData,
    guitar_amp: EntityData,
    mic: EntityData,
}

fn get_gear() -> StudioGear {
    let mix_console: EntityData = EntityData {
        title: "Mixing console".to_owned(),
        allowed_actions: create_actions(vec![
            ActionId::SpillBeerOnTheConsole,
            ActionId::PushRandomButton,
            ActionId::MakeAllFadersUp,
            ActionId::StaringAtTheDeskWithTheDumbFace,
            ActionId::DropItem,
        ]),
    };
    let imac: EntityData = EntityData {
        title: "iMac".to_owned(),
        allowed_actions: create_actions(vec![
            ActionId::TurnOffImac,
            ActionId::BreakScreenOfImac,
            ActionId::CloseOpenedProToolsSession,
            ActionId::TouchImacDisplayWithFinger,
            ActionId::DropItem,
        ]),
    };
    let arm_chair: EntityData = EntityData {
        title: "Arm chair".to_owned(),
        allowed_actions: create_actions(vec![
            ActionId::PutLegsOnTheChair,
            ActionId::KickChairFromSoundGuy,
            ActionId::DropItem,
        ]),
    };
    let bottle_of_water = EntityData {
        title: "Bottle of watter".to_owned(),
        allowed_actions: create_actions(vec![
            ActionId::DrinkFromBottle,
            ActionId::ThrowBottleToTheAssistant,
            ActionId::MakeHoleInBottleAndGetHigh,
            ActionId::DropItem,
        ]),
    };
    let guitar_amp = EntityData {
        title: "Guitar amp".to_owned(),
        allowed_actions: create_actions(vec![
            ActionId::TurnVolumeToEleven,
            ActionId::AllButtonsUp,
            ActionId::PressStandBy,
            ActionId::DropItem,
        ]),
    };
    let drums = EntityData {
        title: "Drums".to_owned(),
        allowed_actions: create_actions(vec![
            ActionId::HitCymbalsRandomly,
            ActionId::ScreamAtTheSnare,
            ActionId::DropItem,
        ]),
    };
    let mic = EntityData {
        title: "Mic".to_owned(),
        allowed_actions: create_actions(vec![
            ActionId::ThrowMicAtTheWall,
            ActionId::CaughtForThreeMinutesStraight,
            ActionId::SwearAtTheCleaner,
            ActionId::DropItem,
        ]),
    };
    let studio_gear = StudioGear {
        mix_console,
        imac,
        arm_chair,
        bottle_of_water,
        guitar_amp,
        drums,
        mic,
    };

    studio_gear
}

pub fn get_items() -> HashMap<ItemId, EntityData> {
    let studio_gear = get_gear();
    let mut gear_map: HashMap<ItemId, EntityData> = HashMap::new();
    gear_map.insert(ItemId::Console, studio_gear.mix_console);
    gear_map.insert(ItemId::Imac, studio_gear.imac);
    gear_map.insert(ItemId::Drums, studio_gear.drums);
    gear_map.insert(ItemId::GuitarAmp, studio_gear.guitar_amp);
    gear_map.insert(ItemId::ArmChair, studio_gear.arm_chair);
    gear_map.insert(ItemId::BottleOfWatter, studio_gear.bottle_of_water);
    gear_map.insert(ItemId::Mic, studio_gear.mic);

    gear_map
}
