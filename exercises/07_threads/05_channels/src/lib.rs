use std::sync::mpsc::{Receiver, Sender};

use crate::{data::TicketDraft, store::TicketStore};

pub mod data;
pub mod store;

#[derive(Clone, Debug)]
pub enum Command {
    Insert(TicketDraft),
}

// Start the system by spawning the server thread.
// It returns a `Sender` instance which can then be used
// by one or more clients to interact with the server.
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    // run until recv returns error
    while let Ok(cmd) = receiver.recv() {
        match cmd {
            // unpack the data in the Command variant
            Command::Insert(draft) => {
                store.add_ticket(draft);
                store.print_tickets();
            }
        }
    }
}
