use clap::Parser;
use std::fs;
use strip_html_attributes::{
    find_attributes_and_replace, write_into_file, DelimiterSchema, Delimiters,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// eg. --attribute=data-testid
    #[arg(short, long)]
    attribute: String,
    /// eg. --file=/var/www/data/whatever.html
    #[arg(short, long)]
    file: String,
}
fn main() {
    // TODO accept folder and do it recursively //
    let args = Args::parse();
    let is_operator = '=';
    let mut attribute = args.attribute;
    // then add operator since this is a reserved key in html, nothing else should have been expected //
    attribute.push_str(&is_operator.to_string());
    let file_path = args.file.as_str();
    let contents = fs::read_to_string(file_path).expect("should have been read the file");

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
}
