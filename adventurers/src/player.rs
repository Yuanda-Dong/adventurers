use crate::direction::Direction;
use crate::Block;
use adventurers_quest::{QuestExt, QuestStatus};
use std::collections::HashMap;
use termgame::{Game, Message, StyledCharacter, ViewportLocation};

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub map: HashMap<(i32, i32), Block>,
    pub breath: i32,
    pub died: bool,
    pub quest: Box<dyn QuestExt<Block>>,
    pub won: bool,
}

pub trait Movement {
    fn remove_player(&mut self, game: &mut Game);
    fn show_player(&self, game: &mut Game);
    fn move_player(&mut self, game: &mut Game, direction: Direction);
    fn check_move(&mut self, direction: &Direction) -> Option<&Block>;
    fn perform_move(&mut self, game: &mut Game, direction: Direction);
}

impl Movement for Player {
    fn remove_player(&mut self, game: &mut Game) {
        if let Some(b) = self.map.get(&(self.x, self.y)) {
            match b {
                Block::Object(_) => {
                    game.set_screen_char(self.x, self.y, Some(StyledCharacter::from(' ')));
                    self.map.remove(&(self.x, self.y));
                }
                _ => game.set_screen_char(self.x, self.y, Some(StyledCharacter::from(b))),
            }
        } else {
            game.set_screen_char(self.x, self.y, None);
        }
    }
    fn show_player(&self, game: &mut Game) {
        if let Some(prev) = game.get_screen_char(self.x, self.y) {
            game.set_screen_char(self.x, self.y, Some(prev.character('♟')));
        } else {
            game.set_screen_char(self.x, self.y, Some(StyledCharacter::from('♟')));
        }

        let viewport = game.get_viewport();
        if self.x > viewport.x + 76 {
            game.set_viewport(ViewportLocation {
                x: self.x - 76,
                y: viewport.y,
            });
        }
        if self.x < viewport.x {
            game.set_viewport(ViewportLocation {
                x: self.x,
                y: viewport.y,
            });
        }
        if self.y > viewport.y + 20 {
            game.set_viewport(ViewportLocation {
                x: viewport.x,
                y: self.y - 20,
            });
        }
        if self.y < viewport.y {
            game.set_viewport(ViewportLocation {
                x: viewport.x,
                y: self.y,
            });
        }
    }
    fn move_player(&mut self, game: &mut Game, direction: Direction) {
        let nb = self.check_move(&direction);
        match nb {
            Some(b) => match b {
                Block::Barrier => {}
                Block::Sign(message) => {
                    let m = Message::new(String::from(message));
                    game.set_message(Some(m));
                    self.perform_move(game, direction)
                }
                Block::Water => {
                    self.breath -= 1;
                    if self.breath == 0 {
                        self.died = true;
                        game.set_message(Some(Message::new(String::from("You Drowned :("))));
                    } else {
                        self.perform_move(game, direction)
                    }
                }
                _ => {
                    self.breath = 10;
                    self.perform_move(game, direction)
                }
            },
            None => {
                self.breath = 10;
                self.perform_move(game, direction)
            }
        }
    }
    fn check_move(&mut self, direction: &Direction) -> Option<&Block> {
        match direction {
            Direction::Up => self.map.get(&(self.x, self.y - 1)),
            Direction::Down => self.map.get(&(self.x, self.y + 1)),
            Direction::Left => self.map.get(&(self.x - 1, self.y)),
            Direction::Right => self.map.get(&(self.x + 1, self.y)),
        }
    }

    fn perform_move(&mut self, game: &mut Game, direction: Direction) {
        let block = match direction {
            Direction::Up => self.map.get(&(self.x, self.y - 1)),
            Direction::Down => self.map.get(&(self.x, self.y + 1)),
            Direction::Left => self.map.get(&(self.x - 1, self.y)),
            Direction::Right => self.map.get(&(self.x + 1, self.y)),
        };
        match block {
            Some(_) => {
                self.quest.register_event(block.unwrap());
            }
            None => {
                self.quest.register_event(&Block::Empty);
            }
        }
        self.remove_player(game);
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
        self.show_player(game);
        if self.quest.get_status() == QuestStatus::Complete {
            self.won = true;
            game.set_message(Some(Message::new(String::from("You Won :)"))));
        }
    }
}
