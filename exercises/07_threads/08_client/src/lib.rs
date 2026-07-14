use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    // sender sends Command messages
    sender: Sender<Command>,
}

impl TicketStoreClient {
    // Feel free to panic on all errors, for simplicity.
    pub fn insert(&self, draft: TicketDraft) -> TicketId {
        let (temp_send, temp_recv) = std::sync::mpsc::channel();
        let command = Command::Insert {
            draft,
            response_channel: temp_send,
        };
        let _ = self.sender.send(command);

        // return ID, panic on SendError
        temp_recv.recv().unwrap()
    }

    pub fn get(&self, id: TicketId) -> Option<Ticket> {
        let (temp_send, temp_recv) = std::sync::mpsc::channel();
        let command = Command::Get { 
            id: id, 
            response_channel: temp_send
        };
        let _ = self.sender.send(command);

        // return ticket
        temp_recv.recv().unwrap()
    }
}

pub fn launch() -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

// No longer public! This becomes an internal detail of the library now.
enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
