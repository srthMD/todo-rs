use clap::ValueEnum;
use console::{Style, StyledObject};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
    pub name: String,
    pub status: EntryStatus,
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
            status: EntryStatus::Incomplete,
        }
    }

    pub fn to_styled(self) -> StyledObject<String> {
        match self.status {
            EntryStatus::Incomplete => {
                let incomplete_style: Style = Style::new();
                incomplete_style.apply_to(self.name)
            }
            EntryStatus::InProgress => {
                let inprogress_style: Style = Style::new().yellow().blink();
                inprogress_style.apply_to(self.name)
            }
            EntryStatus::Scrapped => {
                let scrapped_style: Style = Style::new().strikethrough().dim();
                scrapped_style.apply_to(self.name)
            }
            EntryStatus::Completed => {
                let complete_style: Style = Style::new().green().bold().underlined();
                complete_style.apply_to(self.name)
            }
        }
    }

    pub fn set_status(&mut self, entry_status: EntryStatus) {
        self.status = entry_status
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name) && self.status.eq(&other.status)
    }

    fn ne(&self, other: &Self) -> bool {
        self.name.ne(&other.name) && self.status.ne(&other.status)
    }
}

