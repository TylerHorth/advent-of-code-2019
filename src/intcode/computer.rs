use std::fmt::{Display, Formatter, Error, Debug};
use std::convert::TryFrom;
use crate::intcode::parser::{ParseError, read_program};

pub struct Computer {
    memory: Vec<i64>,
    pc: usize
}

pub enum RuntimeError {
    EndOfInstructions,
    OutOfBounds(i64),
    UnrecognizedOpcode(i64),
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            RuntimeError::EndOfInstructions=>
                write!(f, "End of instructions reached"),
            RuntimeError::OutOfBounds(address) =>
                write!(f, "Attempted to access out of bounds memory {}", address),
            RuntimeError::UnrecognizedOpcode(opcode) =>
                write!(f, "Unrecognized opcode {}", opcode)
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
        let computer = Computer {
            memory,
            pc: 0
        };

        Ok(computer)
    }

    pub fn set(&mut self, address: i64, value: i64) -> Result<(), RuntimeError> {
        let address = usize::try_from(address)
            .map_err(|_| RuntimeError::OutOfBounds(address))?;

        if address >= self.memory.len() {
            Err(RuntimeError::OutOfBounds(address as i64))
        } else {
            self.memory[address] = value;
            Ok(())
        }
    }

    pub fn get(&self, address: i64) -> Result<i64, RuntimeError> {
        let address = usize::try_from(address)
            .map_err(|_| RuntimeError::OutOfBounds(address))?;

        if address >= self.memory.len() {
            Err(RuntimeError::OutOfBounds(address as i64))
        } else {
            Ok(self.memory[address])
        }
    }

    fn read3(&mut self) -> Result<(i64, i64, i64), RuntimeError> {
        if self.pc + 3 >= self.memory.len() {
            return Err(RuntimeError::OutOfBounds(self.pc as i64 + 3))
        }

        Ok((
            self.memory[self.pc + 1],
            self.memory[self.pc + 2],
            self.memory[self.pc + 3]
        ))
    }

    fn step(&mut self) -> Result<bool, RuntimeError> {
        if self.pc >= self.memory.len() {
            return Err(RuntimeError::EndOfInstructions)
        }

        match self.memory[self.pc] {
            1 => { // Add
                let (first, second, result) = self.read3()?;

                self.set(
                    result,
                    self.get(first)? + self.get(second)?
                )?;
                self.pc += 4;

                Ok(true)
            },
            2 => { // Multiply
                let (first, second, result) = self.read3()?;

                self.set(
                    result,
                    self.get(first)? * self.get(second)?
                )?;
                self.pc += 4;

                Ok(true)
            },
            99 => { // Finish
                self.pc = self.memory.len();
                Ok(false)
            },
            opcode => Err(RuntimeError::UnrecognizedOpcode(opcode))
        }
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        while self.step()? {}
        Ok(())
    }
}

