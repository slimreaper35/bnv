use crate::Args;
use crate::path::handle_path;

use owo_colors::OwoColorize;
use regex::Regex;
use std::io::Write;
use std::process;
use std::process::Command;
use std::process::Stdio;

const EQ: char = '=';
const NEWLINE: char = '\n';
const UNDERSCORE: char = '_';

pub fn print_one_env(vars: Vec<(String, String)>, name: String, args: Args) {
    let var = vars.iter().find(|(key, _)| key == &name);

    if let Some(var) = var {
        let record = format!(
            "{} {} {}",
            name.style(args.color.to_style()),
            EQ.dimmed(),
            format_value(&name, var.1.clone())
        );
        println!("{}", record);
    } else {
        eprintln!("Not found: `{name}`");
        process::exit(1);
    }
}

pub fn print_all_env(vars: Vec<(String, String)>, args: Args) {
    let mut output = String::new();
    let mut last_prefix: Option<String> = None;

    let color_style = args.color.to_style();
    let maybe_regex = get_maybe_regex(args.grep);

    for (key, value) in vars {
        if value.is_empty() || key.starts_with(UNDERSCORE) {
            continue;
        }

        if let Some(regex) = &maybe_regex
            && !regex.is_match(&key)
        {
            continue;
        }

        let prefix = first_prefix(&key);
        if let Some(ref last) = last_prefix
            && last != &prefix
        {
            output.push(NEWLINE);
        }
        last_prefix = Some(prefix);

        let record = format!(
            "{} {} {}",
            key.style(color_style),
            EQ.dimmed(),
            format_value(&key, value)
        );

        output.push_str(&record);
        output.push(NEWLINE);
    }

    maybe_page_output(output);
}

fn format_value(key: &str, value: String) -> String {
    if key == "PATH" {
        return handle_path(value);
    }

    value
}

fn get_maybe_regex(grep: Option<String>) -> Option<Regex> {
    match grep {
        Some(grep) => match Regex::new(&grep) {
            Ok(regex) => Some(regex),
            Err(_) => {
                eprintln!("Invalid regex: `{grep}`");
                process::exit(1);
            }
        },
        None => None,
    }
}

fn first_prefix(key: &str) -> String {
    key.split(UNDERSCORE).next().unwrap_or(key).to_string()
}

fn maybe_page_output(content: String) {
    let args = ["-R", "-F", "-X"];
    let command = Command::new("less").args(args).stdin(Stdio::piped()).spawn();

    match command {
        Ok(mut child) => {
            if let Some(mut stdin) = child.stdin.take() {
                let _ = stdin.write_all(content.as_bytes());
            }
            let _ = child.wait();
        }
        Err(_) => {
            print!("{content}");
        }
    }
}
