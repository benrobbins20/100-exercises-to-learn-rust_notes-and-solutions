use std::sync::mpsc::{Receiver, SyncSender, TrySendError, sync_channel};

use crate::data::{Ticket, TicketDraft, TicketPatch};
use crate::store::{TicketId, TicketStore};

// custom update error for optimistic concurrency/version mismatch
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum UpdateError {
    #[error("Ticket not found")]
    NotFound,
    #[error("Version conflict: expected {expected}, but found {actual}")]
    VersionConflict {
        expected: u64,
        actual: u64,
    }
}

//combine the errors into a single error type using thiserror transparent feature
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ClientError {
    #[error("The store is overloaded")]
    Overloaded(#[from] OverloadedError),

    #[error(transparent)]
    Update(#[from] UpdateError)
}

#[derive(Debug, thiserror::Error, PartialEq)]
#[error("The store is overloaded")]
pub struct OverloadedError;

impl From<TrySendError<Command>> for OverloadedError {
    fn from(_: TrySendError<Command>) -> Self {
        OverloadedError
    }
}

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, ClientError> {
        let (response_sender, response_receiver) = sync_channel(1);
        self.sender
            .try_send(Command::Insert {
                draft,
                response_channel: response_sender,
            }).map_err(OverloadedError::from)?;

        Ok(response_receiver.recv().unwrap())
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, ClientError> {
        let (response_sender, response_receiver) = sync_channel(1);
        self.sender
            .try_send(Command::Get {
                id,
                response_channel: response_sender,
            }).map_err(OverloadedError::from)?;

        Ok(response_receiver.recv().unwrap())
    }

    pub fn update(&self, ticket_patch: TicketPatch) -> Result<(), ClientError> {
        let (response_sender, response_receiver) = sync_channel(1);

        self.sender
            .try_send(
                Command::Update { 
                    patch: ticket_patch, 
                    response_channel: response_sender 
                }).map_err(OverloadedError::from)?;
        

        Ok(response_receiver.recv().unwrap()?)
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
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
    Update {
        patch: TicketPatch,
        response_channel: SyncSender<Result<(), UpdateError>>, // either just Ok or try to propagate UpdateError
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
            Ok(Command::Update {
                patch,
                response_channel,
            }) => {
                let result = match store.get_mut(patch.id) {
                    None => Err(UpdateError::NotFound),
                    
                    // capture the ticket result and check for version mismatch
                    Some(ticket) if ticket.version != patch.expected_version => {
                        Err(UpdateError::VersionConflict { 
                            expected: patch.expected_version, 
                            actual: ticket.version })
                    },

                    // capture ticket and proceed with update if no version mismatch
                    Some(ticket) => {
                        if let Some(title) = patch.title{
                        ticket.title = title;
                    }
                        if let Some(description) = patch.description {
                            ticket.description = description;
                        }
                        if let Some(status) = patch.status {
                            ticket.status = status;
                        }
                        ticket.version += 1;

                        Ok(())
                    }
                };
                let _ = response_channel.send(result);
            }

            // receiver.recv() returns general error when channel closes, capture nothing from the error
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
