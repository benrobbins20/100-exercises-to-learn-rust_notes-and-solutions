// general error struct that just prints the string
#[derive(Debug, thiserror::Error,Clone)]
#[error("`{invalid_status}` is not a valid status. Use one of: ToDo, InProgress, Done")]
pub struct StatusError {
    invalid_status: String,
}

#[derive(Debug,PartialEq,Clone)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TryFrom<&str> for Status {
    type Error = StatusError;

    fn try_from(status: &str) -> Result<Self, Self::Error> {
        // take the &str in context for from_from and try to parse into Status enum
        match status.to_lowercase().trim() {
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Self::Done),
            // anything else, place the str.to_string() into the error tuple
            _ => Err(StatusError { invalid_status: status.to_string() })
        }
    }
}

// convert to str and delegate to logic
impl TryFrom<String> for Status {
    type Error = StatusError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("aosibgriaeubrvb");
        // basically prints the same thing, Status results in Err(StatusError)
        dbg!(&status); // this is wrapped with result 
        let s = status.clone();
        dbg!(&s.unwrap_err()); // this is StatusError
        assert!(status.is_err());
    }
}
