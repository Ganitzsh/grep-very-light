use std::{
    io::{BufRead, BufReader},
    ops::Range,
};

use clap::Parser;
use regex::Regex;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'e', long)]
    pattern: String,

    #[arg(short, long)]
    input: Option<String>,

    #[arg(short, long)]
    before: Option<usize>,

    #[arg(short, long)]
    after: Option<usize>,
}

#[derive(Debug)]
struct Grep {
    markers: Vec<usize>,
    lines: Vec<String>,
}

impl Grep {
    fn new<T: BufRead + Sized>(inner: T, pattern: Regex) -> Self {
        let reader = BufReader::new(inner);

        let mut lines: Vec<String> = Vec::new();
        let mut markers: Vec<usize> = Vec::new();

        for (line_number, line_) in reader.lines().enumerate() {
            let line = line_.unwrap();

            lines.push(line.clone());

            match pattern.find(&line) {
                Some(_) => markers.push(line_number),
                None => (),
            }
        }

        Grep { markers, lines }
    }

    fn print_lines(&self, range: Range<i32>, prefix: Option<&str>) {
        for i in range {
            if i >= 0 && i <= self.lines.len() as i32 - 1 {
                println!(
                    "{} {}:\t{}",
                    prefix.unwrap_or(""),
                    i,
                    self.lines[i as usize].clone()
                );
            }
        }
    }

    fn print_results(&mut self, ctx_before: usize, ctx_after: usize) {
        for line_number_ in self.markers.iter() {
            let line_number = *line_number_ as i32;

            if ctx_before > 0 {
                let min: i32 = line_number - ctx_before as i32;
                self.print_lines(min..line_number, Some("-"))
            }
            self.print_lines(line_number..line_number + 1, Some(">"));
            if ctx_after > 0 {
                let max: i32 = 1 + line_number + ctx_after as i32;
                self.print_lines((line_number + 1)..max, Some("-"));
            }

            println!();
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let pattern = Regex::new(&cli.pattern).unwrap();
    let input = cli.input.unwrap_or("-".to_string());

    if input == "-".to_string() {
        let stdin = std::io::stdin();
        let handle = stdin.lock();
        let mut grep = Grep::new(handle, pattern);

        grep.print_results(cli.before.unwrap_or(0), cli.after.unwrap_or(0));
    } else {
        let file = std::fs::File::open(input).unwrap();
        let handle = BufReader::new(file);
        let mut grep = Grep::new(handle, pattern);

        grep.print_results(cli.before.unwrap_or(0), cli.after.unwrap_or(0));
    }
}
