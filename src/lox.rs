use crate::{scanner::Scanner, token::Token};
use std::{fs::read, io, path::Path, process::exit};

pub struct Lox {
    has_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { has_error: false }
    }

    pub fn start(&mut self, args: Vec<String>) {
        if args.len() > 2 {
            eprintln!("Usage: rustox [script]");
            exit(1);
        } else if args.len() == 2 {
            self.run_file(args[1].clone());
        } else {
            println!("To terminate REPL press enter.");
            self.run_prompt();
        }
    }

    pub fn run_file(&mut self, path: String) {
        let file_path = Path::new(&path[..]);
        let bytes: Vec<u8> = read(file_path).unwrap();
        let source = String::from_utf8(bytes).unwrap();
        self.run(source);
    }

    pub fn run_prompt(&mut self) -> io::Result<()> {
        let stdin = io::stdin();
        loop {
            let mut buffer = String::new();
            match stdin.read_line(&mut buffer) {
                Ok(2) => break,
                Ok(n) => {
                    self.run(buffer);
                    self.has_error = false;
                }
                Err(e) => eprintln!("ERROR: {}", e),
            }
        }

        Ok(())
    }

    fn run(&mut self, source: String) -> Vec<Token> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        if let Some(_error) = scanner.error {
            self.has_error = true;
        };
        tokens
    }

    /*
    fn error(&mut self, line: usize, message: String) {
        self.report(line, "".to_string(), message);
    }

    fn report(&mut self, line: usize, where_happened: String, message: String) {
        eprintln!("[{line}] Error {where_happened}: {message}");
        self.has_error = Some(Error::new(line, message));
    }
    */
}
