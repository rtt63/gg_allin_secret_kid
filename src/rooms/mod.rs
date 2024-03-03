use std::collections::HashMap;

use crate::create_actions;
use crate::ActionId;
use crate::EntityData;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RoomId {
    ControlRoom,
    ToneRoom,
    Hall,
    VoiceBooth,
}

struct StudioPlan {
    hall: EntityData,
    control_room: EntityData,
    tone_room: EntityData,
    vocal_booth: EntityData,
}

fn get_studio_plan() -> StudioPlan {
    let hall: EntityData = EntityData {
        title: "Hall".to_owned(),
        allowed_actions: create_actions(vec![
            ActionId::GoToControlRoom,
            ActionId::SingToTheCureAndCry,
            ActionId::PressRandomNumbersOnAnIntercom,
            ActionId::StaringAtTheDoorForLoongTime,
            ActionId::PickBottle,
        ]),
    };
    let control_room: EntityData = EntityData {
        title: "Control room".to_owned(),
        allowed_actions: create_actions(vec![
            ActionId::GoToHall,
            ActionId::GoToToneRoom,
            ActionId::PickConsole,
            ActionId::PickImac,
            ActionId::SingToTheCureAndCry,
            ActionId::PickChair,
        ]),
    };
    let tone_room: EntityData = EntityData {
        title: "Tone room".to_owned(),
        allowed_actions: create_actions(vec![
            ActionId::GoToControlRoom,
            ActionId::GoToVocalBooth,
            ActionId::SingToTheCureAndCry,
            ActionId::PickMic,
            ActionId::PickDrums,
            ActionId::PickGuitarAmp,
        ]),
    };
    let vocal_booth: EntityData = EntityData {
        title: "Vocal booth".to_owned(),
        allowed_actions: create_actions(vec![ActionId::GoToControlRoom]),
    };

    let studio_plan: StudioPlan = StudioPlan {
        hall,
        control_room,
        tone_room,
        vocal_booth,
    };

    studio_plan
}

pub fn get_studio() -> HashMap<RoomId, EntityData> {
    let studio_plan = get_studio_plan();
    let mut studio_rooms_map: HashMap<RoomId, EntityData> = HashMap::new();
    studio_rooms_map.insert(RoomId::ControlRoom, studio_plan.control_room);
    studio_rooms_map.insert(RoomId::Hall, studio_plan.hall);
    studio_rooms_map.insert(RoomId::ToneRoom, studio_plan.tone_room);
    studio_rooms_map.insert(RoomId::VoiceBooth, studio_plan.vocal_booth);

    studio_rooms_map
}
