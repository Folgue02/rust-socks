use crate::syntax;
#[test]
pub fn test_basic_syntax() {
    let sample = "command argument1 \"complex argument\"".to_string();
    let output = vec![
        "command".to_string(),
        "argument1".to_string(),
        "complex argument".to_string()
    ];

    assert_eq!(output, syntax::parse_string_to_segments(sample));
}

#[test]
pub fn unfinished_quotes() {
    let sample = "command argument1 \"complex argument".to_string();
    let output = vec![
        "command".to_string(),
        "argument1".to_string(),
        "complex argument".to_string()
    ];
    assert_eq!(output, syntax::parse_string_to_segments(sample));
}
#[test]
pub fn single_unfinished_quote_at_the_end() {
    let sample = "command argument1 \"".to_string();
    let output = vec![
        "command".to_string(),
        "argument1".to_string()
    ];

    assert_eq!(output, syntax::parse_string_to_segments(sample));
}

#[test]
pub fn no_arguments() {
    let sample = "command".to_string();

    let output = vec![
        "command".to_string()
    ];
    assert_eq!(output, syntax::parse_string_to_segments(sample));
}

#[test]
pub fn empty() {
    let sample = "".to_string();
    let output: Vec<String> = vec![];
    assert_eq!(output, syntax::parse_string_to_segments(sample));
}

