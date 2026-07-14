use std::sync::{Arc, Barrier};

use bounded::data::{Status, Ticket, TicketDraft};
use bounded::launch;
use ticket_fields::TicketTitle;
use ticket_fields::test_helpers::{ticket_description, ticket_title};

#[test]
fn works() {
    let client = launch(5);
    let draft = TicketDraft {
        title: ticket_title(),
        description: ticket_description(),
    };
    let ticket_id = client.insert(draft.clone()).unwrap();

    let client2 = client.clone();
    let ticket = client2.get(ticket_id).unwrap().unwrap();
    assert_eq!(ticket_id, ticket.id);
    assert_eq!(ticket.status, Status::ToDo);
    assert_eq!(ticket.title, draft.title);
    assert_eq!(ticket.description, draft.description);
}


// interesting results of threads spawning and racing
#[test]
fn overload_insert() {
    let client = launch(5);
    let barrier = Arc::new(Barrier::new(101));
    
    // create an overkill collection of anonymous senders
    let clients: Vec<_> = (0..100)
        .map(|_| client.clone())
        .collect();

    let threads: Vec<_> = clients
        .into_iter() // move clients into iterator 
        .enumerate()
        // for each client, 
        .map(|(i,client)| {
            let barrier = Arc::clone(&barrier);

            std::thread::spawn(move || {
                let title = TicketTitle::try_from(i.to_string()).unwrap();
                let draft = TicketDraft {
                    title: title,
                    description: ticket_description(),
                };

                // all threads converge/block here until the barrier in main thread triggers release
                barrier.wait();
                // all threads run the insert at the same time
                let result = client.insert(draft);

                (i, result)
            })
        })
        .collect(); // collect the JoinHandles

    // final wait to add main thread to release barrier
    barrier.wait();

    // one last conceptual layer, run all the thread results and store in a new collection
    let results: Vec<_> = threads
        .into_iter() // consume JoinHandles
        .map(|thread| thread.join().unwrap())
        .collect();

    // printing is now really clean
    for result in results {
        let (i, result) = result;
        println!("Thread {i}: {result:?}");
    }

}