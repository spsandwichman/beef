use std::{io::{self, Read, Write}, num::Wrapping};

pub struct Interpreter {
    pub src: Box<[u8]>,
    src_index: usize,

    mem: Box<[Wrapping<u8>]>,
    pointer: usize,

    loop_stack: Vec<usize>,
}

impl Interpreter {
    pub fn new(src: &[u8], mem_size: usize) -> Interpreter {
        Interpreter {
            src: src.into(),
            src_index: 0,

            mem: vec![Wrapping(0); mem_size].into(),
            pointer: 0,

            loop_stack: vec![],
        }
    }

    fn current(&self) -> u8 {
        self.mem[self.pointer].0
    }

    pub fn execute(&mut self) {
        while self.src_index < self.src.len() {
            // println!("{} at {}", self.src[self.src_index] as char, self.src_index);
            match self.src[self.src_index] {
                b'+' => self.mem[self.pointer] += 1,
                b'-' => self.mem[self.pointer] -= 1,
                b'>' => {
                    self.pointer = (self.pointer + 1) % self.mem.len();
                }
                b'<' => {
                    if self.pointer == 0 {
                        self.pointer = self.mem.len() - 1;
                    } else {
                        self.pointer -= 1;
                    }
                }
                b'.' => {
                    print!("{}", self.current() as char);
                    io::stdout().flush().unwrap();
                }
                b',' => {
                    let byte = std::io::stdin().bytes().next().unwrap().unwrap();
                    self.mem[self.pointer] = Wrapping(byte);
                }
                b'[' => {
                    if self.current() == 0 {
                        self.src_index += 1;

                        let mut level = 0;
                        while self.src_index < self.src.len() {
                            match self.src[self.src_index] {
                                b'[' => level += 1,
                                b']' => level -= 1,
                                _ => (),
                            }
                            self.src_index += 1;
                            if level == 0 {
                                break;
                            }
                        }
                    } else {
                        self.loop_stack.push(self.src_index);
                    }
                }
                b']' => {
                    if self.current() != 0 {
                        self.src_index = *self.loop_stack.last().unwrap();
                    } else {
                        self.loop_stack.pop();
                    }
                }
                _ => (),
            }
            self.src_index += 1;
        }
    }
}