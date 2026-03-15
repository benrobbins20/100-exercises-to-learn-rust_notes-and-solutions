
/* Description error strings */
#[derive(Debug,thiserror::Error)]
pub enum TicketDescriptionError {
    #[error("The description cannot be empty")]
    DescriptionCannotBeEmpty,
    #[error("The description cannot be longer than 500 bytes")]
    DescriptionTooLong,
}

/* Description Struct and impl*/
#[derive(Debug,Clone, PartialEq)]
pub struct TicketDescription(String);

impl TryFrom<&str> for TicketDescription {
    type Error = TicketDescriptionError;

    fn try_from(desc: &str) -> Result<Self, Self::Error> {
        check_description(&desc)?; // will return early with failure if it doesn't pass check
        Ok(Self(desc.to_string()))
    }    
}

impl TryFrom<String> for TicketDescription {
    type Error = TicketDescriptionError;

    fn try_from(desc: String) -> Result<Self, Self::Error> {
        check_description(&desc)?;
        Ok(Self(desc))
    }
}

// private method to invoke an Err early in the TryFrom impl
// sufficient to just return empty Ok(()) if validation passes
fn check_description(desc: &str) -> Result<(),TicketDescriptionError> {
    if desc.is_empty() {
        return Err(TicketDescriptionError::DescriptionCannotBeEmpty);
    }
    if desc.len() > 500 {
        return Err(TicketDescriptionError::DescriptionTooLong)
    }
    Ok(()) // description is okay, will not return early ? in TryFrom
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let description = TicketDescription::try_from("A description".to_string()).unwrap();
        assert_eq!(description.0, "A description");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketDescription::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The description cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let description = "At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus. Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae. Itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut perferendis doloribus asperiores repellat.".to_string();
        let err = TicketDescription::try_from(description).unwrap_err();
        assert_eq!(
            err.to_string(),
            "The description cannot be longer than 500 bytes"
        );
    }

    #[test]
    fn test_try_from_str() {
        let description = TicketDescription::try_from("A description").unwrap();
        assert_eq!(description.0, "A description");
    }
}
