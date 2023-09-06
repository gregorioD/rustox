use crate::{error::Error, scanner::Scanner, token::Token};
use std::{error::Error as StdError, fs::read, io, path::Path, process, process::exit};

pub struct Lox {
    error: Option<Error>,
}

impl Lox {
    pub fn start(&mut self, args: Vec<String>) {
        if args.len() > 2 {
            eprintln!("Usage: rustox [script]");
            process::exit(1);
        } else if args.len() == 2 {
            self.run_file(args[1].clone());
        } else {
            println!("To terminate REPL press enter.");
            self.run_prompt();
        }
    }

    pub fn run_file(&mut self, path: String) -> Result<(), Box<dyn StdError + 'static>> {
        let file_path = Path::new(&path[..]);
        let bytes: Vec<u8> = read(file_path)?;
        let source = String::from_utf8(bytes)?;

        match Self::run(source) {
            Ok(tokens) => {
                for token in tokens {
                    println!("{}", token);
                }
            }
            Err(e) => {
                self.error(e.line(), e.message());
                exit(1);
            }
        }

        Ok(())
    }

    pub fn run_prompt(&mut self) -> io::Result<()> {
        let stdin = io::stdin();
        loop {
            let mut buffer = String::new();
            match stdin.read_line(&mut buffer) {
                Ok(2) => break,
                Ok(n) => {
                    match Self::run(buffer) {
                        Ok(tokens) => {
                            for token in tokens {
                                println!("{}", token);
                            }
                        }
                        Err(e) => self.error(e.line(), e.message()),
                    }
                    self.error = None
                }
                Err(e) => eprintln!("ERROR: {}", e),
            }
        }

        Ok(())
    }

    fn run(source: String) -> Result<Vec<Token>, Error> {
        let mut scanner = Scanner::new(source);

        match scanner.scan_tokens() {
            Ok(tokens) => Ok(tokens),
            Err(e) => Err(e),
        }
    }

    fn error(&mut self, line: usize, message: String) {
        self.report(line, "".to_string(), message);
    }

    fn report(&mut self, line: usize, where_happened: String, message: String) {
        eprintln!("[{line}] Error {where_happened}: {message}");
        self.error = Some(Error::new(line, message));
    }
}
