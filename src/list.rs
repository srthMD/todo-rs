use std::fs::{File, OpenOptions};
use std::io;
use std::io::{ErrorKind, Read, Write};

use serde::{Deserialize, Serialize};

use crate::entry::Entry;

const FILE_PATH: &str = "todo.json";

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    pub entries: Vec<Entry>,
}

impl TodoList {
    pub fn print_styled(&self) {
        if self.entries.is_empty() {
            println!("{}", "Todo list is empty.");
            return;
        }

        let mut index = 1usize;

        for entry in &self.entries {
            println!("{}. {}", index, entry.to_styled());

            index += 1;
        }
    }

    pub fn empty() -> Self {
        TodoList { entries: vec![] }
    }

    pub fn remove_multiple(&mut self, mut elements: Vec<usize>) {
        let mut offset = 0usize;

        if !elements.is_sorted() {
            elements.sort();
        }

        for indx in elements {
            self.entries.remove(indx - offset);

            offset += 1;
        }
    }
}

fn get_file(req_write: bool) -> io::Result<File> {
    let result = File::options()
        .read(true)
        .write(req_write)
        .truncate(req_write)
        .open(FILE_PATH);

    let file = match result {
        Ok(f) => f,

        Err(err) => match err.kind() {
            ErrorKind::NotFound => OpenOptions::new()
                .read(true)
                .write(true)
                .create_new(true)
                .open(FILE_PATH)?,
            _ => return Err(err),
        },
    };

    Ok(file)
}

pub fn get_todo() -> io::Result<TodoList> {
    let mut file = get_file(false)?;

    let metadata = file.metadata()?;

    if metadata.len() <= 0 {
        return Ok(TodoList::empty());
    }

    let mut buf = String::new();

    file.read_to_string(&mut buf)?;

    let deserialized: TodoList = serde_json::from_str(buf.as_str())?;

    Ok(deserialized)
}

pub fn sync_todo(list: &TodoList) -> Result<(), io::Error> {
    let mut file = get_file(true)?;

    let serialized = serde_json::to_string(list)?;

    file.write_all(serialized.as_ref())?;

    Ok(())
}
