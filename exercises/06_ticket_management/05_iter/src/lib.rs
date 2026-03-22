use ticket_fields::{TicketDescription, TicketTitle};
use std::{slice::Iter, vec::IntoIter};

// TODO: Provide an `iter` method that returns an iterator over `&Ticket` items.
//
// Hint: just like in the previous exercise, you want to delegate the iteration to
//   the `Vec<Ticket>` field in `TicketStore`. Look at the standard library documentation
//   for `Vec` to find the right type to return from `iter`.
#[derive(Debug,Clone,PartialEq)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

// reference to struct needs lifetimes 
impl<'a> IntoIterator for &'a TicketStore {
    type Item = &'a Ticket;
    type IntoIter = Iter<'a,Ticket>;
    
    // since TicketStore is ref, iter() just works out of the box
    fn into_iter(self) -> Self::IntoIter {
        self.tickets.iter()
    }
} 


#[derive(Debug, Clone, PartialEq)]
pub struct Ticket {
    title: TicketTitle,
    description: TicketDescription,
    status: Status,
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
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }

    pub fn iter(&self) -> Iter<Ticket>{
        self.tickets.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        let tickets: Vec<&Ticket> = store.iter().collect();
        let tickets2: Vec<&Ticket> = store.iter().collect();
        assert_eq!(tickets, tickets2);
    }

    // adding a test to use the IntoIterator impl as well
    #[test]
    fn test_into_iterator() {
        let mut store = TicketStore::new();
        let mut store_clone = store.clone();
        // test partial_eq
        assert_eq!(store, store_clone);

        let t1 = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        
        let t2 = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(t1.clone());
        store.add_ticket(t2.clone());
        store_clone.add_ticket(t1.clone());
        store_clone.add_ticket(t2.clone());

        // this uses IntoIterator to borrow &TicketStore tickets
        for t in &store{
            println!("{:?}",t);
        }

        // use iter()
        for t in store_clone.iter() {
            println!("{:?}",t)
        }

        // prove both the &TicketStore into_iter and .iter() yield the same thing
        let t1_collect: Vec<_> = (&store).into_iter().collect();
        let t2_collect: Vec<_> = store_clone.iter().collect();
        assert_eq!(t1_collect,t2_collect);
    }
}
