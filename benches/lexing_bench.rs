use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use std::path::PathBuf;
use strip_html_attributes::{find_attributes_and_replace, DelimiterSchema, Delimiters};

fn do_something(c: &mut Criterion) {
    let srcdir = PathBuf::from("./benches/test/big_component.js");
    let string_path = fs::canonicalize(&srcdir);
    let binding = string_path.unwrap();
    let file_path = binding.to_str().unwrap();

    let contents = fs::read_to_string(file_path).expect("should have been read the file");
    let is_operator = '=';

    let mut attribute = String::from("data-testid");
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

    c.bench_function("lexing algorythm", |b| {
        b.iter(|| {
            find_attributes_and_replace(is_operator, &mut attribute, contents.clone(), &delimiters)
        })
    });
}

criterion_group!(benches, do_something);
criterion_main!(benches);
