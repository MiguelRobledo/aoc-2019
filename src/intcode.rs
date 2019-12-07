use std::collections::VecDeque;

pub enum Event {
    Input,
    Output(i64),
    Halt
}

pub struct Intcode {
    mem: Vec<i64>,
    pub pc: usize,
    input: VecDeque<i64>
}

impl Intcode {
    pub fn new(mem: &[i64]) -> Self {
        Intcode { mem: mem.to_vec(), pc: 0, input: VecDeque::new() }
    }
    
    pub fn with_input(mem: &[i64], input: &[i64]) -> Self {
        Intcode { mem: mem.to_vec(), pc: 0, input: VecDeque::from(input.to_vec()) }
    }
    
    fn get_arg(&self, n: u32) -> i64 {
        if self.mem[self.pc] / 10_i64.pow(n + 1) % 10 == 1 {
            self.mem[self.pc + n as usize]
        } else {
            self.mem[self.mem[self.pc + n as usize] as usize]
        }
    }
    
    pub fn input(&mut self, input: i64) {
        self.input.push_back(input);
    }
    
    pub fn get_mem(&self, n: usize) -> i64 {
        self.mem[n]
    }
    
    pub fn run(&mut self) -> Event {
        loop {
            let opcode = self.mem[self.pc] % 100;
            
            match opcode {
                1 | 2 | 7 | 8 => {
                    let (x, y, z) = (
                        self.get_arg(1),
                        self.get_arg(2),
                        self.mem[self.pc + 3] as usize
                    );
                    
                    self.mem[z] = match opcode {
                        1 => x + y,
                        2 => x * y,
                        7 => if x < y { 1 } else { 0 },
                        8 => if x == y { 1 } else { 0 },
                        _ => unreachable!()
                    };
                    
                    self.pc += 4;
                },
                3 | 4 => {
                    let output = match opcode {
                        3 => if let Some(input) = self.input.pop_front() {
                                let x = self.mem[self.pc + 1] as usize;
                                self.mem[x] = input;
                                
                                None
                            } else {
                                Some(Event::Input)
                            },
                        4 => Some(Event::Output(self.get_arg(1))),
                        _ => unreachable!()
                    };
                    
                    self.pc += 2;
                    
                    if let Some(o) = output {
                        break o;
                    }
                },
                5 | 6 => {
                    let (x, y) = (
                        self.get_arg(1),
                        self.get_arg(2) as usize
                    );
                    
                    if (opcode == 5 && x != 0) || (opcode == 6 && x == 0) {
                        self.pc = y;
                        continue;
                    }
                    
                    self.pc += 3;
                },
                99 => break Event::Halt,
                _ => panic!("invalid opcode {}", opcode),
            }
        }
    }
}
