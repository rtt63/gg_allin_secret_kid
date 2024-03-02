use crate::clear_console;
use crate::ActionId;
use crate::Item;
use crate::Room;

pub fn call_action(
    action_id: &ActionId,
    current_room: &mut Room,
    score: &mut i8,
    current_item: &mut Item,
) {
    clear_console().expect("Console wasn't cleared");

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
