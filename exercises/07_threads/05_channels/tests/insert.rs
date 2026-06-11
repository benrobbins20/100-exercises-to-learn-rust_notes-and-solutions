mod tests {
    use channels::data::{Ticket, TicketDraft, ticket_draft};
    use channels::store::TicketStore;
    use channels::{launch, Command};
    use std::sync::mpsc::{Receiver, Sender};
    use std::time::Duration;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    // #[should_panic(expected = "Did you actually spawn a thread? The channel is closed!")]
    fn a_thread_is_spawned() {
        let sender = launch();
        std::thread::sleep(Duration::from_millis(200));
        sender
            .send(Command::Insert(TicketDraft {
                title: ticket_title(),
                description: ticket_description(),
            }))
            // If the thread is no longer running, this will panic
            // because the channel will be closed.
            .expect("Did you actually spawn a thread? The channel is closed!");
        println!("Size of Sender<Command>: {}", std::mem::size_of::<Sender<Command>>());
        println!("{:p}", &sender);
        println!("{}", std::mem::align_of::<Sender<Command>>());
        
    }

    #[test]
    #[should_panic(expected = "All senders dropped")]
    fn kill_sender() {
        let (sender, receiver): (Sender<Command>, Receiver<Command>) = std::sync::mpsc::channel();
        let mut store = TicketStore::new();
        let r = sender.clone().send(Command::Insert(ticket_draft()));
        println!("Sent command: {:?}", r);

        // match Ok(Command::Insert(draft))
        match receiver.recv() {
            Ok(p) => println!("Received command: {:?}", p),
            Err(e) => println!("Error receiving command: {:?}", e),
        }

        // match the RecvError
        drop(sender);
        match receiver.recv() {
            Ok(_) => panic!("Uno reverso, should should be Err"),
            Err(e) => {
                let e_str = e.to_string();
                println!("Error receiving command: {:?}", e_str);
                assert_eq!(e.to_string(), "receiving on a closed channel");
            }
        }

        // force it to panic because recv is_err here
        receiver.recv().expect("All senders dropped"); // panic -> backtrace
    }

    #[test]
    fn ready() {
        // There's very little that we can check automatically in this exercise,
        // since our server doesn't expose any **read** actions.
        // We have no way to know if the inserts are actually happening and if they
        // are happening correctly.
        let move_forward = true;

        assert!(move_forward);
    }
}
