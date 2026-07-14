use ticket_fields::TicketTitle;

// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use core::error;
use std::sync::mpsc::{Receiver, RecvError, Sender, SyncSender, TrySendError};

pub mod data;
pub mod store;

// macro a generic TicketStoreError
#[derive(Debug, thiserror::Error)]
pub enum TicketStoreError {

    // propagate TrySendError in public get/insert
    #[error("Store is full, '{0:?}' could not be inserted")] // you can magically convert debug format here I guess
    Full(TicketTitle),

    #[error("Receive error")]
    Receive(#[from] RecvError),

    #[error("General error")]
    Fail
}

// create implcit error type by constraining Results
type Result<T> = std::result::Result<T, TicketStoreError>;


#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl From<TrySendError<Command>> for TicketStoreError {
    fn from(e: TrySendError<Command>) -> Self {
        match e {
            TrySendError::Full(Command::Insert { draft, .. }) => {
                TicketStoreError::Full(draft.title)
            },
            _ => TicketStoreError::Fail
        }
    }
}


impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId> {
        let (response_sender, response_receiver) = std::sync::mpsc::sync_channel(1); // ack send and receive can be 1
        self.sender.try_send(Command::Insert { draft, response_channel: response_sender })?;
        
        Ok(response_receiver.recv()?)
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>> {
        let (response_sender, response_receiver) = std::sync::mpsc::sync_channel(1); // ack send and receive can be 1
        self.sender.try_send(Command::Get { id, response_channel: response_sender })?;
        Ok(response_receiver.recv()?)

    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    // create bounded mpsc
    let (sender, receiver) = std::sync::mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
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
                let title = draft.title.clone();
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id); // ignore the result

                // debugging server/ticket id order
                println!("Server: {:?} with ID {:?}", title, id);
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
