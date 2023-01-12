use std::fs;
use std::path::Path;

pub struct Delimiters {
    pub all: Vec<DelimiterSchema>,
}

pub struct DelimiterSchema {
    pub starts_with: char,
    pub ends_with: char,
}

pub fn find_attributes_and_replace(
    is_operator: char,
    attribute: &mut String,
    mut file_content: String,
    delimiters: &Delimiters,
) -> String {
    let mut iter_count = 0;
    // we want to mutate the attribute during iteration, so we cloned it.
    let mut given_attribute = attribute.clone();
    // look for given attribute in contents where file represented as string
    let first_position = file_content.find(&given_attribute);
    // check if any byte exist, if yes, then we can continue with this file.
    if first_position > Some(0) {
        // vector collection's purpose is to not to break from the loop, stack them and iterate once more to replace content //
        let mut collection = vec![];
        // collect characters of contents into vec<char> so we can also index reference from char later on and start collecting from the first occurence.
        let char: Vec<char> = file_content
            .chars()
            .skip(*&first_position.unwrap())
            .collect();
        // the purpose of this is to keep track and replace during loop. this bad boy will always come -1 than current index
        let mut char_index = '0';
        // its a simple indicator whether we should start collecting chars into tmp_attribute_stack
        let mut should_collect_chars = 0;
        let mut tmp_attribute_stack = String::new();
        // this is a simple indicator. If goes to 1, we found exact match with delimiters while we were iterating through chars. ye, whatever.
        let mut collect = 0;
        // indicate index of which delimiter is found //
        let mut delim_ends = 0;
        for (i, c) in char.iter().enumerate() {
            // push current index of character into this bad boy, so once we figure lets say it ends its data-testid, then we can start collecting next ones //
            tmp_attribute_stack.push(**&c);
            // if we find exact attribe at the end of temporarily created stack, we get bytes, then we can start to collect what was that variable ..
            if tmp_attribute_stack.ends_with(&given_attribute) {
                collect = 1;
                tmp_attribute_stack.clear();
            }
            // then we clear up till the next findings //
            // so if our flag given to start collecting, we can do it.
            if collect > 0 {
                for (delim_index, delimiter) in delimiters.all.iter().enumerate() {
                    // if current character starts with given delimiter and index minus 1 is "=" operator, we can start to collect the characters
                    if c == &delimiter.starts_with && char[i - 1] == is_operator {
                        should_collect_chars = 1;
                        delim_ends = delim_index;
                    }
                }
                // time to figure where to stop, we simply look for our flag
                if char_index == delimiters.all[delim_ends].ends_with && char[i - 2] != is_operator
                {
                    // we clone the collected attribute values and cloned them into vec //
                    collection.push(given_attribute.clone());
                    // then clear the collection
                    given_attribute.clear();
                    // create again base attribute we look for
                    given_attribute.push_str(&attribute);
                    // stop collecting characters //
                    should_collect_chars = 0;
                    // stop till next endswith//
                    collect = 0;
                }

                if should_collect_chars > 0 {
                    given_attribute.push(*c);
                }
                char_index = *c;
                iter_count += 1;
            }
        }
        // TODO, maybe instead collecting occurrences and doing a replace, remove them on spot... it heps being %63 faster.
        for (_, replacement) in collection.iter().enumerate() {
            // since we got all into a vector whether they are empty or filled, we eliminate empty ones //
            if !attribute.eq(&replacement) {
                file_content = file_content.replace(replacement, "");
            }
        }
    }
    return file_content;
}

pub fn write_into_file(file_content: &mut String, given_attribute: String, file_path: &str) {
    fs::write(
        Path::new(file_path),
        file_content.replace(&given_attribute, ""),
    )
    .expect("TODO: panic message");
}

pub fn find_and_replace_recursively(
    given_path: &String,
    is_operator: char,
    mut attribute: String,
    delimiters: &Delimiters,
) {
    let  sort_path = fs::read_dir(given_path).unwrap();

    for path in sort_path {
        let current_path = path.unwrap().path();
        if current_path.is_dir() {

            find_and_replace_recursively(
                &current_path.display().to_string(),
                is_operator,
                attribute.clone(),
                &delimiters,
            );
            continue;
        }
        if !current_path.is_file() {
            break;
        }

        let file_path = &current_path.display().to_string();
        let contents = fs::read_to_string(file_path).expect("should have been read the file");

        let mut result =
            find_attributes_and_replace(is_operator, &mut attribute, contents, &delimiters);
        write_into_file(&mut result, attribute.clone(), &file_path.clone());
    }
}
