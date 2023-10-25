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
    fn incr(&mut self) -> Result<(), Error> {
        let cell = &mut self.tape[self.pos];
        if *cell < u8::MAX {
            *cell += 1;
            Ok(())
        } else {
            Err(Error::CellOverflow)
        }
    }
    fn decr(&mut self) -> Result<(), Error> {
        let cell = &mut self.tape[self.pos];
        if *cell > 0 {
            *cell -= 1;
            Ok(())
        } else {
            Err(Error::CellUnderflow)
        }
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
    nest_l_square: usize,
    nest_r_square: usize,
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
            nest_l_square: 0,
            nest_r_square: 0,
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
        loop {
            match self.get_current() {
                Token::RSquare => {
                    if self.nest_l_square > 0 {
                        self.nest_l_square -= 1;
                    }
                    if self.nest_l_square == 0 {
                        self.pos += 1;
                        break;
                    }
                }
                _ => self.advance().ok_or(Error::MissingClosingBraket)?,
            }
        }
        Ok(())
    }

    fn iterate(&mut self) -> Result<(), Error> {
        loop {
            match self.get_current() {
                Token::LSquare => {
                    if self.nest_r_square > 0 {
                        self.nest_r_square -= 1;
                    }
                    if self.nest_r_square == 0 {
                        break;
                    }
                }
                _ => {
                    if self.pos > 0 {
                        self.pos -= 1;
                    } else {
                        return Err(Error::MissingOpeningBraket);
                    }
                }
            }
        }
        Ok(())
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
                if cells.get() != 0 {
                    lexer.nest_l_square += 1;
                } else {
                    lexer.exit_loop()?;
                    continue;
                }
            }
            Token::RSquare => {
                if cells.get() == 0 {
                    lexer.nest_r_square += 1;
                } else {
                    lexer.iterate()?;
                    continue;
                }
            }
            Token::Dot => print!("{}", cells.get() as char),
            Token::Minus => cells.decr()?,
            Token::Plus => cells.incr()?,
            Token::Lt => cells.move_left()?,
            Token::Gt => cells.move_right()?,
            Token::Comma => cells.set(read_u8_from_stdin()?),
        }
        if lexer.advance().is_none() {
            return Ok(());
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Lexer;

    #[test]
    fn source_jump_to_next_test() {
        let mut src = Lexer::new("--[[..[,]..]...]".into());
        src.nest_l_square = 1;
        src.pos = 3;
        src.exit_loop().unwrap();
        assert_eq!(src.pos, 9);
    }
    #[test]
    fn source_jump_to_prev_test() {
        let mut src = Lexer::new("--[[..[,]..]...]".into());
        src.nest_r_square = 1;
        src.pos = 11;
        src.iterate().unwrap();
        assert_eq!(src.pos, 6);
    }
}
