use std::{
    fs::File,
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
    file: String,

    #[arg(short, long)]
    before: Option<usize>,

    #[arg(short, long)]
    after: Option<usize>,
}

#[derive(Debug)]
struct GrepFile<'a> {
    pattern: Regex,
    file: &'a File,
    markers: Vec<usize>,
    lines: Vec<String>,
}

impl<'a> GrepFile<'a> {
    fn new(file: &'a File, pattern: Regex) -> Self {
        GrepFile {
            pattern,
            file: &file,
            markers: Vec::new(),
            lines: Vec::new(),
        }
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

    fn look_for_pattern(&mut self, ctx_before: usize, ctx_after: usize) {
        let reader = BufReader::new(self.file);

        for (line_number, line_) in reader.lines().enumerate() {
            let line = line_.unwrap();

            self.lines.push(line.clone());

            match self.pattern.find(&line) {
                Some(_) => self.markers.push(line_number),
                None => (),
            }
        }

        for line_number_ in self.markers.iter() {
            let line_number = *line_number_ as i32;

            if ctx_before > 0 {
                let min: i32 = line_number - ctx_before as i32;
                self.print_lines(min..line_number, Some("-"))
            }

            println!("> {}:\t{}", line_number, self.lines[*line_number_].clone());

            if ctx_after > 0 {
                let max: i32 = 1 + line_number + ctx_before as i32;
                self.print_lines(line_number..max, Some("-"));
            }

            println!("---");
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let pattern = Regex::new(&cli.pattern).unwrap();
    let file = std::fs::File::open(cli.file).unwrap();

    let mut grep_file = GrepFile::new(&file, pattern);

    grep_file.look_for_pattern(cli.before.unwrap_or(0), cli.after.unwrap_or(0));
}
