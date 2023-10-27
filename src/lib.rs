use std::io;

const INITIAL_TAPE_SIZE: usize = 30_000;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    MissingClosingBraket,
    MissingOpeningBraket,
    StdinReadFail,
    EOT, // End Of Tape
}

struct Cells {
    pos: usize,
    tape: Vec<u8>,
}

impl Default for Cells {
    fn default() -> Self {
        let tape = vec![0u8; INITIAL_TAPE_SIZE];
        Cells { pos: 0, tape }
    }
}

impl Cells {
    fn incr(&mut self) {
        self.tape[self.pos] = self.tape[self.pos].wrapping_add(1);
    }
    fn decr(&mut self) {
        self.tape[self.pos] = self.tape[self.pos].wrapping_sub(1);
    }
    fn get(&self) -> u8 {
        self.tape[self.pos]
    }
    fn move_right(&mut self) {
        if self.pos >= self.tape.len() {
            self.tape.push(0);
        }
        self.pos += 1;
    }
    fn move_left(&mut self) -> Result<(), Error> {
        if self.pos > 0 {
            self.pos -= 1;
            Ok(())
        } else {
            Err(Error::EOT)
        }
    }
    fn set(&mut self, v: u8) {
        self.tape[self.pos] = v;
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum Token {
    RSquare,
    LSquare,
    Dot,
    Minus,
    Plus,
    Lt,
    Gt,
    Comma,
}

pub struct Lexer {
    pos: usize,
    data: Vec<Token>,
}

impl Lexer {
    pub fn new(data: String) -> Self {
        Lexer {
            pos: 0,
            data: data
                .chars()
                .filter_map(|c| match c {
                    '[' => Some(Token::LSquare),
                    ']' => Some(Token::RSquare),
                    '.' => Some(Token::Dot),
                    '-' => Some(Token::Minus),
                    '+' => Some(Token::Plus),
                    '<' => Some(Token::Lt),
                    '>' => Some(Token::Gt),
                    ',' => Some(Token::Comma),
                    _ => None,
                })
                .collect(),
        }
    }

    fn get_current(&self) -> Token {
        self.data[self.pos]
    }

    fn advance(&mut self) -> Option<()> {
        if self.pos < self.data.len() - 1 {
            self.pos += 1;
            Some(())
        } else {
            None
        }
    }

    fn go_back(&mut self) -> Result<(), Error> {
        if self.pos > 0 {
            self.pos -= 1;
            Ok(())
        } else {
            Err(Error::MissingOpeningBraket)
        }
    }
}

fn read_u8_from_stdin() -> Result<u8, Error> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| Error::StdinReadFail)?;
    input.trim().parse().map_err(|_| Error::StdinReadFail)
}

pub struct Interpreter {
    nesting_lvl: usize,
    lexer: Lexer,
}

impl Interpreter {
    pub fn new(lexer: Lexer) -> Self {
        Interpreter {
            nesting_lvl: 0,
            lexer,
        }
    }

    fn exit_loop(&mut self) -> Result<(), Error> {
        let target_nesting = self.nesting_lvl;
        loop {
            match self.lexer.get_current() {
                Token::RSquare => {
                    if self.nesting_lvl == 0 {
                        return Err(Error::MissingOpeningBraket);
                    }
                    self.nesting_lvl -= 1;
                    if self.nesting_lvl == target_nesting {
                        self.lexer.advance().ok_or(Error::MissingClosingBraket)?;
                        return Ok(());
                    }
                }
                Token::LSquare => {
                    self.nesting_lvl += 1;
                }
                _ => (),
            }
            self.lexer.advance().ok_or(Error::MissingClosingBraket)?;
        }
    }

    fn iterate(&mut self) -> Result<(), Error> {
        let target_nesting = self.nesting_lvl;
        loop {
            match self.lexer.get_current() {
                Token::LSquare => {
                    if self.nesting_lvl == 0 {
                        return Err(Error::MissingClosingBraket);
                    }
                    self.nesting_lvl -= 1;
                    if self.nesting_lvl == target_nesting {
                        return Ok(());
                    }
                }
                Token::RSquare => self.nesting_lvl += 1,
                _ => (),
            }
            self.lexer.go_back()?
        }
    }

    pub fn interpret(&mut self) -> Result<(), Error> {
        let mut cells = Cells::default();
        loop {
            match self.lexer.get_current() {
                Token::LSquare => {
                    if cells.get() == 0 {
                        self.exit_loop()?;
                        continue;
                    } else {
                        self.nesting_lvl += 1;
                    }
                }
                Token::RSquare => {
                    if cells.get() != 0 {
                        self.iterate()?;
                        continue;
                    } else {
                        self.nesting_lvl -= 1;
                    }
                }
                Token::Dot => print!("{}", cells.get() as char),
                Token::Minus => cells.decr(),
                Token::Plus => cells.incr(),
                Token::Lt => cells.move_left()?,
                Token::Gt => cells.move_right(),
                Token::Comma => cells.set(read_u8_from_stdin()?),
            }
            if self.lexer.advance().is_none() {
                return Ok(());
            }
        }
    }
}
