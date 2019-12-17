use std::fmt::{Display, Formatter, Error, Debug};
use std::convert::TryFrom;
use crate::intcode::parser::{ParseError, read_program};
use dialoguer::theme::CustomPromptCharacterTheme;
use dialoguer::Input;
use std::sync::mpsc::{Receiver, Sender};

pub struct Computer {
    memory: Vec<i64>,
    pc: usize,
    theme: CustomPromptCharacterTheme,
    input: Option<Receiver<i64>>,
    output: Option<Sender<i64>>
}

pub enum RuntimeError {
    OutOfBounds(i64),
    UnrecognizedOpcode([u8; 4]),
    UnrecognizedParameterMode(u8),
    InputError,
    OutputError
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            RuntimeError::OutOfBounds(address) =>
                write!(f, "Attempted to access out of bounds memory {}", address),
            RuntimeError::UnrecognizedOpcode(opcode) =>
                write!(f, "Unrecognized opcode {}, with parameter modes {}, {}, {}",
                       opcode[0], opcode[1], opcode[2], opcode[3]),
            RuntimeError::UnrecognizedParameterMode(mode) =>
                write!(f, "Unrecognized parameter mode {}", mode),
            RuntimeError::InputError =>
                write!(f, "Error reading input"),
            RuntimeError::OutputError =>
                write!(f, "Error writing output"),
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
        let theme = CustomPromptCharacterTheme::new('>');
        let computer = Computer {
            memory,
            pc,
            theme,
            input: None,
            output: None
        };

        Ok(computer)
    }

    pub fn set_input(&mut self, receiver: Receiver<i64>) {
        self.input = Some(receiver);
    }

    pub fn set_output(&mut self, sender: Sender<i64>) {
        self.output = Some(sender);
    }

    fn to_address(&self, address: i64) -> Result<usize, RuntimeError> {
        let address = usize::try_from(address)
            .map_err(|_| RuntimeError::OutOfBounds(address))?;

        if address >= self.memory.len() {
            Err(RuntimeError::OutOfBounds(address as i64))
        } else {
            Ok(address)
        }
    }

    pub fn set(&mut self, address: i64, value: i64) -> Result<(), RuntimeError> {
        let address = self.to_address(address)?;
        self.memory[address] = value;
        Ok(())
    }

    pub fn get(&self, address: i64) -> Result<i64, RuntimeError> {
        let address = self.to_address(address)?;
        Ok(self.memory[address])
    }

    fn opcode(&mut self) -> Result<[u8; 4], RuntimeError> {
        let mut int = self.read_im()?;

        let opcode = int % 100;
        int /= 100;

        let mut result = [opcode as u8, 0, 0, 0];
        for i in 1..=3 {
            if int == 0 {
                break
            }

            result[i] = (int % 10) as u8;
            int /= 10;
        }

        Ok(result)
    }

    fn read_pos(&mut self) -> Result<i64, RuntimeError> {
        let int = self.read_im()?;
        self.get(int)
    }

    fn read_im(&mut self) -> Result<i64, RuntimeError> {
        let int = self.memory
            .get(self.pc)
            .ok_or(RuntimeError::OutOfBounds(self.pc as i64))?
            .clone();

        self.pc += 1;

        Ok(int)
    }

    fn read(&mut self, mode: u8) -> Result<i64, RuntimeError> {
        match mode {
            0 => self.read_pos(),
            1 => self.read_im(),
            m => Err(RuntimeError::UnrecognizedParameterMode(m))
        }
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        while self.pc < self.memory.len() {
            match self.opcode()? {
                // Add
                [1, a, b, 0] => {
                    let first = self.read(a)?;
                    let second = self.read(b)?;
                    let result = self.read_im()?;

                    self.set(result, first + second)?;
                },
                // Multiply
                [2, a, b, 0] => {
                    let first = self.read(a)?;
                    let second = self.read(b)?;
                    let result = self.read_im()?;

                    self.set(result, first * second)?;
                },
                // Input
                [3, 0, 0, 0] => {
                    let result = self.read_im()?;
                    let input = if let Some(receiver) = &self.input {
                        receiver.recv()
                            .map_err(|_| RuntimeError::InputError)?
                    } else {
                        Input::<i64>::with_theme(&self.theme)
                            .interact()
                            .map_err(|_| RuntimeError::InputError)?
                    };

                    self.set(result, input)?;
                }
                // Output
                [4, a, 0, 0] => {
                    let value = self.read(a)?;
                    if let Some(sender) = &self.output {
                        sender.send(value)
                            .map_err(|_| RuntimeError::OutputError)?;
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
                [7, a, b, 0] => {
                    let first = self.read(a)?;
                    let second = self.read(b)?;
                    let result = self.read_im()?;

                    self.set(result, if first < second { 1 } else { 0 })?;
                }
                // equals
                [8, a, b, 0] => {
                    let first = self.read(a)?;
                    let second = self.read(b)?;
                    let result = self.read_im()?;

                    self.set(result, if first == second { 1 } else { 0 })?;
                }
                // Exit
                [99, 0, 0, 0] => {
                    self.pc = self.memory.len();
                },
                opcode => {
                    return Err(RuntimeError::UnrecognizedOpcode(opcode));
                }
            }
        }

        Ok(())
    }
}

