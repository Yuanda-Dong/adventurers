use serde::Deserialize;
use termgame::{GameColor, GameStyle, StyledCharacter};
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum Block {
    Grass,
    Sand,
    Rock,
    Cinderblock,
    Flowerbush,
    Barrier,
    Water,
    Sign(String),
    Object(char),
    Empty,
}

impl From<&Block> for StyledCharacter {
    fn from(block: &Block) -> Self {
        let styled = StyledCharacter::new(' ');
        match block {
            Block::Grass => styled.style(GameStyle::new().background_color(Some(GameColor::Green))),
            Block::Sand => styled.style(GameStyle::new().background_color(Some(GameColor::Yellow))),
            Block::Rock => styled.style(GameStyle::new().background_color(Some(GameColor::Gray))),
            Block::Cinderblock => {
                styled.style(GameStyle::new().background_color(Some(GameColor::LightRed)))
            }
            Block::Flowerbush => {
                styled.style(GameStyle::new().background_color(Some(GameColor::Magenta)))
            }

            Block::Barrier => {
                styled.style(GameStyle::new().background_color(Some(GameColor::White)))
            }
            Block::Water => styled.style(GameStyle::new().background_color(Some(GameColor::Blue))),
            Block::Sign(_) => StyledCharacter::new('💬'),
            Block::Object(char) => StyledCharacter::new(*char),
            Block::Empty => styled.style(GameStyle::new().background_color(Some(GameColor::Black))),
        }
    }
}
