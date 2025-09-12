use anyhow::{bail, Result};
use std::{collections::VecDeque, fs};

pub fn process_response_files(args: &mut VecDeque<String>) -> Result<()> {
    handle_legacy_response_files(args)?;
    load_response_files(args)?;
    Ok(())
}

/// Function that converts "@response.file" to "--response response.file".
fn handle_legacy_response_files(args: &mut VecDeque<String>) -> Result<()> {
    let mut new_args = VecDeque::with_capacity(args.len());

    while let Some(arg) = args.pop_front() {
        let mut chars = arg.chars();

        if arg.is_empty() || chars.next().unwrap() != '@' || arg.contains(char::is_whitespace) {
            new_args.push_back(arg);
            continue;
        }
        if arg.len() == 1 {
            bail!("'@' must be followed by a response file\n");
        }
        if !args.is_empty() && !is_param(&args[0]) {
            bail!("'@' accepts only one argument\n");
        }

        new_args.push_back(String::from("--response"));
        // The "next" function consumes the first char, so collect
        // here returns arg without '@'.
        new_args.push_back(chars.collect());
    }

    args.append(&mut new_args);

    Ok(())
}

fn load_response_files(args: &mut VecDeque<String>) -> Result<()> {
    let mut new_args = VecDeque::with_capacity(args.len());

    while let Some(arg) = args.pop_front() {
        if arg != "--response" {
            new_args.push_back(arg);
            continue;
        }

        if args.is_empty() {
            bail!("'--response' must be followed by a response file\n");
        }

        let mut response_files = Vec::new();
        while let Some(next_arg) = args.pop_front() {
            if is_param(&next_arg) {
                args.push_front(next_arg);
                break;
            }
            response_files.push(next_arg);
        }

        for file in response_files.iter().rev() {
            let response_file = ResponseFileParser::parse(file)?;
            // We push the new args from the response file into "args" instead
            // of "new_args", this way recursive response files are possible.
            // We in the front instead of the back of "args" because that is the
            // vanilla behaviour. Also note the that we have to iterate the response
            // file backwards, because we are pushing in the front.
            for new_arg in response_file.args.into_iter().rev() {
                args.push_front(new_arg);
            }
        }
    }

    args.append(&mut new_args);

    Ok(())
}

fn is_param(str: &str) -> bool {
    str.starts_with('-') && !str.contains(char::is_whitespace)
}

/// A response file is just a text file that may store
/// additional command line parameters.
struct ResponseFile {
    args: VecDeque<String>,
}

struct ResponseFileParser;

impl ResponseFileParser {
    fn parse(file: &str) -> Result<ResponseFile> {
        let file_content = fs::read(file)?;
        let file_size = file_content.len();

        let mut args = VecDeque::new();
        let mut i: usize = 0;

        while i < file_size {
            // Skip past space characters to the next argument
            while i < file_size && (file_content[i] as char).is_whitespace() {
                i += 1;
            }

            if i == file_size {
                // Reached end of file.
                break;
            }

            let mut j = i + 1;

            // If the next argument is enclosed in quote marks, treat
            // the contents as a single argument. This allows long filenames
            // to be specified.
            if (file_content[i] as char) == '"' {
                while j < file_size
                    && (file_content[j] as char) != '"'
                    && (file_content[j] as char) != '\n'
                {
                    j += 1;
                }

                if j == file_size || (file_content[j] as char) == '\n' {
                    bail!("Quotes unclosed in response file '{file}'\n");
                }

                // Skip the first character (")
                i += 1;
            } else {
                while j < file_size && !(file_content[j] as char).is_whitespace() {
                    j += 1;
                }
            }

            let param = std::str::from_utf8(&file_content[i..j]).unwrap().to_owned();
            args.push_back(param);
            i = j + 1;
        }

        Ok(ResponseFile { args })
    }
}
