use std::collections::HashMap;

fn greeting() {
    println!("It’s 2024, and you found an app that questions you about your genealogical tree and says it 99% chance you’re the secret child of GG Allin. Now you are ready to become a punk rock star (what else could you do in this situation, right?).\n\nYou found a used Squier Bullet and asked some lads next to the pub if they are ready to be in a band. You don’t remember what happened next, but now you’re going to the musical studio to record your first LP!\n\nDon’t screw it up!");
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Room {
    ControlRoom,
    ToneRoom,
    Hall,
    VoiceBooth,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Item {
    Console,
    Imac,
    // ArmChair,
    // BottleOfWatter,
    // Drums,
    // GuitarAmp,
    // Mic,
    NoItem,
}

fn call_action(
    action_id: &ActionId,
    current_room: &mut Room,
    score: &mut i8,
    current_item: &mut Item,
) {
    match action_id {
        ActionId::GoToControlRoom => {
            *current_room = Room::ControlRoom;
            println!("You're in the control room");
        }
        ActionId::GoToToneRoom => {
            *current_room = Room::ToneRoom;
            println!("You're in the tone room");
        }
        ActionId::GoToHall => {
            *current_room = Room::Hall;
            println!("You're in the hall");
        }
        ActionId::GoToVocalBooth => {
            *current_room = Room::VoiceBooth;
            println!("You're in the voice booth. It's so scary! No way!!! You're going back");
            *current_room = Room::ToneRoom;
            println!("You're in the tone room");
        }
        ActionId::SpillBeerOnTheConsole => {
            println!("Well... don't do that again");
            *score -= 2;
        }
        ActionId::PickConsole => {
            *current_item = Item::Console;
        }
        ActionId::PushRandomButton => {
            println!("Wow! That's mixing!");
            *score += 1;
        }
        ActionId::MakeAllFadersUp => {
            println!("You're the greatest punk rocker of the universe!");
            *score += 2;
        }
        ActionId::StaringAtTheDeskWithTheDumbFace => {
            println!("Well.. it least you're not hurting anyone");
            *score += 1;
        }
        ActionId::PickImac => {
            *current_item = Item::Imac;
        }
        ActionId::TurnOffImac => {
            println!("This Little Maneuver's Gonna Cost Us 51 Years");
            *score -= 2;
        }
        ActionId::BreakScreenOfImac => {
            println!("And how you wanted to complete the session? Game over, go away");
            *score = -127;
            std::process::exit(0);
        }
        ActionId::CloseOpenedProToolsSession => {
            println!("This Little Maneuver's Gonna Cost Us 51 Years");
            *score -= 1;
        }
        ActionId::TouchImacDisplayWithFinger => {
            println!("And why sound guy is screaming?");
        }
        ActionId::DropItem => {
            *current_item = Item::NoItem;
            println!("Item dropped");
        }
        ActionId::ExitGame => {
            println!("Bye!");
            std::process::exit(0);
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum ActionId {
    GoToControlRoom,
    GoToHall,
    GoToToneRoom,
    GoToVocalBooth,

    SpillBeerOnTheConsole,

    PickConsole,
    PushRandomButton,
    MakeAllFadersUp,
    StaringAtTheDeskWithTheDumbFace,

    PickImac,
    TurnOffImac,
    BreakScreenOfImac,
    CloseOpenedProToolsSession,
    TouchImacDisplayWithFinger,

    DropItem,

    ExitGame,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct ItemData {
    title: String,
    allowed_actions: Vec<ActionId>,
}

fn handle_user_input() -> usize {
    let mut input: String = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Unable to read input");

    let return_value: usize = input.trim().parse().unwrap();

    return_value
}

fn create_actions(action_ids: Vec<ActionId>) -> Vec<ActionId> {
    let mut upd_vec = action_ids.clone();
    upd_vec.push(ActionId::ExitGame);

    upd_vec
}

fn main() {
    std::process::Command::new("clear").status().unwrap();
    let mut score: i8 = 0;

    let mut action_id_to_string: HashMap<ActionId, String> = HashMap::new();
    action_id_to_string.insert(ActionId::GoToControlRoom, "Go to control room".to_owned());
    action_id_to_string.insert(ActionId::GoToHall, "Go to hall".to_owned());
    action_id_to_string.insert(ActionId::GoToVocalBooth, "Go to vocal booth".to_owned());
    action_id_to_string.insert(ActionId::GoToToneRoom, "Go to tone room".to_owned());

    let mut current_room: Room = Room::Hall;
    let mut current_item: Item = Item::NoItem;

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
    let mut current_room_to_room_data: HashMap<Room, ItemData> = HashMap::new();
    current_room_to_room_data.insert(Room::ControlRoom, control_room);
    current_room_to_room_data.insert(Room::Hall, hall);
    current_room_to_room_data.insert(Room::ToneRoom, tone_room);
    current_room_to_room_data.insert(Room::VoiceBooth, vocal_booth);

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
    let mut current_item_to_item_data: HashMap<Item, ItemData> = HashMap::new();
    current_item_to_item_data.insert(Item::Console, mix_console);
    current_item_to_item_data.insert(Item::Imac, imac);
    greeting();

    loop {
        let current_room_data = current_room_to_room_data.get(&current_room);
        let current_item_data = current_item_to_item_data.get(&current_item);

        let mut index_to_action_id: HashMap<usize, &ActionId> = HashMap::new();

        if current_item != Item::NoItem {
            match current_item_data {
                Some(data) => {
                    println!("Availabe actions with {}:\n", data.title);
                    for (i, action_id) in data.allowed_actions.iter().enumerate() {
                        index_to_action_id.insert(i, &action_id);
                        println!("{} -> {:?}", i, &action_id);
                    }
                    let user_input = handle_user_input();
                    call_action(
                        index_to_action_id.get(&user_input).unwrap(),
                        &mut current_room,
                        &mut score,
                        &mut current_item,
                    );
                }
                None => {}
            }
        } else {
            match current_room_data {
                Some(data) => {
                    println!("Availabe actions in {}:\n", data.title);
                    for (i, action_id) in data.allowed_actions.iter().enumerate() {
                        index_to_action_id.insert(i, &action_id);
                        println!("{} -> {:?}", i, &action_id);
                    }
                    let user_input = handle_user_input();
                    call_action(
                        index_to_action_id.get(&user_input).unwrap(),
                        &mut current_room,
                        &mut score,
                        &mut current_item,
                    );
                }
                None => {}
            }
        }

        if score > -10 && score < -7 {
            println!("Why so many angry people around?\n");
        }

        if score <= -10 {
            println!("It is over, you're not going to be a star T_T");
            break;
        }
        if score >= 10 {
            println!("You are truly GG Allin Secret Kid... Pure legend...");
            break;
        }
    }
}
