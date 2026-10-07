use std::path::Path;

use owo_colors::OwoColorize;

const SEMICOLON: char = ';';
const COLON: char = ':';
const NEWLINE: &str = "\n";
const ARROW: &str = "→";

pub fn handle_path(value: String) -> String {
    let separator = if cfg!(windows) { SEMICOLON } else { COLON };

    let mut tree_output = String::new();

    for path in value.split(separator) {
        if Path::new(path).exists() {
            let line = format!("{NEWLINE}  {} {}", ARROW.dimmed(), path);
            tree_output.push_str(&line);
        } else {
            let line = format!(
                "{NEWLINE}  {} {} {}",
                ARROW.dimmed(),
                path.strikethrough().dimmed(),
                "[DOES NOT EXIST]".yellow().bold()
            );
            tree_output.push_str(&line);
        }
    }

    tree_output
}
