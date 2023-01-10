use std::fs;
use std::path::Path;
use std::path::PathBuf;
use strip_html_attributes::{
    find_attributes_and_replace, write_into_file, DelimiterSchema, Delimiters,
};
#[test]
fn test_with_given_delimiters_in_populated_file() {
    let srcdir = PathBuf::from("./example/components/suchbutton.js");
    let string_path = fs::canonicalize(&srcdir);
    let binding = string_path.unwrap();
    let file_path = binding.to_str().unwrap();

    let mut contents = fs::read_to_string(file_path).expect("should have been read the file");
    let is_operator = '=';

    let mut prevContents = contents.clone();
    let mut attribute = String::from("data-testid");
    attribute.push_str(&is_operator.to_string());

    let attr = attribute.clone();
    let delimiters = Delimiters {
        all: vec![
            DelimiterSchema {
                starts_with: '\u{27}',
                ends_with: '\u{27}',
            },
            DelimiterSchema {
                starts_with: '{',
                ends_with: '}',
            },
            DelimiterSchema {
                starts_with: '"',
                ends_with: '"',
            },
        ],
    };

    let mut result = find_attributes_and_replace(is_operator, &mut attribute, contents, delimiters);
    write_into_file(&mut result, attribute, file_path);

    if result.find(&attr) > Some(0) {
        assert!(false);
    }

    fs::write(Path::new(file_path), prevContents).expect("TODO: panic message");
}
#[test]
fn test_with_given_delimiters_in_not_populated_file() {
    let srcdir = PathBuf::from("./example/components/suchinput.js");
    let string_path = fs::canonicalize(&srcdir);
    let binding = string_path.unwrap();
    let file_path = binding.to_str().unwrap();

    let mut contents = fs::read_to_string(file_path).expect("should have been read the file");
    let is_operator = '=';

    let mut prevContents = contents.clone();
    let mut attribute = String::from("data-testid");
    attribute.push_str(&is_operator.to_string());

    let attr = attribute.clone();
    let delimiters = Delimiters {
        all: vec![
            DelimiterSchema {
                starts_with: '\u{27}',
                ends_with: '\u{27}',
            },
            DelimiterSchema {
                starts_with: '{',
                ends_with: '}',
            },
            DelimiterSchema {
                starts_with: '"',
                ends_with: '"',
            },
        ],
    };

    let mut result = find_attributes_and_replace(is_operator, &mut attribute, contents, delimiters);
    write_into_file(&mut result, attribute, file_path);

    if result.find(&attr) > Some(0) {
        assert!(false);
    }

    fs::write(Path::new(file_path), prevContents).expect("TODO: panic message");
}
#[test]
fn test_with_html_file() {
    let srcdir = PathBuf::from("./example/components/suchanhtmlfile.html");
    let string_path = fs::canonicalize(&srcdir);
    let binding = string_path.unwrap();
    let file_path = binding.to_str().unwrap();

    let mut contents = fs::read_to_string(file_path).expect("should have been read the file");
    let is_operator = '=';

    let mut prevContents = contents.clone();
    let mut attribute = String::from("data-testid");
    attribute.push_str(&is_operator.to_string());

    let attr = attribute.clone();
    let delimiters = Delimiters {
        all: vec![
            DelimiterSchema {
                starts_with: '\u{27}',
                ends_with: '\u{27}',
            },
            DelimiterSchema {
                starts_with: '{',
                ends_with: '}',
            },
            DelimiterSchema {
                starts_with: '"',
                ends_with: '"',
            },
        ],
    };

    let mut result = find_attributes_and_replace(is_operator, &mut attribute, contents, delimiters);
    write_into_file(&mut result, attribute, file_path);

    if result.find(&attr) > Some(0) {
        assert!(false);
    }

    fs::write(Path::new(file_path), prevContents).expect("TODO: panic message");
}
