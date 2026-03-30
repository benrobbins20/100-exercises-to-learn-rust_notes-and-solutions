use std::ops::AddAssign;
use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
    next_id: TicketId,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TicketId(u64);

impl AddAssign<u64> for TicketId {
    fn add_assign(&mut self, rhs: u64) {
        // self.u64 + u64
        self.0 += rhs;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
            next_id: TicketId(100), // init id to 100
        }
    }

    pub fn add_ticket(&mut self, draft: TicketDraft) -> TicketId {
        let id = self.next_id;
        self.next_id += 1; // increment the id using mutable self handle
        

        let ticket = Ticket {
            title: draft.title,
            description: draft.description,
            status: Status::ToDo,
            id: id
        };
        self.tickets.push(ticket);

        id
    }

    pub fn get(&self, id: TicketId, debug: bool) -> Option<&Ticket> {
        // idiomatic rust
        self.tickets
            .iter()
            .find(|ticket| ticket.id == id)

        // clunky iteration
        // let iter = self.tickets.iter();
        // if debug { dbg!(&iter); }
        // let mut t = None;
        // for ticket in iter {
        //     if debug { println!("ticket id: {:?}, id: {:?}", ticket.id, id) };
        //     if id == ticket.id {
        //         t = Some(ticket);
        //     }
        // }

        // t
    }
}

#[cfg(test)]
mod tests {
    use crate::{Status, TicketDraft, TicketStore};
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn works() {
        let mut store = TicketStore::new();

        let draft1 = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        let id1 = store.add_ticket(draft1.clone());
        let ticket1 = store.get(id1, false).unwrap();
        assert_eq!(draft1.title, ticket1.title);
        assert_eq!(draft1.description, ticket1.description);
        assert_eq!(ticket1.status, Status::ToDo);

        let draft2 = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        let id2 = store.add_ticket(draft2);
        let ticket2 = store.get(id2, false).unwrap();

        assert_ne!(id1, id2);
    }

    #[test]
    fn run_get() {
        let mut store = TicketStore::new(); // initializes id

        let d = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        let id = store.add_ticket(d); 
        let _ticket = store.get(id, true);
    }
}
