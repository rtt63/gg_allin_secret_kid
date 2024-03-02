use std::collections::HashMap;

use crate::create_actions;
use crate::ActionId;
use crate::ItemData;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RoomId {
    ControlRoom,
    ToneRoom,
    Hall,
    VoiceBooth,
}

struct StudioPlan {
    hall: ItemData,
    control_room: ItemData,
    tone_room: ItemData,
    vocal_booth: ItemData,
}

fn get_studio_plan() -> StudioPlan {
    let hall: ItemData = ItemData {
        title: "Hall".to_owned(),
        allowed_actions: create_actions(vec![ActionId::GoToControlRoom]),
    };
    let control_room: ItemData = ItemData {
        title: "Control room".to_owned(),
        allowed_actions: create_actions(vec![
            ActionId::GoToHall,
            ActionId::GoToToneRoom,
            ActionId::GoToVocalBooth,
            ActionId::PickConsole,
            ActionId::PickImac,
        ]),
    };
    let tone_room: ItemData = ItemData {
        title: "Tone room".to_owned(),
        allowed_actions: create_actions(vec![ActionId::GoToControlRoom, ActionId::GoToVocalBooth]),
    };
    let vocal_booth: ItemData = ItemData {
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

pub fn get_studio() -> HashMap<RoomId, ItemData> {
    let studio_plan = get_studio_plan();
    let mut current_room_to_room_data: HashMap<RoomId, ItemData> = HashMap::new();
    current_room_to_room_data.insert(RoomId::ControlRoom, studio_plan.control_room);
    current_room_to_room_data.insert(RoomId::Hall, studio_plan.hall);
    current_room_to_room_data.insert(RoomId::ToneRoom, studio_plan.tone_room);
    current_room_to_room_data.insert(RoomId::VoiceBooth, studio_plan.vocal_booth);

    current_room_to_room_data
}
