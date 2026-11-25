mod command;

use std::io::{BufRead, Write};

use thiserror::Error;

use crate::{
    ledger::Ledger,
    repl::command::{Command, error::InvalidCommand},
};

fn indent(n: usize) {
    print!("{}", " ".repeat(n));
}

#[derive(Debug, Error)]
pub enum ReplError {
    #[error(transparent)]
    Command(#[from] InvalidCommand),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug)]
pub struct Repl<R> {
    ledger: Ledger,
    reader: R,
}

fn print_hint() {
    let commands = [
        Command::Help,
        Command::Post {
            account: "".into(),
            amount: 0,
        },
        Command::Balance { account: "".into() },
        Command::Clear,
        Command::Exit,
    ];
    let longest = commands
        .iter()
        .map(|c| c.help_example().len())
        .max()
        .unwrap_or(0);
    println!("commands:");
    for command in commands.iter() {
        let example = command.help_example();
        let space = longest.abs_diff(example.len()) + 5;
        indent(2);
        print!("{}", command.help_example());
        indent(space);
        println!("{}", command.help_message());
    }
}

impl<R: BufRead> Repl<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            ledger: Ledger::new(),
        }
    }

    fn get_command(&mut self) -> Result<Command, ReplError> {
        let mut buf = String::new();
        if self.reader.read_line(&mut buf)? == 0 {
            return Ok(Command::Exit);
        }
        let command: Command = buf.parse()?;
        Ok(command)
    }

    pub fn run(&mut self) -> Result<(), ReplError> {
        print_hint();
        loop {
            match self.get_command() {
                Ok(Command::Help) => print_hint(),
                Ok(Command::Post { account, amount }) => {
                    self.ledger.post(account.clone(), amount);
                    println!("posted {amount} dollars to {account}");
                }
                Ok(Command::Balance { account }) => {
                    println!("{account} has {} dollars", self.ledger.balance(&account))
                }
                Ok(Command::Clear) => {
                    print!("\x1B[2J\x1B[1;1H");
                    std::io::stdout().flush()?;
                }
                Ok(Command::Exit) => return Ok(()),
                Err(e) => println!("{e}"),
            }
        }
    }
}
