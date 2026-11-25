use std::str::FromStr;

use crate::repl::command::error::{InvalidBalance, InvalidCommand, InvalidPost};

pub mod error;

#[derive(Debug, Clone)]
pub enum Command {
    Help,
    Post { account: String, amount: i32 },
    Balance { account: String },
    Exit,
}

impl Command {
    pub fn account(&self) -> Option<&str> {
        match self {
            Self::Post { account, .. } | Self::Balance { account } => Some(account),
            Self::Help | Self::Exit => None,
        }
    }

    pub fn amount(&self) -> Option<i32> {
        match self {
            Self::Post { amount, .. } => Some(*amount),
            Self::Help | Self::Balance { .. } | Self::Exit => None,
        }
    }

    pub fn help_example(&self) -> &'static str {
        match self {
            Self::Help => "help",
            Self::Post { .. } => "post <account> <amount>",
            Self::Balance { .. } => "balance <account>",
            Self::Exit => "exit",
        }
    }

    pub fn help_message(&self) -> &'static str {
        match self {
            Self::Help => "show this message",
            Self::Post { .. } => "add an amount to an account",
            Self::Balance { .. } => "show an account's balance",
            Self::Exit => "quit",
        }
    }
}

impl FromStr for Command {
    type Err = InvalidCommand;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let Some(command) = parts.next() else {
            return Err(InvalidCommand::Empty);
        };

        Ok(match command {
            "help" => Self::Help,
            "post" => {
                let Some(account) = parts.next().map(String::from) else {
                    return Err(InvalidPost::MissingArg.into());
                };
                let Some(amount) = parts.next() else {
                    return Err(InvalidPost::MissingArg.into());
                };
                let amount: i32 = match amount.parse() {
                    Ok(a) => a,
                    Err(e) => return Err(InvalidPost::ParseArg(e).into()),
                };
                Self::Post { amount, account }
            }
            "balance" => {
                let Some(account) = parts.next().map(String::from) else {
                    return Err(InvalidBalance::Account.into());
                };
                Self::Balance { account }
            }
            "exit" => Self::Exit,
            _ => return Err(InvalidCommand::Unrecognized),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_command_ok_help() {
        assert!(matches!("help".parse::<Command>().unwrap(), Command::Help));
    }

    #[test]
    fn parse_command_ok_post() {
        let command = "post foo 1".parse::<Command>().unwrap();
        assert!(matches!(command, Command::Post { .. }));
        assert_eq!(command.account(), Some("foo"));
        assert_eq!(command.amount(), Some(1));
    }

    #[test]
    fn parse_command_err_post_missing_arg() {
        assert!(matches!(
            "post".parse::<Command>(),
            Err(InvalidCommand::Post(InvalidPost::MissingArg))
        ));
        assert!(matches!(
            "post foo".parse::<Command>(),
            Err(InvalidCommand::Post(InvalidPost::MissingArg))
        ));
    }

    #[test]
    fn parse_command_err_post_parse_arg() {
        assert!(matches!(
            "post foo bar".parse::<Command>(),
            Err(InvalidCommand::Post(InvalidPost::ParseArg(_)))
        ));
    }

    #[test]
    fn parse_command_ok_balance() {
        let command = "balance foo".parse::<Command>().unwrap();
        assert!(matches!(command, Command::Balance { .. }));
        assert_eq!(command.account(), Some("foo"));
        assert_eq!(command.amount(), None);
    }

    #[test]
    fn parse_command_err_balance() {
        assert!(matches!(
            "balance".parse::<Command>(),
            Err(InvalidCommand::Balance(InvalidBalance::Account))
        ));
    }

    #[test]
    fn parse_command_err_empty() {
        assert!(matches!("".parse::<Command>(), Err(InvalidCommand::Empty)));
    }
}
