use std::process::ExitCode;

use nook::repl::Repl;

fn main() -> ExitCode {
    if let Err(e) = Repl::new(std::io::stdin().lock()).run() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
