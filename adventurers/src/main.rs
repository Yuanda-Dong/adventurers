use adventurers_quest::{OrderedQuest, QuestExt, QuestSystem, SubMemQuest, SubQuest};
use block::Block;
use direction::Direction;
use player::{Movement, Player};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use termgame::{
    run_game, Controller, Game, GameEvent, GameSettings, KeyCode, Message, SimpleEvent,
    StyledCharacter,
};
enum MyResult {
    GameParam(HashMap<(i32, i32), Block>, Box<dyn QuestExt<Block>>),
    MissingArguments,
    MapNotFound,
    MapIncorrectFormat,
    QuestNotImplemented,
}

pub mod block;
pub mod direction;
pub mod player;
pub struct MyGame {
    player: Player,
}

impl Controller for MyGame {
    fn on_start(&mut self, game: &mut Game) {
        for (key, value) in &self.player.map {
            game.set_screen_char(key.0, key.1, Some(StyledCharacter::from(value)));
        }
        self.player.show_player(game);
    }

    fn on_event(&mut self, game: &mut Game, event: GameEvent) {
        game.set_message(None);
        if self.player.died || self.player.won {
            game.end_game();
        }
        match event.into() {
            SimpleEvent::Just(KeyCode::Up) => {
                self.player.move_player(game, Direction::Up);
            }
            SimpleEvent::Just(KeyCode::Down) => {
                self.player.move_player(game, Direction::Down);
            }
            SimpleEvent::Just(KeyCode::Left) => {
                self.player.move_player(game, Direction::Left);
            }
            SimpleEvent::Just(KeyCode::Right) => {
                self.player.move_player(game, Direction::Right);
            }
            SimpleEvent::Just(KeyCode::Char(c)) => {
                if c == 'q' {
                    game.set_message(Some(Message::new(self.player.quest.to_string())));
                }
                if c == 'r' {
                    self.player.quest.reset();
                }
            }
            _ => {}
        }
    }
    fn on_tick(&mut self, _game: &mut Game) {}
}

fn prepare_launch(args: Vec<String>) -> MyResult {
    if args.len() < 3 {
        return MyResult::MissingArguments;
    }
    let my_path = args.get(1).unwrap();
    let my_quest = args.get(2).unwrap();
    let file = File::open(my_path);
    if file.is_err() {
        return MyResult::MapNotFound;
    }
    let reader = BufReader::new(file.unwrap());
    let map: Result<HashMap<(i32, i32), Block>, _> = ron::de::from_reader(reader);
    if map.is_err() {
        return MyResult::MapIncorrectFormat;
    }
    if my_quest == "q1" {
        let quest1 = SubQuest::new(Block::Sand, 5, "Walk on Sand".to_string());
        MyResult::GameParam(map.unwrap(), Box::new(quest1))
    } else if my_quest == "q2" {
        let quest2_1 = SubQuest::new(Block::Object('x'), 5, "Collect a 'x'".to_string());
        let quest2_2 = SubQuest::new(Block::Object('y'), 3, "Collect a 'y'".to_string());
        let quest2 = OrderedQuest::new(vec![quest2_1, quest2_2]);
        MyResult::GameParam(map.unwrap(), Box::new(quest2))
    } else if my_quest == "q3" {
        let quest3_1_1 = SubQuest::new(Block::Sand, 5, "Walk on Sand".to_string());
        let quest3_1_2 = SubQuest::new(Block::Object('x'), 1, "Collect a 'x'".to_string());
        let quest3_1 = OrderedQuest::new(vec![quest3_1_1, quest3_1_2]);
        let quest3_2_1 = SubQuest::new(Block::Object('y'), 1, "Collect a 'y'".to_string());
        let quest3_2_2 = SubQuest::new(Block::Grass, 1, "Walk on Grass".to_string());
        let quest3_2 = OrderedQuest::new(vec![quest3_2_1, quest3_2_2]);
        let quest3_3 = SubMemQuest::new(
            Block::Water,
            9,
            3,
            "Walk over exactly 9 blocks of water".to_string(),
        );
        let quest3 = QuestSystem::new(vec![quest3_1, quest3_2], vec![], vec![quest3_3], 2);
        MyResult::GameParam(map.unwrap(), Box::new(quest3))
    } else {
        MyResult::QuestNotImplemented
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    match prepare_launch(args) {
        MyResult::GameParam(map, quest) => {
            let player = Player {
                x: 2,
                y: 2,
                map,
                breath: 10,
                died: false,
                quest,
                won: false,
            };
            let mut controller = MyGame { player };

            run_game(
                &mut controller,
                GameSettings::new()
                    .tick_duration(Duration::from_millis(50))
                    .quit_event(Some(SimpleEvent::WithControl(KeyCode::Char('c')).into())),
            )?;
            println!("Game Ended!");
        }
        MyResult::MissingArguments => println!("Missing arguments"),
        MyResult::MapNotFound => println!("Map not found"),
        MyResult::MapIncorrectFormat => println!("Map is in incorrect format"),
        MyResult::QuestNotImplemented => {
            println!("Quest not supported, valid quests are q1,q2 and q3")
        }
    }
    Ok(())
}
