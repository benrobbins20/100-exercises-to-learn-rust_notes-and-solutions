use crate::status::{ParseStatusError, Status};
mod status;

/* This way is a lot easier */
// #[derive(Debug, thiserror::Error)]
// pub enum TicketNewError {
//     #[error("Title cannot be empty")]
//     TitleCannotBeEmpty,
//     #[error("Title cannot be longer than 50 bytes")]
//     TitleTooLong,
//     #[error("Description cannot be empty")]
//     DescriptionCannotBeEmpty,
//     #[error("Description cannot be longer than 500 bytes")]
//     DescriptionTooLong,
//     // This essentially adds ParseStatusError to TicketNewError, and allows error propagation to Ticket things
//     #[error("{0}")]
//     InvalidStatusEnum(#[from] ParseStatusError) // THIS is the tuple, the {0} item is the ParseStatusError Struct
// }

impl From<ParseStatusError> for TicketNewError {
    fn from(value: ParseStatusError) -> Self {
        TicketNewError::InvalidStatusEnum { 
            invalid_status_cp: value.invalid_status.clone(), // must declare the struct field as pub
            source: value }
    }
}

/* ALTERNATIVE USING STRUCT UNPACKING (it kind of redundant because its mirroring struct already in status.rs*/
#[derive(Debug,thiserror::Error)]
pub enum TicketNewError {
    #[error("Title cannot be empty")]
    TitleCannotBeEmpty,
    #[error("Title cannot be longer than 50 bytes")]
    TitleTooLong,
    #[error("Description cannot be empty")]
    DescriptionCannotBeEmpty,
    #[error("Description cannot be longer than 500 bytes")]
    DescriptionTooLong,
    #[error("`{invalid_status_cp}` is not a valid status. Use one of: ToDo, InProgress, Done")]
    InvalidStatusEnum{
        invalid_status_cp: String,
        #[source]
        source: ParseStatusError
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Result<Self, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleCannotBeEmpty);
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleTooLong);
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionCannotBeEmpty);
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionTooLong);
        }

        // use the try_from impls to convert the status string to enum
        // must have statuses error included in TicketNewError for propagation
        let status = Status::try_from(status)?;

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use common::{valid_description, valid_title};
    use std::error::Error;

    use super::*;

    #[test]
    fn invalid_status() {
        let err = Ticket::new(valid_title(), valid_description(), "bitchass".into()).unwrap_err();
        assert_eq!(
            err.to_string(),
            "`bitchass` is not a valid status. Use one of: ToDo, InProgress, Done"
        );
        assert!(err.source().is_some());
    }
}
