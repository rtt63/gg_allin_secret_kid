use std::collections::HashMap;

use dispatcher::call_action;
mod dispatcher;
mod items;
mod rooms;

use items::{get_items, Item};
use rooms::{get_studio, RoomId};

fn greeting() {
    println!("It’s 2024, and you found an app that questions you about your genealogical tree and says it 99% chance you’re the secret child of GG Allin. Now you are ready to become a punk rock star (what else could you do in this situation, right?).\n\nYou found a used Squier Bullet and asked some lads next to the pub if they are ready to be in a band. You don’t remember what happened next, but now you’re going to the musical studio to record your first LP!\n\nDon’t screw it up!");
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

fn handle_user_input() -> Result<usize, std::num::ParseIntError> {
    let mut input: String = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Unable to read input");

    let return_value: usize = input.trim().parse()?;

    Ok(return_value)
}

fn create_actions(action_ids: Vec<ActionId>) -> Vec<ActionId> {
    let mut upd_vec = action_ids.clone();
    upd_vec.push(ActionId::ExitGame);

    upd_vec
}

fn clear_console() -> () {
    std::process::Command::new("clear")
        .status()
        .expect("Unable to clear console");

    ()
}
fn main() {
    clear_console();
    let mut score: i8 = 0;

    let studio = get_studio();
    let mut current_room: RoomId = RoomId::Hall;

    let items = get_items();
    let mut current_item: Item = Item::NoItem;

    greeting();

    loop {
        let current_room_data = studio.get(&current_room);
        let current_item_data = items.get(&current_item);

        let mut index_to_action_id: HashMap<usize, &ActionId> = HashMap::new();

        if current_item != Item::NoItem {
            match current_item_data {
                Some(data) => {
                    println!("Availabe actions with {}:\n", data.title);
                    for (i, action_id) in data.allowed_actions.iter().enumerate() {
                        index_to_action_id.insert(i, &action_id);
                        println!("{} -> {:?}", i, &action_id);
                    }
                    match handle_user_input() {
                        Ok(user_input) => {
                            call_action(
                                index_to_action_id.get(&user_input).unwrap(),
                                &mut current_room,
                                &mut score,
                                &mut current_item,
                            );
                        }
                        _ => {}
                    }
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
                    match handle_user_input() {
                        Ok(user_input) => {
                            call_action(
                                index_to_action_id.get(&user_input).unwrap(),
                                &mut current_room,
                                &mut score,
                                &mut current_item,
                            );
                        }
                        _ => {}
                    }
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
