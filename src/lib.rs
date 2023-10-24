use std::io;

const TAPE_SIZE: usize = 30_000;

#[derive(Debug)]
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

struct Source {
    pos: usize,
    data: String,
    nest_l_square: usize,
    nest_r_square: usize,
}

impl Source {
    fn new(data: String) -> Self {
        Source {
            pos: 0,
            data,
            nest_l_square: 0,
            nest_r_square: 0,
        }
    }

    fn get_current(&self) -> Option<char> {
        self.data.chars().nth(self.pos)
    }

    fn advance(&mut self) {
        // end plus 1 us allowed
        if self.pos < self.data.len() {
            self.pos += 1;
        }
    }

    fn exit_loop(&mut self) -> Result<(), Error> {
        while let Some(c) = self.get_current() {
            match c {
                ']' => {
                    if self.nest_l_square > 0 {
                        self.nest_l_square -= 1;
                    }
                    if self.nest_l_square == 0 {
                        self.pos += 1;
                        break;
                    }
                }
                _ => {
                    if self.pos < TAPE_SIZE - 1 {
                        self.pos += 1;
                    } else {
                        return Err(Error::MissingClosingBraket);
                    }
                }
            }
        }
        Ok(())
    }

    fn iterate(&mut self) -> Result<(), Error> {
        while let Some(c) = self.get_current() {
            match c {
                '[' => {
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
    input.trim().parse::<u8>().map_err(|_| Error::StdinReadFail)
}

pub fn interpret(src: String) -> Result<(), Error> {
    let mut buffer = Cells::default();
    let mut source = Source::new(src);
    while let Some(c) = source.get_current() {
        match c {
            '[' => {
                if buffer.get() != 0 {
                    source.nest_l_square += 1;
                } else {
                    source.exit_loop()?;
                    continue;
                }
            }
            ']' => {
                if buffer.get() == 0 {
                    source.nest_r_square += 1;
                } else {
                    source.iterate()?;
                    continue;
                }
            }
            '.' => print!("{}", buffer.get() as char),
            '-' => buffer.decr()?,
            '+' => buffer.incr()?,
            '<' => buffer.move_left()?,
            '>' => buffer.move_right()?,
            ',' => buffer.set(read_u8_from_stdin()?),
            _ => (), // Comment
        }
        source.advance();
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::Source;

    #[test]
    fn source_jump_to_next_test() {
        let mut src = Source::new("--[[..[,]..]...]".into());
        src.nest_l_square = 1;
        src.pos = 3;
        src.exit_loop().unwrap();
        assert_eq!(src.pos, 9);
    }
    #[test]
    fn source_jump_to_prev_test() {
        let mut src = Source::new("--[[..[,]..]...]".into());
        src.nest_r_square = 1;
        src.pos = 11;
        src.iterate().unwrap();
        assert_eq!(src.pos, 6);
    }
}
