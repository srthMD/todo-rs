use crate::entry::{Entry, EntryStatus};
use crate::list::{get_todo, sync_todo};
use clap::{Parser, Subcommand};
use color_eyre::owo_colors::OwoColorize;
use log::warn;
use std::io;

mod entry;
mod list;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds one or more entries to the todo list
    Add {
        /// The entry(s) to add to the list, single entries with spaces must be encased with quotes, multiple entries are seperated by spaces
        #[arg(value_name = "ENTRIES", value_delimiter = ',', num_args = 1..)]
        entries: Vec<String>,
    },

    /// Marks one or more entries with a certain status (complete, in progress, etc.)
    Set {
        /// The status of the entry or entries
        #[arg(value_enum)]
        status: EntryStatus,

        /// The entries to mark with the status, by its index
        #[arg(value_name = "ENTRIES", num_args = 1..)]
        entries: Vec<String>,
    },

    /// Removes one or more entries from the todo list
    Remove {
        /// The index or indices of the entries to remove
        #[arg(value_name = "ENTRIES", num_args = 1..)]
        entries: Vec<String>,
    },

    /// Clears the todo list
    Clear {
        /// Only clears any entry with the provided status
        #[arg(short, long, value_enum)]
        with_status: Option<EntryStatus>,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        // show todo list
        None => {
            let list = get_todo();
            match list {
                Err(err) => {
                    println!("{}: {}", "Error while running command".red().bold(), err)
                }
                Ok(ls) => {
                    ls.print_styled();
                }
            }
        }
        Some(command) => match match_command(&command) {
            Ok(_) => {}
            Err(err) => {
                println!("{}: {}", "Error while running command".red().bold(), err)
            }
        },
    }
}

fn match_command(command: &Commands) -> Result<(), io::Error> {
    let mut list = get_todo()?;

    match command {
        Commands::Add { entries } => {
            for val in entries {
                println!("{}", val);
                let entry = Entry::new(val.into());

                if list.entries.contains(&entry) {
                    println!("{} {}", entry.name, "already exists in the list");
                    continue;
                }

                list.entries.push(entry);
            }
            sync_todo(&list)?;

            Ok(())
        }

        Commands::Set { status, entries } => {
            for val in entries {
                let index = match val.parse::<usize>() {
                    Ok(i) => i,
                    Err(_) => {
                        warn!("{} {}", "Failed to parse index value ", val);
                        continue;
                    }
                };

                let entry = list.entries.get_mut(index - 1);

                if entry.is_none() {
                    warn!("Failed to get index {} in todo list", index);
                    continue;
                }

                entry.unwrap().set_status(*status);
            }
            sync_todo(&list)?;

            Ok(())
        }

        Commands::Remove { entries } => {
            let indicies = entries
                .iter()
                .map(|s_indx| s_indx.parse::<usize>().unwrap() - 1)
                .collect();

            list.remove_multiple(indicies);

            sync_todo(&list)?;

            Ok(())
        }

        Commands::Clear { with_status } => {
            match with_status {
                None => {
                    list.entries.clear();
                }
                Some(status) => list.entries.retain(|e| !(e.status == *status)),
            }

            sync_todo(&list)?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{entry::Entry, list::TodoList};

    #[test]
    fn test_remove_multiple() {
        let mut list = TodoList::empty();

        list.entries.push(Entry::new("hello".to_string()));
        list.entries.push(Entry::new("world".to_string()));
        list.entries.push(Entry::new("one".to_string()));
        list.entries.push(Entry::new("two".to_string()));

        list.remove_multiple(vec![1, 2]);

        assert!(list.entries.len() == 2);

        let two_entry = list.entries.get(1);
        assert!(two_entry.is_some());
        assert_eq!(two_entry.unwrap().name, "two".to_string())
    }
}
