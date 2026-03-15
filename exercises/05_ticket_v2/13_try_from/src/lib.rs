
// create basic error, in the solutions it jumps strait to a struct with thiserror which was confusing
// I used enum with tuple type. Rust structs,enum data structures and powerful but really confusing
#[derive(Debug,thiserror::Error)]
enum ParseStatusError {
    #[error("{0} is not a valid status")] // referring to the first index (tuple) of InvalidStatus
    InvalidStatus(String) // you must put the String in this on Err
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

// since &str slice is the more general form, impl this first. String can use as_str()
impl TryFrom<&str> for Status {
    type Error = ParseStatusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // take the &str in context for from_from and try to parse into Status enum
        match value.to_lowercase().trim() {
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Self::Done),
            // anything else, place the str.to_string() into the error tuple
            _ => Err(ParseStatusError::InvalidStatus(value.to_string()))
        }
    }
}

// convert to str and delegate to logic
impl TryFrom<String> for Status {
    type Error = ParseStatusError;

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
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}