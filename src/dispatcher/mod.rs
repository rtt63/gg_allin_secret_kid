use crate::clear_console;
use crate::ItemId;
use crate::RoomId;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ActionId {
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

    PickChair,
    KickChairFromSoundGuy,
    PutLegsOnTheChair,

    PickBottle,
    MakeHoleInBottleAndGetHigh,
    ThrowBottleToTheAssistant,
    DrinkFromBottle,

    PickDrums,
    HitCymbalsRandomly,
    ScreamAtTheSnare,

    PickGuitarAmp,
    TurnVolumeToEleven,
    AllButtonsUp,
    PressStandBy,

    PickMic,
    SwearAtTheCleaner,
    CaughtForThreeMinutesStraight,
    ThrowMicAtTheWall,

    SingToTheCureAndCry,
    PressRandomNumbersOnAnIntercom,
    StaringAtTheDoorForLoongTime,

    DropItem,

    ExitGame,
}

struct GlobalState {
    mics_broken: u8,
    the_cure_singing_attemps: u8,
    intercom_open_counter: u8,
}

impl GlobalState {
    pub fn break_mic(&self) {
        let upd_value = &self.mics_broken + 1;
        self.mics_broken = upd_value;
    }

    pub fn sing_the_cure(&self) {
        let upd_value = &self.the_cure_singing_attemps + 1;
        self.the_cure_singing_attemps = upd_value;
    }

    pub fn open_intercom(&self) {
        let upd_value = &self.intercom_open_counter + 1;
        self.intercom_open_counter = upd_value;
    }
}

const STATE: GlobalState = GlobalState {
    mics_broken: 0,
    the_cure_singing_attemps: 0,
    intercom_open_counter: 0,
};

pub fn call_action(
    action_id: &ActionId,
    current_room: &mut RoomId,
    score: &mut i8,
    current_item: &mut ItemId,
) {
    clear_console();

    match action_id {
        ActionId::GoToControlRoom => {
            *current_room = RoomId::ControlRoom;
            println!("You're in the control room");
        }
        ActionId::GoToToneRoom => {
            *current_room = RoomId::ToneRoom;
            println!("You're in the tone room");
        }
        ActionId::GoToHall => {
            *current_room = RoomId::Hall;
            println!("You're in the hall");
        }
        ActionId::GoToVocalBooth => {
            *current_room = RoomId::VoiceBooth;
            println!("You're in the voice booth. It's so scary! No way!!! You're going back");
            *current_room = RoomId::ToneRoom;
            println!("You're in the tone room");
        }
        ActionId::SpillBeerOnTheConsole => {
            println!("Well... don't do that again");
            *score -= 2;
        }
        ActionId::PickConsole => {
            *current_item = ItemId::Console;
        }
        ActionId::PushRandomButton => {
            println!("Wow! That's mixing!");
            *score += 1;
        }
        ActionId::MakeAllFadersUp => {
            println!("You're the greatest punk rocker of the universe!");
            *score += 2;
        }
        ActionId::PickMic => {
            *current_item = ItemId::Mic;
        }
        ActionId::PickDrums => {
            *current_item = ItemId::Drums;
        }
        ActionId::PickBottle => {
            *current_item = ItemId::BottleOfWatter;
        }
        ActionId::PickGuitarAmp => {
            *current_item = ItemId::GuitarAmp;
        }
        ActionId::PickChair => {
            *current_item = ItemId::ArmChair;
        }
        ActionId::StaringAtTheDeskWithTheDumbFace => {
            println!("Well.. it least you're not hurting anyone");
            *score += 1;
        }
        ActionId::PickImac => {
            *current_item = ItemId::Imac;
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
        ActionId::PressStandBy => {
            println!("Ooops, you powered it off. Need help!!");
            *score -= 1;
        }
        ActionId::TurnVolumeToEleven => {
            println!("Yeah! It is Marshall!");
            *score += 2;
        }
        ActionId::AllButtonsUp => {
            println!("You broke it!");
            *score -= 1;
        }
        ActionId::SwearAtTheCleaner => {
            println!("WOW! An album full of hits is done!");
            *score = 127;
        }
        ActionId::CaughtForThreeMinutesStraight => {
            println!("Great lyrics!!");
            *score += 1;
        }
        ActionId::HitCymbalsRandomly => {
            println!("No one can talk, no one can hear a thing. You're happy???");
            *score -= 2;
        }
        ActionId::ScreamAtTheSnare => {
            println!("Wow.. sound like.. some... Pink Floyd?...");
            *score += 1;
        }
        ActionId::SingToTheCureAndCry => {
            STATE.sing_the_cure();
            let done_attemps = STATE.the_cure_singing_attemps;
            match done_attemps {
                1 => {
                    println!("Your band members already want to get rid of you");
                }
                2 => {
                    println!("Seriosly, stop it");
                }
                3 => {
                    println!("No more The Cure please");
                }
                _ => {
                    println!("They kicked you out from the studio. What did you expect?");
                    *score = -127;
                    std::process::exit(0);
                }
            }
            *score -= 2;
        }
        ActionId::PressRandomNumbersOnAnIntercom => {
            let intercome_opened = STATE.intercom_open_counter;
            match intercome_opened {
                0 => {
                    println!("Ha! There was a pizza delivery guy waiting! Nice one!");
                    *score += 2;
                    STATE.open_intercom();
                }
                _ => {
                    println!("Nobody there");
                }
            }
        }
        ActionId::StaringAtTheDoorForLoongTime => {
            println!("At least you're not hurting anyone");
            *score += 1;
        }
        ActionId::DrinkFromBottle => {
            println!("Hydrating!");
            *score += 1;
        }
        ActionId::MakeHoleInBottleAndGetHigh => {
            println!("TRUEEE PUNK ROCKER");
            *score += 2;
        }
        ActionId::ThrowBottleToTheAssistant => {
            println!("Sound guy doesn't like him anyway");
            *score += 1;
        }
        ActionId::ThrowMicAtTheWall => {
            STATE.break_mic();
            let broken_mics = STATE.mics_broken;
            match broken_mics {
                1 => {
                    println!("You broke it");
                }
                2 => {
                    println!("They ain't free!!!'");
                }
                3 => {
                    println!("There is only one now");
                }
                _ => {
                    println!("You broke the entire studio.");
                }
            }
            *score -= 2;
        }
        ActionId::KickChairFromSoundGuy => {
            println!("Make him moooove!");
            *score += 1;
        }
        ActionId::PutLegsOnTheChair => {
            println!("...");
        }
        ActionId::DropItem => {
            *current_item = ItemId::NoItem;
            println!("Item dropped");
        }
        ActionId::ExitGame => {
            println!("Bye!");
            std::process::exit(0);
        }
    }
}
