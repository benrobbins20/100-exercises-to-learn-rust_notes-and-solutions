mod ticket {
    #[derive(Debug)]
    pub struct Ticket {
        title: String,
        description: String,
        status: String,
    }

    impl Ticket {
        pub fn new(title: String, description: String, status: String) -> Ticket {
            if title.is_empty() {
                panic!("Title cannot be empty");
            }
            if title.len() > 50 {
                panic!("Title cannot be longer than 50 bytes");
            }
            if description.is_empty() {
                panic!("Description cannot be empty");
            }
            if description.len() > 500 {
                panic!("Description cannot be longer than 500 bytes");
            }
            if status != "To-Do" && status != "In Progress" && status != "Done" {
                panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
            }

            Ticket {
                title,
                description,
                status,
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::any::Any;

    // access the ticket module from the super()/global scope
    use super::ticket::Ticket;

    // not doing any work in test so just comment out the attribute
    #[test]
    fn should_not_be_possible() {
        // the Ticket impl method is public and the struct is public so can write parameters
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        
        // cannot access the fields of the scruct directly
        // assert_eq!(ticket.description, "A description");

        
        println!("{:?}", ticket.type_id()); // hash of the struct Ticket
        println!("{:p}", &ticket); // address
        println!("Size of Ticket: {}", std::mem::size_of::<Ticket>());
        println!("Alignment of Ticket: {}", std::mem::align_of::<Ticket>()); // essentially the allocated memory block offset
        // derive debug is useful for printing the whole object/struct
        println!("{:?}", ticket); // object information of the local ticket instance
    }
    // #[test]
    fn encapsulation_cannot_be_violated() {
        // this is directly writing the parameters not, using the new() method, so this is not legal
        // let ticket = Ticket {
        //     title: "A title".into(),
        //     description: "A description".into(),
        //     status: "To-Do".into(),
        // };
    }
}
