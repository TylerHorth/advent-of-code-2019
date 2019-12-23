use crate::intcode::parser::{read_program, ParseError};
use crossterm::style::Print;
use crossterm::ExecutableCommand;
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Error, Formatter};
use std::io::{stderr, stdin};
use std::sync::mpsc::{Receiver, Sender};

pub struct Computer {
    memory: Vec<i64>,
    pc: usize,
    rb: usize,
    input: Option<Receiver<i64>>,
    output: Option<Sender<i64>>,
}

#[derive(Eq, PartialEq)]
pub enum RuntimeError {
    OutOfBounds(i64),
    UnrecognizedOpcode([u8; 4]),
    UnrecognizedParameterMode(u8),
    InputError,
    OutputError,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            RuntimeError::OutOfBounds(address) => {
                write!(f, "Attempted to access out of bounds memory {}", address)
            }
            RuntimeError::UnrecognizedOpcode(opcode) => write!(
                f,
                "Unrecognized opcode {}, with parameter modes {}, {}, {}",
                opcode[0], opcode[1], opcode[2], opcode[3]
            ),
            RuntimeError::UnrecognizedParameterMode(mode) => {
                write!(f, "Unrecognized parameter mode {}", mode)
            }
            RuntimeError::InputError => write!(f, "Error reading input"),
            RuntimeError::OutputError => write!(f, "Error writing output"),
        }
    }
}

impl Debug for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        Display::fmt(self, f)
    }
}

impl Computer {
    pub fn load(program: &str) -> Result<Computer, ParseError> {
        let memory = read_program(program)?;
        let pc = 0;
        let rb = 0;
        let computer = Computer {
            memory,
            pc,
            rb,
            input: None,
            output: None,
        };

        Ok(computer)
    }

    pub fn set_input(&mut self, receiver: Receiver<i64>) {
        self.input = Some(receiver);
    }

    pub fn set_output(&mut self, sender: Sender<i64>) {
        self.output = Some(sender);
    }

    pub fn clone_output(&self) -> Option<Sender<i64>> {
        self.output.clone()
    }

    pub fn take_input(&mut self) -> Option<Receiver<i64>> {
        self.input.take()
    }

    fn to_address(&self, address: i64) -> Result<usize, RuntimeError> {
        usize::try_from(address).map_err(|_| RuntimeError::OutOfBounds(address))
    }

    fn extend(&mut self, address: usize) {
        if self.memory.len() <= address {
            self.memory.resize(address + 1, 0);
        }
    }

    pub fn set(&mut self, address: i64, value: i64) -> Result<(), RuntimeError> {
        let address = self.to_address(address)?;
        self.extend(address);
        self.memory[address] = value;
        Ok(())
    }

    pub fn get(&mut self, address: i64) -> Result<i64, RuntimeError> {
        let address = self.to_address(address)?;
        self.extend(address);
        Ok(self.memory[address])
    }

    fn opcode(&mut self) -> Result<[u8; 4], RuntimeError> {
        let mut int = self.read_im()?;

        let opcode = int % 100;
        int /= 100;

        let mut result = [opcode as u8, 0, 0, 0];
        for i in 1..=3 {
            if int == 0 {
                break;
            }

            result[i] = (int % 10) as u8;
            int /= 10;
        }

        Ok(result)
    }

    fn read_pos(&mut self) -> Result<i64, RuntimeError> {
        let address = self.read_im()?;
        self.get(address)
    }

    fn read_im(&mut self) -> Result<i64, RuntimeError> {
        self.extend(self.pc);
        let int = self.memory[self.pc];

        self.pc += 1;

        Ok(int)
    }

    fn read_rel(&mut self) -> Result<i64, RuntimeError> {
        let offset = self.read_im()?;
        self.get(self.rb as i64 + offset)
    }

    fn read(&mut self, mode: u8) -> Result<i64, RuntimeError> {
        match mode {
            0 => self.read_pos(),
            1 => self.read_im(),
            2 => self.read_rel(),
            m => Err(RuntimeError::UnrecognizedParameterMode(m)),
        }
    }

    fn write_pos(&mut self, value: i64) -> Result<(), RuntimeError> {
        let address = self.read_im()?;
        self.set(address, value)
    }

    fn write_rel(&mut self, value: i64) -> Result<(), RuntimeError> {
        let offset = self.read_im()?;
        self.set(self.rb as i64 + offset, value)
    }

    fn write(&mut self, mode: u8, value: i64) -> Result<(), RuntimeError> {
        match mode {
            0 => self.write_pos(value),
            2 => self.write_rel(value),
            m => Err(RuntimeError::UnrecognizedParameterMode(m)),
        }
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        loop {
            match self.opcode()? {
                // Add
                [1, a, b, r] => {
                    let first = self.read(a)?;
                    let second = self.read(b)?;

                    self.write(r, first + second)?;
                }
                // Multiply
                [2, a, b, r] => {
                    let first = self.read(a)?;
                    let second = self.read(b)?;

                    self.write(r, first * second)?;
                }
                // Input
                [3, r, 0, 0] => {
                    let input = if let Some(receiver) = &self.input {
                        receiver.recv().map_err(|_| RuntimeError::InputError)?
                    } else {
                        stderr()
                            .execute(Print("> "))
                            .map_err(|_| RuntimeError::InputError)?;

                        let mut input = String::new();
                        stdin()
                            .read_line(&mut input)
                            .map_err(|_| RuntimeError::InputError)?;

                        input.trim().parse().map_err(|_| RuntimeError::InputError)?
                    };

                    self.write(r, input)?;
                }
                // Output
                [4, a, 0, 0] => {
                    let value = self.read(a)?;
                    if let Some(sender) = &self.output {
                        sender.send(value).map_err(|_| RuntimeError::OutputError)?;
                    } else {
                        println!("{}", value);
                    }
                }
                // Jump-if-true
                [5, a, b, 0] => {
                    let condition = self.read(a)?;
                    let address = self.read(b)?;

                    if condition != 0 {
                        self.pc = self.to_address(address)?;
                    }
                }
                // Jump-if-false
                [6, a, b, 0] => {
                    let condition = self.read(a)?;
                    let address = self.read(b)?;

                    if condition == 0 {
                        self.pc = self.to_address(address)?;
                    }
                }
                // Less than
                [7, a, b, r] => {
                    let first = self.read(a)?;
                    let second = self.read(b)?;

                    self.write(r, if first < second { 1 } else { 0 })?;
                }
                // equals
                [8, a, b, r] => {
                    let first = self.read(a)?;
                    let second = self.read(b)?;

                    self.write(r, if first == second { 1 } else { 0 })?;
                }
                // Relative base offset
                [9, a, 0, 0] => {
                    let offset = self.read(a)?;
                    self.rb = self.to_address(self.rb as i64 + offset)?;
                }
                // Exit
                [99, 0, 0, 0] => return Ok(()),
                opcode => {
                    return Err(RuntimeError::UnrecognizedOpcode(opcode));
                }
            }
        }
    }
}
