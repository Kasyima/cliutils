use clap::Args;
use clap::{Parser, Subcommand};
use regex::Regex;
use std::fs::File;
use std::io::{self, Read, Write};
#[derive(Parser, Debug)]
#[command(
    author = "Yiangi Yui",
    version = "1.0",
    about = "A command-line text manipulation utility",
    name = "Text Manipulation Utility"
)]
pub struct Cli {
    /// Turn debugging information on.
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// Find and Replace commands.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Subcommand for handling find operations.
    Find(FindCommands),
    /// Subcommand for handling replace operations.
    Replace(ReplaceCommands),
}

/// Represents find-related commands.
#[derive(Debug, Args)]
pub struct FindCommands {
    /// Sets the input file to process
    #[clap(short = 'i', long = "input", num_args = 1.., value_delimiter = ' ')]
    pub input: Option<Vec<String>>,
    /// Sets the pattern to find.
    #[clap(short = 'p', long = "pattern")]
    pub pattern: Option<String>,
}

/// Represents replace-related commands.
#[derive(Debug, Args)]
pub struct ReplaceCommands {
    /// Sets the input file to process
    #[clap(short = 'i', long = "input", num_args = 1.., value_delimiter = ' ')]
    pub input: Option<Vec<String>>,

    /// Sets the pattern to find.
    #[clap(short = 'p', long = "pattern")]
    pub pattern: Option<String>,

    /// Sets the replacement text.
    #[clap(short = 'r', long = "replace")]
    pub replace: Option<String>,

    #[clap(short = 'c', long = "ignore-case")]
    pub ignore_case: Option<bool>,
}

fn read_file_to_string(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn find_and_replace(content: &mut String, pattern: &str, replacement: &str, ignore_case: bool) {
    let regex = if ignore_case {
        Regex::new(&format!(r"(?i){}", pattern)).unwrap()
    } else {
        Regex::new(pattern).unwrap()
    };
    *content = regex.replace_all(content, replacement).to_string();
}

fn write_string_to_file(file_path: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let mut input_files = vec!["".to_string()];
    let mut pattern_to_find = "".to_string();
    let mut replacement_text = "".to_string();
    let mut ignore_case = false;

    match args.command {
        Some(Commands::Find(command)) => {
            if let Some(input) = &command.input {
                input_files = input.to_vec();
            } else {
                println!("Please provide a file name.");
            }

            if let Some(pattern) = &command.pattern {
                pattern_to_find = pattern.clone();
                println!("Pattern: {:?}", pattern);
            } else {
                println!("Please provide a pattern.");
            }

            for input_file_path in input_files {
                let mut input_content = read_file_to_string(&input_file_path)?;
                find_and_replace(&mut input_content, &pattern_to_find, "", ignore_case);
                println!("Found Content: {:?}", input_content);
            }
        }
        Some(Commands::Replace(command)) => {
            if let Some(input) = &command.input {
                input_files = input.to_vec();
            } else {
                println!("Please provide a file name.");
            }

            if let Some(pattern) = &command.pattern {
                pattern_to_find = pattern.clone();
                println!("Pattern: {:?}", pattern_to_find);
            } else {
                println!("Please provide a pattern.");
            }

            if let Some(replace) = &command.replace {
                replacement_text = replace.clone();
                println!("Replacement text: {:?}", replacement_text);
            } else {
                println!("Please provide a replacement text.");
            }

            if let Some(ignore) = &command.ignore_case {
                ignore_case = *ignore;
                println!("Ignore case: {:?}", ignore_case);
            } else {
                println!("Please provide if case sensitive replacement.");
            }

            for input_file_path in input_files {
                let mut input_content = read_file_to_string(&input_file_path)?;
                println!("Content Before: {:?}", input_content);
                find_and_replace(
                    &mut input_content,
                    &pattern_to_find,
                    &replacement_text,
                    ignore_case,
                );
                write_string_to_file(&input_file_path, &input_content)?;
                input_content = read_file_to_string(&input_file_path)?;
                println!("Content After: {:?}", input_content);
            }
        }

        None => println!("Unknown command. Use `--help` for usage instructions."),
    };

    Ok(())
}
