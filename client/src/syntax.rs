#[derive(Debug, PartialEq)]
pub struct Input {
    pub command: String,
    pub arguments: Vec<String>,
}

impl Input {
    pub fn new(segments: Vec<String>) -> Self {
        let mut command = String::new();
        let mut arguments: Vec<String> = vec![];

        if segments.len() > 1 {
            command = segments[0].clone();
            if segments.len() >= 2 {
                arguments = segments[1..].to_vec();
            }
        }
        Self { command, arguments }
    }

    pub fn from_string(unparsed: String) -> Self {
        let parsed_string = parse_string_to_segments(unparsed);
        Self::new(parsed_string)
    }
}

pub fn parse_string_to_segments(user_input: String) -> Vec<String> {
    let mut quote_status = false;
    let mut foo: String = String::new();

    let mut segments: Vec<String> = Vec::new();

    for c in user_input.chars() {
        // Add new segment
        if c == ' ' && !quote_status {
            segments.push(foo.clone());
            foo = String::new();
        } else if c == '\"' {
            // Quotes found
            quote_status = !quote_status;
        } else {
            // Add char to the segments vector
            foo.push(c);
        }
    }

    // String remains in the buffer
    if foo != "" {
        segments.push(foo);
    }
    segments
}
