use patch::data::{Status, Ticket, TicketDraft, TicketPatch};
use patch::launch;
use ticket_fields::{TicketDescription, TicketTitle};
use ticket_fields::test_helpers::{ticket_description, ticket_title};

#[test]
fn works() {
    let client = launch(5);
    let draft = TicketDraft {
        title: ticket_title(),
        description: ticket_description(),
    };
    let ticket_id = client.insert(draft.clone()).unwrap();

    let ticket = client.get(ticket_id).unwrap().unwrap();
    assert_eq!(ticket_id, ticket.id);
    assert_eq!(ticket.status, Status::ToDo);
    assert_eq!(ticket.title, draft.title);
    assert_eq!(ticket.description, draft.description);

    let patch = TicketPatch {
        id: ticket_id,
        title: None,
        description: None,
        status: Some(Status::InProgress),
        expected_version: ticket.version,
    };
    client.update(patch).unwrap();

    let ticket = client.get(ticket_id).unwrap().unwrap();
    assert_eq!(ticket.id, ticket_id);
    assert_eq!(ticket.status, Status::InProgress);
}

#[test]
fn test_optimistic_concurrency() {
    use patch::{ClientError, UpdateError};

    let client = launch(5);
    let draft = TicketDraft {
        title: ticket_title(),
        description: ticket_description(),
    };
    let ticket_id = client.insert(draft.clone()).unwrap();
    let ticket = match client.get(ticket_id).unwrap() {
        Some(ticket) => {
            assert_eq!(ticket.version, 1);
            ticket
        }
        None => {
            panic!("Ticket not found");
        }
    };

    let patch1 = TicketPatch {
        id: ticket_id.clone(),
        title: TicketTitle::try_from("Bitchass").ok(),
        description: TicketDescription::try_from("bitchass").ok(),
        status: Some(Status::InProgress),
        expected_version: ticket.version,
    };

    let patch2 = TicketPatch {
        id: ticket_id.clone(),
        title: TicketTitle::try_from("Bitchass").ok(),
        description: TicketDescription::try_from("bitchass").ok(),
        status: Some(Status::InProgress),
        expected_version: ticket.version,
    };

    // this one passes
    client.update(patch1).unwrap();

    // this one errors with version conflict
    let failure = client.update(patch2).unwrap_err();

    assert_eq!(
        failure,
        ClientError::Update(UpdateError::VersionConflict {
            expected: 1,
            actual: 2,
        })
    );

    let patch_ticket = client.get(ticket_id).unwrap().unwrap();
    assert_eq!(patch_ticket.version, 2);
}
