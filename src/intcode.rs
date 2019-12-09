use std::collections::{HashMap, VecDeque};

struct Memory {
    mem: HashMap<usize, i64>
}

impl Memory {
    fn get(&mut self, addr: usize) -> i64 {
        *self.mem.entry(addr).or_insert(0)
    }
    
    fn set(&mut self, addr: usize, value: i64) {
        self.mem.insert(addr, value);
    }
}

impl From<&[i64]> for Memory {
    fn from(t: &[i64]) -> Self {
        Memory { mem: t.iter().copied().enumerate().collect() }
    }
}

#[derive(Debug)]
pub enum Event {
    Output(i64),
    Halt
}

pub struct Intcode {
    mem: Memory,
    pc: usize,
    rel_base: usize,
    input: VecDeque<i64>
}

impl Intcode {
    pub fn new(mem: &[i64]) -> Self {
        Intcode { mem: Memory::from(mem), pc: 0, rel_base: 0, input: VecDeque::new() }
    }
    
    pub fn with_input(mem: &[i64], input: &[i64]) -> Self {
        Intcode { mem: Memory::from(mem), pc: 0, rel_base: 0, input: VecDeque::from(input.to_vec()) }
    }
    
    pub fn input(&mut self, input: i64) {
        self.input.push_back(input);
    }
    
    pub fn get_mem(&mut self, n: usize) -> i64 {
        self.mem.get(n)
    }
    
    fn get_mode(&mut self, n: usize) -> i64 {
        self.mem.get(self.pc) / 10_i64.pow(n as u32 + 1) % 10
    }
    
    fn get_arg_addr(&mut self, n: usize) -> usize {
        match self.get_mode(n) {
            0 => self.mem.get(self.pc + n) as usize,
            1 => self.pc + n,
            2 => self.mem.get(self.pc + n) as usize + self.rel_base,
            m => panic!("invalid mode {}", m)
        }
    }
    
    fn get_arg(&mut self, n: usize) -> i64 {
        let addr = self.get_arg_addr(n);
        self.mem.get(addr)
    }
    
    pub fn run(&mut self) -> Event {
        loop {
            let opcode = self.mem.get(self.pc) % 100;
            
            match opcode {
                1 | 2 | 7 | 8 => {
                    let (x, y, z) = (
                        self.get_arg(1),
                        self.get_arg(2),
                        self.get_arg_addr(3)
                    );
                    
                    self.pc += 4;
                    
                    self.mem.set(z, match opcode {
                        1 => x + y,
                        2 => x * y,
                        7 => if x < y { 1 } else { 0 },
                        8 => if x == y { 1 } else { 0 },
                        _ => unreachable!()
                    });
                },
                3 | 4 | 9 => {
                    let x = self.get_arg_addr(1);
                    
                    self.pc += 2;
                    
                    match opcode {
                        3 => if let Some(input) = self.input.pop_front() {
                                self.mem.set(x, input);
                            } else {
                                panic!("unexpected end of input");
                            }
                        4 => break Event::Output(self.mem.get(x)),
                        9 => self.rel_base += self.mem.get(x) as usize,
                        _ => unreachable!()
                    };
                },
                5 | 6 => {
                    let (x, y) = (
                        self.get_arg(1),
                        self.get_arg(2) as usize
                    );
                    
                    if (opcode == 5 && x != 0) || (opcode == 6 && x == 0) {
                        self.pc = y;
                    } else {
                        self.pc += 3;
                    }
                },
                99 => break Event::Halt,
                _ => panic!("invalid opcode {}", opcode),
            }
        }
    }
    
    pub fn execute<F: FnMut(i64)>(&mut self, mut output_fn: F) {
        loop {
            match self.run() {
                Event::Output(o) => output_fn(o),
                Event::Halt => break
            }
        }
    }
}
