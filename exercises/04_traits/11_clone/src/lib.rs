pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    (ticket.clone(), ticket.summary())
}

#[derive(Debug, Clone)]
pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

impl Ticket {
    pub fn summary(self) -> Summary {
        Summary {
            title: self.title,
            status: self.status,
        }
    }
}

// Ticket is not directly clonable but Strings are so just manually clone over the strings to a new Self
// or macro..
// impl Clone for Ticket {
//     fn clone(&self) -> Self {
//         let title = self.title.clone();
//         let description = self.description.clone();
//         let status = self.status.clone();

//         Self { title: title, description: description, status: status }
//     }
// }

pub struct Summary {
    pub title: String,
    pub status: String,
}
