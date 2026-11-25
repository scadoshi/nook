mod command;

use std::io::{BufRead, Write};

use thiserror::Error;

use crate::ledger::Ledger;
use crate::repl::command::{Command, HELP, ParseCommandError};

#[derive(Debug, Error)]
pub enum ReplError {
    #[error(transparent)]
    Command(#[from] ParseCommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug)]
pub struct Repl<R, W> {
    reader: R,
    writer: W,
    ledger: Ledger,
}

impl<R: BufRead, W: Write> Repl<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader,
            writer,
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

    fn indent(&mut self, n: usize) -> Result<(), std::io::Error> {
        write!(self.writer, "{}", " ".repeat(n))?;
        Ok(())
    }

    fn print_hint(&mut self) -> Result<(), std::io::Error> {
        let longest = HELP
            .iter()
            .map(|entry| entry.example.len())
            .max()
            .unwrap_or(0);
        writeln!(self.writer, "commands:")?;
        for entry in HELP {
            let space = longest.abs_diff(entry.example.len()).saturating_add(5);
            self.indent(2)?;
            write!(self.writer, "{}", entry.example)?;
            self.indent(space)?;
            writeln!(self.writer, "{}", entry.message)?;
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), ReplError> {
        self.print_hint()?;
        loop {
            match self.get_command() {
                Ok(Command::Help) => self.print_hint()?,
                Ok(Command::Create { account_name }) => {
                    match self.ledger.create(account_name.clone()) {
                        Ok(()) => writeln!(self.writer, "{} created", &*account_name)?,
                        Err(e) => writeln!(self.writer, "create error: {e}")?,
                    }
                }
                Ok(Command::Post {
                    account_name,
                    amount,
                }) => {
                    let label = if amount == 1 { "dollar" } else { "dollars" };
                    match self.ledger.post(&account_name, amount) {
                        Ok(()) => {
                            writeln!(self.writer, "posted {amount} {label} to {}", &*account_name)?;
                        }
                        Err(e) => writeln!(self.writer, "post error: {e}")?,
                    }
                }
                Ok(Command::Balance { account_name }) => match self.ledger.balance(&account_name) {
                    Ok(balance) => {
                        let label = if balance == 1 { "dollar" } else { "dollars" };
                        writeln!(self.writer, "{} has {balance} {label}", &*account_name)?;
                    }
                    Err(e) => writeln!(self.writer, "balance error: {e}")?,
                },
                Ok(Command::Exit) => return Ok(()),
                Err(e) => writeln!(self.writer, "{e}")?,
            }
            self.writer.flush()?;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Input with no exit line: run returns Ok when the reader runs out.
    #[test]
    #[ignore = "stub, not written yet"]
    fn run_exits_on_eof() {}

    /// "exit" returns Ok, and lines after it are never processed.
    #[test]
    #[ignore = "stub, not written yet"]
    fn run_exits_on_exit_command() {}

    /// create, post, balance end to end; check the written output.
    #[test]
    #[ignore = "stub, not written yet"]
    fn run_create_post_balance() {}

    /// Output for posting 1, 2, 0 and -5. What should "dollar" vs "dollars" say for each?
    #[test]
    #[ignore = "stub, not written yet"]
    fn run_post_label_singular_plural() {}

    /// Balance of 1 vs 2: does the balance line pluralize the same way post does?
    #[test]
    #[ignore = "stub, not written yet"]
    fn run_balance_label() {}

    /// A bad line writes an error and the next good line still runs.
    #[test]
    #[ignore = "stub, not written yet"]
    fn run_parse_error_continues() {}

    /// Posting to a missing account writes an error and the loop keeps going.
    #[test]
    #[ignore = "stub, not written yet"]
    fn run_ledger_error_continues() {}

    /// "help" output (and the startup hint) shows up in the writer, not on stdout.
    #[test]
    #[ignore = "stub, not written yet"]
    fn run_help_goes_to_writer() {}
}
