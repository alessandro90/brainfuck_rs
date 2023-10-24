use std::io;

const BUFFER_SIZE: usize = 30_000;

struct Cells {
    pos: usize,
    data: [u8; BUFFER_SIZE],
}

impl Default for Cells {
    fn default() -> Self {
        Cells {
            pos: 0,
            data: [0; BUFFER_SIZE],
        }
    }
}

impl Cells {
    fn incr(&mut self) {
        self.data[self.pos] += 1
    }
    fn decr(&mut self) {
        self.data[self.pos] -= 1
    }
    fn get(&self) -> u8 {
        self.data[self.pos]
    }
    fn move_right(&mut self) {
        self.pos += 1
    }
    fn move_left(&mut self) {
        self.pos -= 1
    }
    fn set(&mut self, v: u8) {
        self.data[self.pos] = v;
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
        self.pos += 1
    }

    fn jump_after_next_square_bracket(&mut self) {
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
                _ => self.pos += 1,
            }
        }
    }

    fn jump_to_previous_square_bracket(&mut self) {
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
                _ => self.pos -= 1,
            }
        }
    }
}

fn read_u8_from_stdin() -> Option<u8> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse().ok()
}

pub fn interpret(src: String) -> Option<()> {
    let mut buffer = Cells::default();
    let mut source = Source::new(src);
    while let Some(c) = source.get_current() {
        match c {
            '[' => {
                if buffer.get() != 0 {
                    source.nest_l_square += 1;
                } else {
                    source.jump_after_next_square_bracket();
                    continue;
                }
            }
            ']' => {
                if buffer.get() == 0 {
                    source.nest_r_square += 1;
                } else {
                    source.jump_to_previous_square_bracket();
                    continue;
                }
            }
            '.' => {
                print!("{}", buffer.get() as char);
            }
            '-' => {
                buffer.decr();
            }
            '+' => {
                buffer.incr();
            }
            '<' => {
                buffer.move_left();
            }
            '>' => {
                buffer.move_right();
            }
            ',' => {
                if let Some(n) = read_u8_from_stdin() {
                    buffer.set(n);
                } else {
                    return None;
                }
            }
            _ => (), // Comment
        }
        source.advance();
    }
    Some(())
}

#[cfg(test)]
mod test {
    use crate::Source;

    #[test]
    fn source_jump_to_next_test() {
        let mut src = Source::new("--[[..[,]..]...]".into());
        src.nest_l_square = 1;
        src.pos = 3;
        src.jump_after_next_square_bracket();
        assert_eq!(src.pos, 9);
    }
    #[test]
    fn source_jump_to_prev_test() {
        let mut src = Source::new("--[[..[,]..]...]".into());
        src.nest_r_square = 1;
        src.pos = 11;
        src.jump_to_previous_square_bracket();
        assert_eq!(src.pos, 6);
    }
}
