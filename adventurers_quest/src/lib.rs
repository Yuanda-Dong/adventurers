#![warn(missing_docs)]
//! # Adventurer quest crate
//! This is a library crate that can be used with any games with a quest system
//! This library implements four types of quests, all of with implements the quest trait
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]

///QuestStatus indicates whether the quest has been completed
pub enum QuestStatus {
    ///Quest is completed
    Complete,
    ///Quest is still ongoing
    Ongoing,
}
#[derive(Debug, PartialEq, Eq)]
/// An enum created for testing purpose.
/// All quest implements register(event), an event could be a terrain type in the game.
/// The actual event type used in the game is upto the implementor of the game.
pub enum Block {
    /// Grass type terrain
    Grass,
    /// Sand type terrain
    Sand,
    /// Rock type terrain
    Rock,
    /// Cinderblock type terrain
    Cinderblock,
    /// Flowerbush type terrain
    Flowerbush,
    /// Barrier type terrain
    Barrier,
    /// Water type terrain
    Water,
    /// Sign (with string) type terrain
    Sign(String),
    /// Object (with character) type terrain
    Object(char),
}

/// This is what a "quest" should do.
/// Note that all `Quests` implement Debug and Display.
/// Traits' Debug implementation does not matter, but
/// they should implement [`std::fmt::Display`] to show
/// the current progress of the quest.
pub trait Quest<Event>: std::fmt::Display + std::fmt::Debug {
    /// Whenever something happens, you call "register_event" to tell the quest what's happened.
    fn register_event(&mut self, event: &Event) -> QuestStatus;
    /// Reset the quest, so that players can restart.
    fn reset(&mut self);
}

/// QuestExt trait exends Quest trait, and requires impleting get_status function
pub trait QuestExt<Event>: Quest<Event> {
    /// returns the status of the current quest
    fn get_status(&mut self) -> QuestStatus;
}
/// Subquest struct has a target count for a target type, and is
/// useful for implementing quests of type "The player wins the game if they walk over 5 sand blocks".
#[derive(Debug)]
pub struct SubQuest<Event: std::fmt::Debug + std::cmp::PartialEq> {
    target_type: Event,
    target_count: u32,
    count: u32,
    complete: QuestStatus,
    prompt: String,
}

impl<Event: std::fmt::Debug + std::cmp::PartialEq> SubQuest<Event> {
    /// Constructor for Subquest
    /// # Arguments
    /// * `target_type` - A generic type event that implements PartialEq and Debug, so that it can be tested for equality and inequality
    /// * `target_count` - Expected event count to complete the quest
    /// * `prompt` - A string prompt for the quest to be displayed
    pub fn new(target_type: Event, target_count: u32, prompt: String) -> Self {
        SubQuest {
            target_type,
            target_count,
            count: 0,
            complete: QuestStatus::Ongoing,
            prompt,
        }
    }
}

impl<Event: std::fmt::Debug + std::cmp::PartialEq> Display for SubQuest<Event> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.complete {
            QuestStatus::Ongoing => {
                if self.target_count == 1 {
                    write!(f, "[ ] {}...", self.prompt)
                } else {
                    write!(
                        f,
                        "[ ] {}...\n ^ (Complete {} more times)",
                        self.prompt,
                        self.target_count - self.count
                    )
                }
            }
            QuestStatus::Complete => {
                if self.target_count == 1 {
                    write!(f, "[✅] {}...", self.prompt)
                } else {
                    write!(
                        f,
                        "[✅] {}...\n ^ (Complete {} more times)",
                        self.prompt,
                        self.target_count - self.count
                    )
                }
            }
        }
    }
}

impl<Event: std::fmt::Debug + std::cmp::PartialEq> Quest<Event> for SubQuest<Event> {
    fn register_event(&mut self, event: &Event) -> QuestStatus {
        if self.complete == QuestStatus::Ongoing && event == &self.target_type {
            self.count += 1;
        }
        if self.count == self.target_count {
            self.complete = QuestStatus::Complete;
        }
        self.complete
    }
    fn reset(&mut self) {
        self.complete = QuestStatus::Ongoing;
        self.count = 0;
    }
}

impl<Event: std::fmt::Debug + std::cmp::PartialEq> QuestExt<Event> for SubQuest<Event> {
    fn get_status(&mut self) -> QuestStatus {
        self.complete
    }
}
/// SubMemQuest struct
///
/// A SubMemQuest stores 2 types of target_count and is
/// useful for implementing quests of type "walk over 9 blocks of water, 3 times".
#[derive(Debug)]
pub struct SubMemQuest<Event: std::fmt::Debug + std::cmp::PartialEq> {
    target_type: Event,
    target_count: u32,
    count: u32,
    target_mem_count: u32,
    mem_count: u32,
    complete: QuestStatus,
    prompt: String,
}

impl<Event: std::fmt::Debug + std::cmp::PartialEq> SubMemQuest<Event> {
    /// Constructor for SubMemQuest
    /// # Arguments
    /// * `target_type` - A generic type event that implements PartialEq and Debug, so that it can be tested for equality and inequality
    /// * `target_mem_count` - Expected number of of times of receiving the same incoming event without interuption, to make progress on `target_count`
    /// * `target_count` - Expected number of times of reaching `target_mem_count`, to complete the quest
    /// * `prompt` - A string prompt for the quest to be displayed
    pub fn new(
        target_type: Event,
        target_mem_count: u32,
        target_count: u32,
        prompt: String,
    ) -> Self {
        SubMemQuest {
            target_type,
            target_count,
            count: 0,
            target_mem_count,
            mem_count: 0,
            complete: QuestStatus::Ongoing,
            prompt,
        }
    }
}

impl<Event: std::fmt::Debug + std::cmp::PartialEq> Display for SubMemQuest<Event> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.complete {
            QuestStatus::Ongoing => {
                if self.target_count == 1 {
                    write!(f, "[ ] {}...", self.prompt)
                } else {
                    write!(
                        f,
                        "[ ] {}...\n ^ (Complete {} more times)",
                        self.prompt,
                        self.target_count - self.count
                    )
                }
            }
            QuestStatus::Complete => {
                if self.target_count == 1 {
                    write!(f, "[✅] {}...", self.prompt)
                } else {
                    write!(
                        f,
                        "[✅] {}...\n ^ (Complete {} more times)",
                        self.prompt,
                        self.target_count - self.count
                    )
                }
            }
        }
    }
}

impl<Event: std::fmt::Debug + std::cmp::PartialEq> Quest<Event> for SubMemQuest<Event> {
    fn register_event(&mut self, event: &Event) -> QuestStatus {
        if self.complete == QuestStatus::Ongoing {
            if event == &self.target_type {
                self.mem_count += 1;
            } else {
                self.mem_count = 0;
            }
            if self.mem_count == self.target_mem_count {
                self.count += 1;
                self.mem_count = 0;
            }
            if self.count == self.target_count {
                self.complete = QuestStatus::Complete;
            }
        }
        self.complete
    }
    fn reset(&mut self) {
        self.complete = QuestStatus::Ongoing;
        self.count = 0;
        self.mem_count = 0;
    }
}

impl<Event: std::fmt::Debug + std::cmp::PartialEq> QuestExt<Event> for SubMemQuest<Event> {
    fn get_status(&mut self) -> QuestStatus {
        self.complete
    }
}

/// OrderedQuest struct
///
/// An orderedQuest struct stores a vector of [`SubQuest`] to be completed in sequence
///
/// Useful for implementing quests of type "First, collect five objects called 'x',
/// After finishing that, collect three objects called 'y'".
#[derive(Debug)]
pub struct OrderedQuest<Event: std::fmt::Debug + std::cmp::PartialEq> {
    sub_quests: Vec<SubQuest<Event>>,
    complete: QuestStatus,
    prompt: String,
}

impl<Event: std::fmt::Debug + std::cmp::PartialEq> OrderedQuest<Event> {
    /// Constructor for Subquest
    /// # Arguments
    /// * `sub_quests` - A vector of [`SubQuest`] to be completed in sequence
    pub fn new(sub_quests: Vec<SubQuest<Event>>) -> Self {
        OrderedQuest {
            sub_quests,
            complete: QuestStatus::Ongoing,
            prompt: String::from("You must, in order, complete each of these quests:"),
        }
    }
}

impl<Event: std::fmt::Debug + std::cmp::PartialEq> Display for OrderedQuest<Event> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut children = String::from("");
        self.sub_quests.iter().for_each(|sub| {
            let child_message = sub.to_string();
            for line in child_message.split('\n') {
                children.push_str("  ");
                children.push_str(line);
                children.push('\n');
            }
        });
        match self.complete {
            QuestStatus::Ongoing => write!(f, "[ ] {}...\n{}", self.prompt, children),
            QuestStatus::Complete => write!(f, "[✅] {}...\n{}", self.prompt, children),
        }
    }
}

impl<Event: std::fmt::Debug + std::cmp::PartialEq> Quest<Event> for OrderedQuest<Event> {
    fn register_event(&mut self, event: &Event) -> QuestStatus {
        if let Some(quest) = self
            .sub_quests
            .iter_mut()
            .find(|quest| quest.complete == QuestStatus::Ongoing)
        {
            quest.register_event(event);
        }
        match self
            .sub_quests
            .iter_mut()
            .find(|quest| quest.complete == QuestStatus::Ongoing)
        {
            Some(_) => {}
            None => {
                self.complete = QuestStatus::Complete;
            }
        }
        self.complete
    }
    fn reset(&mut self) {
        self.complete = QuestStatus::Ongoing;
        for sub in &mut self.sub_quests {
            sub.reset();
        }
    }
}

impl<Event: std::fmt::Debug + std::cmp::PartialEq> QuestExt<Event> for OrderedQuest<Event> {
    fn get_status(&mut self) -> QuestStatus {
        self.complete
    }
}

/// QuestSystem struct
///
/// A quest system struct stores arbitary number of any OrderedQuest, SubQuest, SubMemQuest.
///
/// A quest system is completed if the player has completed a defined amount of stored quests.
///
/// Useful for implementing quests of type "The player wins the game if they do 2 of the following:"
///
/// "walk over 5 blocks of sand", then "collect an 'x' object"
///
/// "collect a 'y' object", then "walk on grass"
///
/// "walk over 9 blocks of water, 3 times"
#[derive(Debug)]
pub struct QuestSystem<Event: std::fmt::Debug + std::cmp::PartialEq> {
    v_ordered: Vec<OrderedQuest<Event>>,
    v_sub_quest: Vec<SubQuest<Event>>,
    v_mem_quest: Vec<SubMemQuest<Event>>,
    count: u32,
    target_count: u32,
    complete: QuestStatus,
    prompt: String,
}

impl<Event: std::fmt::Debug + std::cmp::PartialEq> QuestSystem<Event> {
    /// Constructor for Subquest
    /// # Arguments
    /// * `v_ordered` - A vector of [`OrderedQuest`]
    /// * `v_sub_quest` - A vector of [`SubQuest`]
    /// * `v_mem_quest` - A vector of [`SubMemQuest`]
    /// * `target_count` - The number of stored quests that need to be completed in order to complete the quest system
    pub fn new(
        v_ordered: Vec<OrderedQuest<Event>>,
        v_sub_quest: Vec<SubQuest<Event>>,
        v_mem_quest: Vec<SubMemQuest<Event>>,
        target_count: u32,
    ) -> QuestSystem<Event> {
        QuestSystem {
            v_ordered,
            v_sub_quest,
            v_mem_quest,
            count: 0,
            target_count,
            complete: QuestStatus::Ongoing,
            prompt: format!(
                "You must complete at least {} of these quests",
                target_count
            ),
        }
    }
}

impl<Event: std::fmt::Debug + std::cmp::PartialEq> Display for QuestSystem<Event> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut children = String::from("");
        self.v_ordered.iter().for_each(|sub| {
            let child_message = sub.to_string();
            for line in child_message.split('\n') {
                children.push_str("  ");
                children.push_str(line);
                children.push('\n')
            }
        });
        self.v_sub_quest.iter().for_each(|sub| {
            let child_message = sub.to_string();
            for line in child_message.split('\n') {
                children.push_str("  ");
                children.push_str(line);
                children.push('\n');
            }
        });
        self.v_mem_quest.iter().for_each(|sub| {
            let child_message = sub.to_string();
            for line in child_message.split('\n') {
                children.push_str("  ");
                children.push_str(line);
                children.push('\n');
            }
        });
        match self.complete {
            QuestStatus::Ongoing => write!(f, "[ ] {}...\n{}", self.prompt, children),
            QuestStatus::Complete => write!(f, "[✅] {}...\n{}", self.prompt, children),
        }
    }
}

impl<Event: std::fmt::Debug + std::cmp::PartialEq> Quest<Event> for QuestSystem<Event> {
    fn register_event(&mut self, event: &Event) -> QuestStatus {
        let mut num_completed: u32 = 0;
        for quest in &mut self.v_ordered {
            quest.register_event(event);
            if quest.complete == QuestStatus::Complete {
                num_completed += 1;
            }
        }
        for quest in &mut self.v_sub_quest {
            quest.register_event(event);
            if quest.complete == QuestStatus::Complete {
                num_completed += 1;
            }
        }
        for quest in &mut self.v_mem_quest {
            quest.register_event(event);
            if quest.complete == QuestStatus::Complete {
                num_completed += 1;
            }
        }
        self.count = num_completed;

        if self.count >= self.target_count {
            self.complete = QuestStatus::Complete;
        }
        self.complete
    }

    fn reset(&mut self) {
        self.complete = QuestStatus::Ongoing;
        self.count = 0;
        for quest in &mut self.v_ordered {
            quest.reset();
        }
        for quest in &mut self.v_sub_quest {
            quest.reset();
        }
    }
}

impl<Event: std::fmt::Debug + std::cmp::PartialEq> QuestExt<Event> for QuestSystem<Event> {
    fn get_status(&mut self) -> QuestStatus {
        self.complete
    }
}

#[cfg(test)]
mod tests {
    use crate::{Block, OrderedQuest, Quest, QuestSystem, SubMemQuest, SubQuest};

    #[test]
    fn sub_quest1() {
        let test_quest = SubQuest::new(Block::Grass, 5, "Walk on Grass 5 times".to_string());
        assert_eq!(
            String::from("[ ] Walk on Grass 5 times...\n ^ (Complete 5 more times)"),
            test_quest.to_string()
        );
    }

    #[test]
    fn sub_quest2() {
        let mut test_quest = SubQuest::new(Block::Grass, 5, "Walk on Grass 5 times".to_string());
        test_quest.register_event(&Block::Grass);
        assert_eq!(
            String::from("[ ] Walk on Grass 5 times...\n ^ (Complete 4 more times)"),
            test_quest.to_string()
        );
    }

    #[test]
    fn sub_quest3() {
        let mut test_quest = SubQuest::new(Block::Grass, 5, "Walk on Grass 5 times".to_string());
        test_quest.register_event(&Block::Grass);
        test_quest.register_event(&Block::Grass);
        test_quest.register_event(&Block::Grass);
        test_quest.register_event(&Block::Grass);
        test_quest.register_event(&Block::Grass);
        assert_eq!(
            String::from("[✅] Walk on Grass 5 times...\n ^ (Complete 0 more times)"),
            test_quest.to_string()
        );
    }

    #[test]
    fn sub_quest4() {
        let mut test_quest = SubQuest::new(Block::Grass, 5, "Walk on Grass 5 times".to_string());
        test_quest.register_event(&Block::Water);
        assert_eq!(
            String::from("[ ] Walk on Grass 5 times...\n ^ (Complete 5 more times)"),
            test_quest.to_string()
        );
    }

    #[test]
    fn ordered1() {
        let test_quest1 = SubQuest::new(Block::Grass, 5, "Walk on Grass 5 times".to_string());
        let test_quest2 = SubQuest::new(Block::Sand, 1, "Walk on Sand".to_string());
        let mut ordered = OrderedQuest::new(vec![test_quest1, test_quest2]);
        ordered.register_event(&Block::Sand);
        assert_eq!(
            "[ ] You must, in order, complete each of these quests:...\n  [ ] Walk on Grass 5 times...\n   ^ (Complete 5 more times)\n  [ ] Walk on Sand...\n",
            ordered.to_string()
        );
    }

    #[test]
    fn ordered2() {
        let test_quest1 = SubQuest::new(Block::Grass, 5, "Walk on Grass 5 times".to_string());
        let test_quest2 = SubQuest::new(Block::Sand, 1, "Walk on Sand".to_string());
        let mut ordered = OrderedQuest::new(vec![test_quest1, test_quest2]);
        for _ in 0..5 {
            ordered.register_event(&Block::Grass);
        }
        assert_eq!(
            "[ ] You must, in order, complete each of these quests:...\n  [✅] Walk on Grass 5 times...\n   ^ (Complete 0 more times)\n  [ ] Walk on Sand...\n",
            ordered.to_string()
        );
    }

    #[test]
    fn ordered3() {
        let test_quest1 = SubQuest::new(Block::Grass, 5, "Walk on Grass 5 times".to_string());
        let test_quest2 = SubQuest::new(Block::Sand, 1, "Walk on Sand".to_string());
        let mut ordered = OrderedQuest::new(vec![test_quest1, test_quest2]);
        for _ in 0..5 {
            ordered.register_event(&Block::Grass);
        }
        ordered.register_event(&Block::Sand);
        assert_eq!(
            "[✅] You must, in order, complete each of these quests:...\n  [✅] Walk on Grass 5 times...\n   ^ (Complete 0 more times)\n  [✅] Walk on Sand...\n",
            ordered.to_string()
        );
    }

    #[test]
    fn sub_mem_quest1() {
        let mut test_quest = SubMemQuest::new(
            Block::Water,
            5,
            3,
            "Walk over exactly 5 blocks of water".to_string(),
        );

        for _ in 0..4 {
            test_quest.register_event(&Block::Water);
        }
        test_quest.register_event(&Block::Sand);
        test_quest.register_event(&Block::Water);
        assert_eq!(
            "[ ] Walk over exactly 5 blocks of water...\n ^ (Complete 3 more times)",
            test_quest.to_string()
        );
    }

    #[test]
    fn sub_mem_quest2() {
        let mut test_quest = SubMemQuest::new(
            Block::Water,
            5,
            3,
            "Walk over exactly 5 blocks of water".to_string(),
        );
        for _ in 0..5 {
            test_quest.register_event(&Block::Water);
        }
        assert_eq!(
            "[ ] Walk over exactly 5 blocks of water...\n ^ (Complete 2 more times)",
            test_quest.to_string()
        );
    }

    #[test]
    fn sub_mem_quest3() {
        let mut test_quest = SubMemQuest::new(
            Block::Water,
            5,
            3,
            "Walk over exactly 5 blocks of water".to_string(),
        );
        for _ in 0..15 {
            test_quest.register_event(&Block::Water);
        }
        assert_eq!(
            "[✅] Walk over exactly 5 blocks of water...\n ^ (Complete 0 more times)",
            test_quest.to_string()
        );
    }

    #[test]
    fn quest_sys() {
        let test_quest1 = SubQuest::new(Block::Grass, 5, "Walk on Grass 5 times".to_string());
        let test_quest2 = SubQuest::new(Block::Sand, 1, "Walk on Sand".to_string());
        let ordered1 = OrderedQuest::new(vec![test_quest1, test_quest2]);
        let test_quest3 = SubQuest::new(Block::Grass, 6, "Walk on Grass 6 times".to_string());
        let test_quest4 = SubQuest::new(Block::Sand, 1, "Walk on Sand".to_string());
        let ordered2 = OrderedQuest::new(vec![test_quest3, test_quest4]);
        let mem_quest = SubMemQuest::new(
            Block::Water,
            5,
            3,
            "Walk over exactly 5 blocks of water".to_string(),
        );
        let mut quest_sys = QuestSystem::new(vec![ordered1, ordered2], vec![], vec![mem_quest], 2);
        quest_sys.register_event(&Block::Sand);
        quest_sys.register_event(&Block::Grass);
        quest_sys.register_event(&Block::Grass);
        quest_sys.register_event(&Block::Grass);
        quest_sys.register_event(&Block::Grass);
        quest_sys.register_event(&Block::Grass);
        quest_sys.register_event(&Block::Sand);
        quest_sys.register_event(&Block::Barrier);
        quest_sys.register_event(&Block::Barrier);
        quest_sys.register_event(&Block::Water);
        quest_sys.register_event(&Block::Water);
        quest_sys.register_event(&Block::Water);
        quest_sys.register_event(&Block::Water);
        quest_sys.register_event(&Block::Water);
        quest_sys.register_event(&Block::Sand);
        quest_sys.register_event(&Block::Water);
        quest_sys.register_event(&Block::Water);
        quest_sys.register_event(&Block::Water);
        quest_sys.register_event(&Block::Water);
        quest_sys.register_event(&Block::Water);
        quest_sys.register_event(&Block::Water);
        quest_sys.register_event(&Block::Water);
        quest_sys.register_event(&Block::Water);
        quest_sys.register_event(&Block::Water);
        quest_sys.register_event(&Block::Water);
        assert_eq!("[✅] You must complete at least 2 of these quests...\n  [✅] You must, in order, complete each of these quests:...\n    [✅] Walk on Grass 5 times...\n     ^ (Complete 0 more times)\n    [✅] Walk on Sand...\n  \n  [ ] You must, in order, complete each of these quests:...\n    [ ] Walk on Grass 6 times...\n     ^ (Complete 1 more times)\n    [ ] Walk on Sand...\n  \n  [✅] Walk over exactly 5 blocks of water...\n   ^ (Complete 0 more times)\n", quest_sys.to_string());
    }
}
