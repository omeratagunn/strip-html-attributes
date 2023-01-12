use clap::Parser;
use strip_html_attributes::{
    find_and_replace_recursively, DelimiterSchema,
    Delimiters,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// eg. --attribute=data-testid
    #[arg(short, long)]
    attribute: String,
    /// eg. --folder=/var/www/data/whatever.html
    #[arg(short, long)]
    folder: String,
}
fn main() {
    // todo filter by only these files //
    let _supported_file_types = ["js", "jsx", "ts", "tsx", "html"];

    let args = Args::parse();
    let is_operator = '=';

    let mut attribute = args.attribute;
    attribute.push_str(&is_operator.to_string());

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
    find_and_replace_recursively(&args.folder, is_operator, attribute, &delimiters);
}
