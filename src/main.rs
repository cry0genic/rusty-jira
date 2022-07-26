#![allow(clippy::new_without_default)]

use crate::models::{Comment, Status, TicketDraft, TicketPatch, Title};
use std::error::Error;
use std::str::FromStr;

pub mod models;
pub mod persistence;
pub mod store;

#[derive(structopt::StructOpt)]
pub enum Command {
    Create {
        #[structopt(long)]
        description: String,
        #[structopt(long)]
        title: String,
    },
    Edit {
        #[structopt(long)]
        ticket_id: u64,
        #[structopt(long)]
        title: Option<String>,
        #[structopt(long)]
        description: Option<String>,
    },
    Delete {
        #[structopt(long)]
        ticket_id: u64,
    },
    List,
    Move {
        #[structopt(long)]
        ticket_id: u64,
        #[structopt(long)]
        status: Status,
    },
    Comment {
        #[structopt(long)]
        ticket_id: u64,
        #[structopt(long)]
        comment: String,
    },
}

impl FromStr for Status {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        let status = match s.as_str() {
            "todo" | "to-do" => Status::ToDo,
            "inprogress" | "in-progress" => Status::InProgress,
            "blocked" => Status::Blocked,
            "done" => Status::Done,
            _ => panic!("The status you specified is not valid. Valid values: todo, inprogress, blocked and done.")
        };
        Ok(status)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let command = <Command as paw::ParseArgs>::parse_args()?;

    let mut ticket_store = persistence::load();
    match command {
        Command::Create { description, title } => {
            let draft = TicketDraft {
                title: Title::new(title)?,
                description,
            };
            ticket_store.create(draft);
        }
        Command::Edit {
            ticket_id,
            title,
            description,
        } => {
            let title = title.map(Title::new).transpose()?;
            let ticket_patch = TicketPatch { title, description };
            match ticket_store.update_ticket(ticket_id, ticket_patch) {
                Some(_) => println!("Ticket {:?} was updated.", ticket_id),
                None => println!(
                    "There was no ticket associated to the ticket id {:?}",
                    ticket_id
                ),
            }
        }
        Command::Delete { ticket_id } => match ticket_store.delete(ticket_id) {
            Some(deleted_ticket) => println!(
                "The following ticket has been deleted:\n{:?}",
                deleted_ticket
            ),
            None => println!(
                "There was no ticket associated to the ticket id {:?}",
                ticket_id
            ),
        },
        Command::List => {
            let ticket_list = ticket_store
                .list()
                .into_iter()
                .map(|t| format!("{}", t))
                .collect::<Vec<String>>()
                .join("\n\n");
            println!("{}", ticket_list);
        }
        Command::Move { ticket_id, status } => {
            match ticket_store.update_ticket_status(ticket_id, status) {
                Some(_) => println!(
                    "Status of ticket {:?} was updated to {:?}",
                    ticket_id, status
                ),
                None => println!(
                    "There was no ticket associated to the ticket id {:?}",
                    ticket_id
                ),
            }
        }
        Command::Comment { ticket_id, comment } => {
            let new_comment = Comment::new(comment)?;
            match ticket_store.add_comment_to_ticket(ticket_id, new_comment) {
                Some(_) => println!("Comment has been added to ticket {:?}", ticket_id),
                None => println!(
                    "There was no ticket associated to the ticket id {:?}",
                    ticket_id
                ),
            }
        }
    }

    persistence::save(&ticket_store);
    Ok(())
}
