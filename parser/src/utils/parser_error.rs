use crate::parser::ErrorKind;
use colored::Colorize;
use lexer::Token;
use std::cmp::min;

const ERROR_OUT_MAX_WIDTH: usize = 40;

#[derive(Debug, PartialEq)]
pub struct ParserError<'a> {
    pub kind: ErrorKind,
    pub token: Token,
    pub source: &'a str,
}

impl<'a> ParserError<'a> {
    pub fn print(&self) {
        let mut lines = self.source.split('\n');
        let mut prev_line: Option<&str>;
        let mut curr_line: Option<&str> = None;
        let next_line: Option<&str>;
        let mut curr_line_start: usize = 0;
        let mut curr_line_nbr: usize = 0;

        loop {
            curr_line_nbr += 1;
            prev_line = curr_line.take();
            curr_line = lines.next();

            if curr_line.is_none() {
                unreachable!("Could not find token inside ParserError");
            }

            if let Some(line) = curr_line {
                let line_end = curr_line_start + line.len();

                if self.token.start < line_end {
                    if self.token.end > line_end {
                        todo!("The token spans multiple lines");
                    }
                    break;
                }

                curr_line_start += line.len() + 1; // +1 to include \n
            }
        }

        next_line = lines.next();

        let mut out_str = String::with_capacity(5 * (ERROR_OUT_MAX_WIDTH + 1));
        let digit_count = count_digits(curr_line_nbr + 1);
        let skip_left: usize = if curr_line_start + ERROR_OUT_MAX_WIDTH < self.token.end {
            self.token.start - curr_line_start
        } else {
            0
        };

        eprintln!("An error occured:");

        let err_kind_msg: String = match self.kind {
            ErrorKind::InternalError => "Internal error".to_owned(),
            ErrorKind::InvalidToken => format!(
                "Invalid token: {}",
                &self.source[self.token.start..self.token.end]
            ),
        };

        if let Some(l) = prev_line {
            write_line(&mut out_str, l, digit_count, curr_line_nbr - 1, skip_left);
        }

        if let Some(l) = curr_line {
            // Write the source line, with the token highlighted in red
            let start = (self.token.start + skip_left).saturating_sub(curr_line_start);
            let len = self.token.end - self.token.start;

            // Split the line around the token
            let line_start = &l[..start];
            let line_token = &l[start..start + len].red();
            let line_end = &l[start + len..];

            // Write line number and line text
            for _ in 0..(digit_count - count_digits(curr_line_nbr)) {
                out_str.push(' ');
            }
            out_str.push_str(&format!("{}| ", curr_line_nbr));
            out_str.push_str(line_start);
            out_str.push_str(&line_token.to_string());
            out_str.push_str(line_end);
            out_str.push('\n');

            // Write caret line
            for _ in 0..digit_count {
                out_str.push(' ');
            }
            out_str.push_str("| ");
            for _ in 0..start {
                out_str.push(' ');
            }
            let mut carets = String::with_capacity(len);
            for _ in 0..len {
                carets.push('^');
            }
            out_str.push_str(&carets.red().to_string());
            out_str.push('\n');

            // Write error message under the caret
            for _ in 0..digit_count {
                out_str.push(' ');
            }
            out_str.push_str("| ");
            for _ in 0..start {
                out_str.push(' ');
            }

            out_str.push_str(&err_kind_msg.red().to_string());
            out_str.push('\n');
        }

        if let Some(l) = next_line {
            write_line(&mut out_str, l, digit_count, curr_line_nbr + 1, skip_left);
        }

        eprintln!("{}", out_str);
    }
}

fn count_digits(mut num: usize) -> usize {
    let mut count = 1;
    while num >= 10 {
        num /= 10;
        count += 1;
    }
    count
}

fn write_line(
    out_str: &mut String,
    curr_line: &str,
    digit_count: usize,
    line_nbr: usize,
    skip_left: usize,
) {
    use std::fmt::Write as FmtWrite;

    for _ in 0..(digit_count - count_digits(line_nbr)) {
        out_str.push(' ');
    }

    let print_str: &str = if curr_line.len() - skip_left > ERROR_OUT_MAX_WIDTH {
        let end = skip_left + min(ERROR_OUT_MAX_WIDTH, curr_line.len());
        &curr_line[skip_left..end]
    } else {
        &curr_line
    };

    write!(out_str, "{}| {}\n", line_nbr, print_str).unwrap();
}

