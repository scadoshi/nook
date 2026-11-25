use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq)]
pub enum ParseCommandError {
    #[error("input was empty")]
    Empty,
    #[error("unrecognized command: {0}")]
    Unrecognized(String),
    #[error("missing argument: <{0}>")]
    MissingArg(String),
    #[error("too many arguments")]
    TooManyArguments,
    #[error("couldn't parse argument: {0} into int: {1}")]
    ArgParseInt(&'static str, ParseIntError),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Help,
    Create { account: String },
    Post { account: String, amount: i32 },
    Balance { account: String },
    Exit,
}

impl Command {
    pub fn help_example(&self) -> &'static str {
        match self {
            Self::Help => "help",
            Self::Create { .. } => "create <account>",
            Self::Post { .. } => "post <account> <amount>",
            Self::Balance { .. } => "balance <account>",
            Self::Exit => "exit",
        }
    }

    pub fn help_message(&self) -> &'static str {
        match self {
            Self::Help => "show this message",
            Self::Create { .. } => "create a new account",
            Self::Post { .. } => "add an amount to an account",
            Self::Balance { .. } => "show an account's balance",
            Self::Exit => "quit",
        }
    }
}

impl FromStr for Command {
    type Err = ParseCommandError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let Some(command) = parts.next() else {
            return Err(ParseCommandError::Empty);
        };

        Ok(match command.to_lowercase().as_ref() {
            "help" => {
                if parts.next().is_some() {
                    return Err(ParseCommandError::TooManyArguments);
                }
                Self::Help
            }
            "create" => {
                let Some(account) = parts.next().map(String::from) else {
                    return Err(ParseCommandError::MissingArg("account".into()));
                };
                if parts.next().is_some() {
                    return Err(ParseCommandError::TooManyArguments);
                }
                Self::Create { account }
            }
            "post" => {
                let Some(account) = parts.next().map(String::from) else {
                    return Err(ParseCommandError::MissingArg("account".into()));
                };
                let Some(amount) = parts.next() else {
                    return Err(ParseCommandError::MissingArg("amount".into()));
                };
                if parts.next().is_some() {
                    return Err(ParseCommandError::TooManyArguments);
                }
                let amount: i32 = match amount.parse() {
                    Ok(a) => a,
                    Err(e) => return Err(ParseCommandError::ArgParseInt("amount", e)),
                };
                Self::Post { amount, account }
            }
            "balance" => {
                let Some(account) = parts.next().map(String::from) else {
                    return Err(ParseCommandError::MissingArg("account".into()));
                };
                Self::Balance { account }
            }
            "exit" => Self::Exit,
            x => return Err(ParseCommandError::Unrecognized(x.into())),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_help_command() {
        let cases = ["help", "HELP", "HeLp"];
        for str in cases {
            assert_eq!(
                str.parse::<Command>().unwrap(),
                Command::Help,
                "input = {str:?}"
            );
        }
    }

    #[test]
    fn parse_create() {
        let account = "foo";
        let expected = Command::Create {
            account: account.into(),
        };
        let cases = ["create", "CREATE", "CrEaTe"]
            .into_iter()
            .map(|s| format!("{s} {account}"));
        for str in cases {
            let command: Command = str.parse().unwrap();
            assert_eq!(command, expected, "input = {str:?}");
        }
    }

    #[test]
    fn parse_post() {
        let account = "foo";
        let amount = 1;
        let expected = Command::Post {
            account: account.into(),
            amount,
        };
        let cases = ["post", "POST", "PoSt"]
            .into_iter()
            .map(|s| format!("{s} {account} {amount}"));
        for str in cases {
            let command: Command = str.parse().unwrap();
            assert_eq!(command, expected, "input = {str:?}");
        }
    }

    #[test]
    fn parse_balance() {
        let account = "foo";
        let expected = Command::Balance {
            account: account.into(),
        };
        let cases = ["balance", "BALANCE", "BaLaNcE"]
            .into_iter()
            .map(|s| format!("{s} {account}"));
        for str in cases {
            let command: Command = str.parse().unwrap();
            assert_eq!(command, expected, "input = {str:?}");
        }
    }

    #[test]
    fn parse_exit() {
        let cases = ["exit", "EXIT", "ExIt"];
        for str in cases {
            assert_eq!(
                str.parse::<Command>().unwrap(),
                Command::Exit,
                "input = {str:?}"
            );
        }
    }

    #[test]
    fn missing_arguments() {
        let cases = vec![
            ("account", vec!["create", "post", "balance"]),
            ("amount", vec!["post foo"]),
        ];
        for (expected, cases) in cases {
            for str in cases {
                assert_eq!(
                    str.parse::<Command>().unwrap_err(),
                    ParseCommandError::MissingArg(expected.into()),
                    "input = {str:?}"
                );
            }
        }
    }

    #[test]
    fn too_many_arguments() {
        let cases = vec![
            "help foo",
            "create foo bar",
            "post foo 1 bar",
            "balance foo bar",
            "exit foo",
        ];
        for str in cases {
            assert_eq!(
                str.parse::<Command>().unwrap_err(),
                ParseCommandError::TooManyArguments,
                "input = {str:?}"
            );
        }
    }

    /// Table: "", "   ", "\n" all return Empty.
    #[test]
    #[ignore = "stub, not written yet"]
    fn empty_input() {}

    /// Table: "post foo bar" and an amount past i32::MAX both return ArgParseInt on "amount".
    #[test]
    #[ignore = "stub, not written yet"]
    fn arg_parse_int() {}

    /// "post foo -5" parses with amount -5.
    #[test]
    #[ignore = "stub, not written yet"]
    fn parse_post_negative() {}

    /// "post Bob 1" keeps the account as "Bob", not "bob".
    #[test]
    #[ignore = "stub, not written yet"]
    fn preserves_account_case() {}

    /// An unknown word returns Unrecognized.
    #[test]
    #[ignore = "stub, not written yet"]
    fn unrecognized() {}
}
