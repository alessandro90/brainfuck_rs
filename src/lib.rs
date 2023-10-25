use std::io;

const TAPE_SIZE: usize = 30_000;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    MissingClosingBraket,
    MissingOpeningBraket,
    StdinReadFail,
    EOT, // End Of Tape
    CellOverflow,
    CellUnderflow,
}

struct Cells {
    pos: usize,
    tape: [u8; TAPE_SIZE],
}

impl Default for Cells {
    fn default() -> Self {
        Cells {
            pos: 0,
            tape: [0; TAPE_SIZE],
        }
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
    fn move_right(&mut self) -> Result<(), Error> {
        if self.pos < TAPE_SIZE - 1 {
            self.pos += 1;
            Ok(())
        } else {
            Err(Error::EOT)
        }
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

struct Lexer {
    pos: usize,
    data: Vec<Token>,
    nesting_lvl: usize,
}

impl Lexer {
    fn new(data: String) -> Self {
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
            nesting_lvl: 0,
        }
    }

    fn get_current(&self) -> Token {
        self.data[self.pos]
    }

    fn advance(&mut self) -> Option<()> {
        // end plus 1 us allowed
        if self.pos < self.data.len() - 1 {
            self.pos += 1;
            Some(())
        } else {
            None
        }
    }

    fn exit_loop(&mut self) -> Result<(), Error> {
        let target_nesting = self.nesting_lvl;
        loop {
            match self.get_current() {
                Token::RSquare => {
                    if self.nesting_lvl == 0 {
                        return Err(Error::MissingOpeningBraket);
                    }
                    self.nesting_lvl -= 1;
                    if self.nesting_lvl == target_nesting {
                        self.advance().ok_or(Error::MissingClosingBraket)?;
                        return Ok(());
                    }
                }
                Token::LSquare => {
                    self.nesting_lvl += 1;
                }
                _ => (),
            }
            self.advance().ok_or(Error::MissingClosingBraket)?;
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

    fn iterate(&mut self) -> Result<(), Error> {
        let target_nesting = self.nesting_lvl;
        loop {
            match self.get_current() {
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
            self.go_back()?
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

pub fn interpret(src: String) -> Result<(), Error> {
    let mut cells = Cells::default();
    let mut lexer = Lexer::new(src);
    loop {
        match lexer.get_current() {
            Token::LSquare => {
                if cells.get() == 0 {
                    lexer.exit_loop()?;
                    continue;
                } else {
                    lexer.nesting_lvl += 1;
                }
            }
            Token::RSquare => {
                if cells.get() != 0 {
                    lexer.iterate()?;
                    continue;
                } else {
                    lexer.nesting_lvl -= 1;
                }
            }
            Token::Dot => print!("{}", cells.get() as char),
            Token::Minus => cells.decr(),
            Token::Plus => cells.incr(),
            Token::Lt => cells.move_left()?,
            Token::Gt => cells.move_right()?,
            Token::Comma => cells.set(read_u8_from_stdin()?),
        }
        if lexer.advance().is_none() {
            return Ok(());
        }
    }
}
