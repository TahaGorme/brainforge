// The eight language commands each consist of a single character:

// Character	Instruction Performed
// >	Increment the data pointer by one (to point to the next cell to the right).
// <	Decrement the data pointer by one (to point to the next cell to the left). Undefined if at 0.
// +	Increment the byte at the data pointer by one modulo 256.
// -	Decrement the byte at the data pointer by one modulo 256.
// .	Output the byte at the data pointer.
// ,	Accept one byte of input, storing its value in the byte at the data pointer.[b]
// [	If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
// ]	If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.[c]

use anyhow::{Context, Ok, Result};
use std::fs::File;
use std::io::Read;
use std::process;
use std::{env, io::BufReader};
struct Lexer<R: Read> {
    reader: R,
    location: SourceLocation,
    peeked_token: Option<Token>,
}
#[derive(Debug, Copy, Clone)]
struct SourceLocation {
    line: usize,
    column: usize,
}

impl Default for SourceLocation {
    fn default() -> Self {
        Self { line: 1, column: 1 }
    }
}

#[derive(Debug, Clone, Copy)]
struct Token {
    char: char,
    location: SourceLocation,
}
enum Operation {
    MoveRight,
    MoveLeft,
    IncrementCell,
    DecrementCell,
    WriteCell,
    ReadCell,
    JumpIfZero,
    JumpIfNotZero,
}
impl<R> Lexer<R>
where
    R: Read,
{
    fn new(source: R) -> Self {
        Self {
            reader: source,
            location: SourceLocation::default(),
            peeked_token: None,
        }
    }

    fn is_instruction_char(candidate: char) -> bool {
        let instruction_chars = "<>+-.,[]";
        for ch in instruction_chars.chars() {
            if ch == candidate {
                return true;
            }
        }
        return false;
    }

    fn next_token(&mut self) -> Result<Option<Token>> {
        if let Some(token) = self.peeked_token {
            self.peeked_token = None;
            return Ok(Some((token)));
        }
        let mut buf: [u8; 1] = [0; 1];
        let mut loc = self.location;
        while !Self::is_instruction_char(buf[0].into()) {
            loc = self.location;
            let read_bytes = self
                .reader
                .read(&mut buf)
                .context("failed to read next byte from source file")?;
            if read_bytes != 1 {
                return Ok(None);
            }
            self.location.column += 1;
            if buf[0] == '\n' as u8 {
                self.location.column = 1;
                self.location.line += 1;
            }
        }

        Ok(Some(Token {
            char: buf[0].into(),
            location: loc,
        }))
    }
    fn peek(&mut self) -> Result<Option<Token>> {
        if self.peeked_token.is_some() {
            let token = self
                .peeked_token
                .take()
                .expect("peek token to be available");

            return Ok(Some((token)));
        }
        self.peeked_token = self.next_token().context("failed to read next token")?;
        return Ok(self.peeked_token);
    }
}
fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let (command, args) = args.split_first().expect("command is expected");
    if args.is_empty() {
        eprintln!("Usage: {command} <path_to_file>");
        process::exit(1);
    }
    let file_path = args.first().expect("path to file is expected");
    println!("Opening the file: {file_path}");

    let reader = BufReader::new(File::open(file_path).context("failed to open file specified")?);
    let mut lexer = Lexer::new(reader);
    let peeked_token = lexer.peek()?;
    println!("peeked token {:?}", peeked_token);
    let next_token = lexer.next_token();
    println!("chopped token {:?}", next_token);

    while let Some(token) = lexer
        .next_token()
        .context("lexer failed while reading next token")?
    {
        println!("{:?}", token);
    }
    println!("");
    Ok(())
}
