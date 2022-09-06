use regex::Regex;
use std::env;
use std::fs;

enum TabulatedLine {
    Comment,
    BlankLine,
    TabulatedItem(Vec<String>),
    Header(Vec<String>),
}

type Lines = Vec<TabulatedLine>;

type ColumnWidths = Vec<u64>;

fn main() {
    // let args: Vec<String> = env::args().collect();
    iterate_over_lines();
    println!("Hello, world!");
}

// 1. Separate each line into columns - with a union that may include an unformatted line
// 2. separate data structure; have <Vec<integer>> as column widths
// 3. write out file that separates each line into columns

fn iterate_over_lines() {
    let header_regex = Regex::new(r"#:?(?:\s*\<.*\>)$").unwrap();
    let file = fs::read_to_string("./test-tab.txt").unwrap();
    let lines = file.lines();
    let mut parsed_lines: Lines = Vec::new();
    // let mut column_widths: Vec<ColumnWidths> = vec![];
    for line in lines {
        if line.is_empty() {
            parsed_lines.push(TabulatedLine::BlankLine);
        } else if line.starts_with("#:") && header_regex.is_match(line) {
            let mut header_columns: Vec<String> = Vec::new();
            let mut cursor: usize = 0;
            let mut line_tmp = line.to_string();
            while cursor < line_tmp.len(){
                let open = line_tmp[cursor..].find("<");
                match open {
                    
                }
                // let close = line_tmp[cursor + open..].find(">").unwrap();
            }
            line_tmp.remove_matches(pat)
            line.split
            parsed_lines.push(TabulatedLine::Header(vec![]))
        } else if line.starts_with("#") {
            parsed_lines.push(TabulatedLine::Comment)
        }
    }
}
