// Get the input
// Check if a message exists by splitting on the space
// Check message has a newline at the end by splitting on it
// If more than one

#[derive(Eq, PartialEq, Debug)]
pub enum Command {
    Publish(String),
    Retrieve,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Error {
    UnknownVerb,
    UnexpectedPayload,
    MissingPayload,
    EmptyMessage,
    IncompleteMessage,
    TrailingData,
}

pub fn parse(input: &str) -> Result<Command, Error> {
    if let Some(pos) = input.rfind('\n') {
        if !((pos + 1) == input.len()) {
            return Err(Error::TrailingData);
        }
    } else {
        return Err(Error::IncompleteMessage);
    }

    let mut split = input.splitn(2, ' ');

    if let Some(verb) = split.next() {
        match verb.trim() {
            "RETRIEVE" => {
                if split.next() == None {
                    Ok(Command::Retrieve)
                } else {
                    Err(Error::UnexpectedPayload)
                }
            }
            "PUBLISH" => {
                if let Some(payload) = split.next() {
                    Ok(Command::Publish(payload.into()))
                } else {
                    Err(Error::MissingPayload)
                }
            }
            "" => Err(Error::EmptyMessage),
            _ => Err(Error::UnknownVerb),
        }
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_publish() {
        let input = "PUBLISH hello\n";
        let result = parse(input);
        let expected = Ok(Command::Publish("hello\n".into()));
        assert_eq!(result, expected);
    }
}
