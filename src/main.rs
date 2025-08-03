mod abilities;

use std::collections::VecDeque;
use std::io;

use crate::abilities::*;

enum GameState {
    TitleScreen,
    ClassSelection,
    InGame,
    Combat,
    GameOver,
    Quit,
}

struct Player {
    class: Class,
    level: u8,
    abilities: Vec<PlayerAbility>,
    learnset: VecDeque<PlayerAbility>,
}

impl Player {
    fn new(class: Class) -> Self {
        let learnset = class.get_learnset();
        Player {
            class,
            level: 1,
            abilities: vec![],
            learnset,
        }
    }
}

enum Stat {
    Hp,
    Atk,
    Def,
    Int,
    Res,
    Dex,
    Spd,
}

// impl Stat {
//     fn apply_buff(&mut self, stat: Stat, amount: u8) {
//         match stat {
//             Stat::Hp => self.hp += amount as u32,
//             Stat::Atk => self.atk += amount,
//             Stat::Def => self.def += amount,
//             Stat::Int => self.int += amount,
//             Stat::Res => self.res += amount,
//             Stat::Dex => self.dex += amount,
//             Stat::Spd => self.spd += amount,
//         }
//     }
// }

#[derive(Debug)]
enum Class {
    Archer,
    Knight,
    Mage,
    Rogue,
}

impl Class {
    fn get_learnset(&self) -> VecDeque<PlayerAbility> {
        match self {
            Class::Knight => VecDeque::from(vec![
                abilities::slash(),
                abilities::shield_bash(),
                abilities::block(),
            ]),
            Class::Mage => VecDeque::from(vec![
                abilities::fireball(),
                abilities::ice_spike(),
                abilities::mana_shield(),
            ]),
            Class::Archer => VecDeque::from(vec![
                abilities::arrow_shot(),
                abilities::caltrops(),
                abilities::dodge_stance(),
            ]),
            Class::Rogue => VecDeque::from(vec![
                abilities::stab(),
                abilities::poison_barb(),
                abilities::smokescreen(),
            ]),
        }
    }
}

fn main() {
    let mut state = GameState::TitleScreen;
    let mut player: Option<Player> = None;

    loop {
        match state {
            GameState::TitleScreen => title_screen(&mut state),
            GameState::ClassSelection => class_selection(&mut state, &mut player),
            GameState::InGame => (),
            GameState::Combat => (),
            GameState::GameOver => (),
            GameState::Quit => break,
        }
    }
}

fn title_screen(state: &mut GameState) {
    println!(
        r#"
   _____  _____ ____  _   _ _   _  ____ _____ ___  _   _ 
  / _ \ \/ /_ _|  _ \| | | | \ | |/ ___| ____/ _ \| \ | |
 | | | \  / | || | | | | | |  \| | |  _|  _|| | | |  \| |
 | |_| /  \ | || |_| | |_| | |\  | |_| | |__| |_| | |\  |
  \___/_/\_\___|____/ \___/|_| \_|\____|_____\___/|_| \_|

                   WELCOME TO OXIDUNGEON                 
                  
                  Press 'Enter' to start!"#
    );

    let mut _buffer = String::new();
    let _ = io::stdin().read_line(&mut _buffer);

    *state = GameState::ClassSelection;
}

fn class_selection(state: &mut GameState, player: &mut Option<Player>) {
    println!("Choose a class: Mage, Knight, Rogue, Archer");
    println!("Type your class!");

    let mut _class = String::new();
    let _ = io::stdin()
        .read_line(&mut _class)
        .expect("Failed to read line");

    let class_choice = match _class.trim().to_lowercase().as_str() {
        "mage" => Some(Class::Mage),
        "knight" => Some(Class::Knight),
        "rogue" => Some(Class::Rogue),
        "archer" => Some(Class::Archer),
        _ => None,
    };

    match class_choice {
        Some(class) => {
            println!("You chose {:?}", class);
            *player = Some(Player::new(class));
            *state = GameState::InGame;
        }
        None => {
            println!("Invalid class!");
            *state = GameState::ClassSelection;
            class_selection(state, player)
        }
    }
}
