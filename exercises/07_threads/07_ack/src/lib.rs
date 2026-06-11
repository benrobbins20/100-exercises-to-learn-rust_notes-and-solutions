use std::sync::mpsc::{Receiver, Sender};
use crate::{data::{Ticket, TicketDraft}, store::{TicketId, TicketStore}};

pub mod data;
pub mod store;

// Refer to the tests to understand the expected schema.
// sender is added to both inserts and gets for feedback
pub enum Command {
    Insert { 
        draft: TicketDraft,
        response_sender: Sender<TicketId>,
     },
    Get { 
        id: TicketId,
        response_sender: Sender<Option<Ticket>>,
     }
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();

    // launch spawns thread for server (which is looping), returns a sender instance
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: handle incoming commands as expected.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_sender,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_sender.send(id); // send a response to the reciver
            }
            Ok(Command::Get {
                id,
                response_sender,
            }) => {
                let ticket = store.get(id); // Option(&Ticket)
                let _ = response_sender.send(ticket.cloned()); // convert to owned value?
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break
            },
        }
    }
}
