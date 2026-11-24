mod command;

use std::process::ExitCode;

use crate::ledger::Ledger;

fn help() {
    println!("post <account> <int>");
    println!("balance <account>");
}

#[derive(Debug)]
pub struct State {
    ledger: Ledger,
}

impl State {
    fn parse_command() {}
}

pub fn run() -> ExitCode {
    help();
    let mut ledger = Ledger::new();
    let mut input = String::new();

    loop {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_lowercase();

        let mut parts = input.split_whitespace();
        let Some(command) = parts.next() else {
            println!("no command given");
            continue;
        };

        match command {
            "help" => help(),

            "post" => {
                let Some(account) = parts.next() else {
                    println!("no account given");
                    continue;
                };
                let Some(amount) = parts.next() else {
                    println!("no amount given");
                    continue;
                };
                match amount.parse::<i32>() {
                    Ok(amount) => {
                        ledger.post(account, amount);
                        println!("posted {amount} dollars to {account}");
                    }
                    Err(e) => println!("failed to parse amount: {e}"),
                }
            }

            "balance" => {
                let Some(account) = parts.next() else {
                    println!("no account given");
                    continue;
                };
                println!("{account} has {} dollars", ledger.balance(account));
            }

            "exit" => {
                println!("thank you goodbye");
                return ExitCode::SUCCESS;
            }

            _ => {
                println!("invalid input");
            }
        }

        input.clear();
    }
}
