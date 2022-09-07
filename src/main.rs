use regex::Regex;
// use std::env;
use std::fs;

enum TabulatedLine {
    Comment(String),
    BlankLine,
    TabulatedItem(Vec<String>),
    Header(Vec<String>),
}

type Lines = Vec<TabulatedLine>;

type ColumnWidths = Vec<u64>;

fn main() {
    // let args: Vec<String> = env::args().collect();
    iterate_over_lines();
}

// 1. Separate each line into columns - with a union that may include an unformatted line
// 2. separate data structure; have <Vec<integer>> as column widths
// 3. write out file that separates each line into columns

fn iterate_over_lines() {
    let header_regex = Regex::new(r"#:?(?:\s*<.*>)$").unwrap();
    let file = fs::read_to_string("./test-tab.txt").unwrap();
    let lines = file.lines();
    let mut parsed_lines: Lines = Vec::new();
    let mut widths: ColumnWidths = Vec::new();
    // let mut column_widths: Vec<ColumnWidths> = vec![];
    for line in lines {
        if line.is_empty() {
            parsed_lines.push(TabulatedLine::BlankLine);
        } else if header_regex.is_match(line) {
            let mut header_columns: Vec<String> = Vec::new();
            let mut cursor: usize = 0;
            loop {
                let found_column = line[cursor..].find("<").and_then(|start_index| {
                    line[cursor + start_index..]
                        .find(">")
                        .map(|end_index| (start_index, end_index))
                });
                match found_column {
                    Some((start_index, end_index)) => {
                        let column = line
                            [cursor + start_index + 1..cursor + start_index + end_index]
                            .to_string();
                        header_columns.push(column);
                        cursor += start_index + end_index + 1;
                    }
                    None => {
                        break;
                    }
                }
                if cursor >= line.len() {
                    break;
                }
            }
            for (index, column) in header_columns.iter().enumerate() {
                if index >= widths.len() {
                    widths.push(column.len() as u64);
                } else {
                    widths[index] = std::cmp::max(widths[index], column.len() as u64);
                }
            }
            parsed_lines.push(TabulatedLine::Header(header_columns))
        } else if line.starts_with("#") {
            parsed_lines.push(TabulatedLine::Comment(line.to_string()))
        } else {
            let item_columns = line.split_whitespace();
            let item_columns_2 = item_columns.clone();
            for (index, column) in item_columns.enumerate() {
                if index >= widths.len() {
                    widths.push(column.len() as u64);
                } else {
                    widths[index] = std::cmp::max(widths[index], column.len() as u64);
                }
            }
            parsed_lines.push(TabulatedLine::TabulatedItem(
                item_columns_2.map(|s| s.to_string()).collect(),
            ))
        }
    }
    let mut output_lines: Vec<String> = Vec::new();
    for line in parsed_lines.iter() {
        match line {
            TabulatedLine::Comment(comment) => {
                output_lines.push(comment.to_string());
            }
            TabulatedLine::BlankLine => {
                output_lines.push("".to_string());
            }
            TabulatedLine::Header(header_columns) => {
                let mut output_line = String::new();
                output_line.push_str("#: ");
                for (index, column) in header_columns.iter().enumerate() {
                    let pad_width = if index == 0 {
                        widths[index] as usize - 2 - column.len()
                    } else {
                        widths[index] as usize + 1 - column.len()
                    };
                    output_line.push_str("<");
                    output_line.push_str(&column);
                    output_line.push_str(">");
                    output_line.push_str(&" ".repeat(pad_width));
                }
                output_lines.push(output_line);
            }
            TabulatedLine::TabulatedItem(item_columns) => {
                let mut output_line = String::new();
                for (index, column) in item_columns.iter().enumerate() {
                    let pad_width = widths[index] as usize - column.len() + 3;
                    output_line.push_str(&column);
                    output_line.push_str(&" ".repeat(pad_width));
                }
                output_lines.push(output_line);
            }
        }
    }
    println!("{}", output_lines.join("\n"));
}
