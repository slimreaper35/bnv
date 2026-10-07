mod path;
mod print;

use crate::print::print_all_env;
use crate::print::print_one_env;

use clap::Parser;
use clap::ValueEnum;
use owo_colors::Style;
use std::env;

#[derive(Parser, Debug)]
#[command(version, about = "A beautiful, modern environment variable explorer")]
struct Args {
    #[arg(help = "Print the value of a single environment variable")]
    name: Option<String>,

    #[arg(
        short,
        long,
        help = "Only show variables whose name matches this regex"
    )]
    grep: Option<String>,

    #[arg(
        short,
        long,
        help = "Color of the environment variables keys",
        default_value = "cyan"
    )]
    color: Color,
}

#[derive(Debug, Clone, ValueEnum)]
enum Color {
    Blue,
    Cyan,
    Green,
    Magenta,
    Purple,
    Red,
    White,
    Yellow,
}

impl Color {
    fn to_style(&self) -> Style {
        match self {
            Color::Blue => Style::new().blue(),
            Color::Cyan => Style::new().cyan(),
            Color::Green => Style::new().green(),
            Color::Magenta => Style::new().magenta(),
            Color::Purple => Style::new().purple(),
            Color::Red => Style::new().red(),
            Color::White => Style::new().white(),
            Color::Yellow => Style::new().yellow(),
        }
    }
}

fn main() {
    let mut args = Args::parse();

    match env::var("NO_COLOR") {
        Ok(_) => owo_colors::set_override(false),
        Err(_) => owo_colors::set_override(true),
    }

    let mut vars: Vec<(String, String)> = env::vars().collect();
    vars.sort_by(|a, b| a.0.cmp(&b.0));

    match args.name.take() {
        Some(name) => print_one_env(vars, name, args),
        None => print_all_env(vars, args),
    }
}
