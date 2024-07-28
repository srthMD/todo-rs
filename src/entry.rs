use clap::ValueEnum;
use console::{Style, StyledObject};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
    pub name: String,
    pub status: EntryStatus
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Serialize, Deserialize, Debug)]
pub enum EntryStatus {
    Incomplete = 0,
    InProgress = 1,
    Scrapped = 2,
    Completed = 3,
}

impl Entry {
    pub fn new(name: String) -> Entry {
        Entry {
            name,
            status: EntryStatus::Incomplete
        }
    }

    pub fn to_styled(self) -> StyledObject<String> {
        // where the fuck else am i supposed to put these
        let incomplete_style: Style = Style::new();
        let complete_style: Style = Style::new().green().bold().underlined();
        let inprogress_style: Style = Style::new().yellow().blink();
        let scrapped_style: Style = Style::new().strikethrough().dim();

        match self.status {
            EntryStatus::Incomplete => { incomplete_style.apply_to(self.name) }
            EntryStatus::InProgress => { inprogress_style.apply_to(self.name) }
            EntryStatus::Scrapped => { scrapped_style.apply_to(self.name) }
            EntryStatus::Completed => { complete_style.apply_to(self.name) }
        }
    }

    pub fn set_status(&mut self, entry_status: EntryStatus) {
        self.status = entry_status
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }

    fn ne(&self, other: &Self) -> bool {
        self.name.ne(&other.name)
    }
}