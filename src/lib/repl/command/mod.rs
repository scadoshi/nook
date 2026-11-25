use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

use crate::ledger::account_name::{AccountName, ParseAccountNameError};

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
    #[error(transparent)]
    ParseAccountName(#[from] ParseAccountNameError),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Help,
    Create {
        account_name: AccountName,
    },
    Post {
        account_name: AccountName,
        amount: i32,
    },
    Balance {
        account_name: AccountName,
    },
    Exit,
}

/// One line of the help text: how to type a command, and what it does.
#[derive(Debug, Clone, Copy)]
pub(crate) struct HelpEntry {
    pub(crate) example: &'static str,
    pub(crate) message: &'static str,
}

/// Every command's help line, in the order help prints them. Add a row when
/// you add a `Command` variant; nothing checks that the two stay in step.
pub(crate) const HELP: [HelpEntry; 5] = [
    HelpEntry {
        example: "help",
        message: "show this message",
    },
    HelpEntry {
        example: "create <account name>",
        message: "create a new account",
    },
    HelpEntry {
        example: "post <account name> <amount>",
        message: "add an amount to an account",
    },
    HelpEntry {
        example: "balance <account name>",
        message: "show an account's balance",
    },
    HelpEntry {
        example: "exit",
        message: "quit",
    },
];

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
                let account_name = parts
                    .next()
                    .ok_or_else(|| ParseCommandError::MissingArg("account".into()))
                    .map(AccountName::from_str)??;
                if parts.next().is_some() {
                    return Err(ParseCommandError::TooManyArguments);
                }
                Self::Create { account_name }
            }
            "post" => {
                let account_name = parts
                    .next()
                    .ok_or_else(|| ParseCommandError::MissingArg("account".into()))
                    .map(AccountName::from_str)??;
                let amount: i32 = parts
                    .next()
                    .ok_or_else(|| ParseCommandError::MissingArg("amount".into()))?
                    .parse()
                    .map_err(|e| ParseCommandError::ArgParseInt("amount", e))?;
                if parts.next().is_some() {
                    return Err(ParseCommandError::TooManyArguments);
                }
                Self::Post {
                    account_name,
                    amount,
                }
            }
            "balance" => {
                let account_name = parts
                    .next()
                    .ok_or_else(|| ParseCommandError::MissingArg("account".into()))
                    .map(AccountName::from_str)??;
                if parts.next().is_some() {
                    return Err(ParseCommandError::TooManyArguments);
                }
                Self::Balance { account_name }
            }
            "exit" => {
                if parts.next().is_some() {
                    return Err(ParseCommandError::TooManyArguments);
                }
                Self::Exit
            }
            x => return Err(ParseCommandError::Unrecognized(x.into())),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::account_name;

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
        let expected = Command::Create {
            account_name: account_name(),
        };
        let cases = ["create", "CREATE", "CrEaTe"]
            .into_iter()
            .map(|s| format!("{s} {}", &*account_name()));
        for str in cases {
            let command: Command = str.parse().unwrap();
            assert_eq!(command, expected, "input = {str:?}");
        }
    }

    #[test]
    fn parse_post() {
        let amount = 1;
        let expected = Command::Post {
            account_name: account_name(),
            amount,
        };
        let cases = ["post", "POST", "PoSt"]
            .into_iter()
            .map(|s| format!("{s} {} {amount}", &*account_name()));
        for str in cases {
            let command: Command = str.parse().unwrap();
            assert_eq!(command, expected, "input = {str:?}");
        }
    }

    #[test]
    fn parse_balance() {
        let expected = Command::Balance {
            account_name: account_name(),
        };
        let cases = ["balance", "BALANCE", "BaLaNcE"]
            .into_iter()
            .map(|s| format!("{s} {}", &*account_name()));
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

    #[test]
    fn empty_input() {
        let cases = vec!["", " ", "\n"];
        for str in cases {
            assert_eq!(
                str.parse::<Command>().unwrap_err(),
                ParseCommandError::Empty,
                "input = {str:?}"
            );
        }
    }

    #[test]
    fn arg_parse_int() {
        let cases = ["bar", "2147483648"]
            .iter()
            .map(|s| format!("post foo {s}"));
        for str in cases {
            assert!(
                matches!(
                    str.parse::<Command>().unwrap_err(),
                    ParseCommandError::ArgParseInt(..)
                ),
                "input = {str:?}"
            );
        }
    }

    #[test]
    fn parse_post_negative() {
        let amount = -1;
        assert_eq!(
            format!("post {} {}", &*account_name(), amount)
                .parse::<Command>()
                .unwrap(),
            Command::Post {
                account_name: account_name(),
                amount,
            }
        );
    }

    #[test]
    fn unrecognized() {
        assert!(matches!(
            "foo".parse::<Command>().unwrap_err(),
            ParseCommandError::Unrecognized(..)
        ));
    }
}
